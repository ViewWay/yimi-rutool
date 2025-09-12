//! SQL query builder utilities
//!
//! This module provides a fluent interface for building SQL queries
//! in a database-agnostic way.

use crate::error::{Error, Result};
use std::collections::HashMap;

/// SQL query builder
#[derive(Debug, Clone)]
pub struct QueryBuilder {
    query_type: QueryType,
    table: Option<String>,
    columns: Vec<String>,
    values: Vec<QueryValue>,
    conditions: Vec<Condition>,
    joins: Vec<Join>,
    group_by: Vec<String>,
    having: Vec<Condition>,
    order_by: Vec<OrderBy>,
    limit: Option<usize>,
    offset: Option<usize>,
}

#[derive(Debug, Clone)]
enum QueryType {
    Select,
    Insert,
    Update,
    Delete,
}

#[derive(Debug, Clone)]
/// Query value types for SQL operations
pub enum QueryValue {
    /// String value
    String(String),
    /// Integer value  
    Integer(i64),
    /// Float value
    Float(f64),
    /// Boolean value
    Boolean(bool),
    /// Null value
    Null,
}

#[derive(Debug, Clone)]
/// SQL condition for WHERE clauses
pub struct Condition {
    column: String,
    operator: String,
    value: QueryValue,
    connector: String, // AND, OR
}

#[derive(Debug, Clone)]
/// SQL JOIN clause configuration
pub struct Join {
    join_type: String, // INNER, LEFT, RIGHT, FULL
    table: String,
    on_condition: String,
}

#[derive(Debug, Clone)]
/// SQL ORDER BY clause configuration
pub struct OrderBy {
    column: String,
    direction: String, // ASC, DESC
}

impl QueryBuilder {
    /// Create a new SELECT query builder
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::db::QueryBuilder;
    ///
    /// let query = QueryBuilder::select()
    ///     .columns(&["id", "name"])
    ///     .from("users")
    ///     .where_eq("active", true)
    ///     .build();
    /// ```
    pub fn select() -> Self {
        Self {
            query_type: QueryType::Select,
            table: None,
            columns: Vec::new(),
            values: Vec::new(),
            conditions: Vec::new(),
            joins: Vec::new(),
            group_by: Vec::new(),
            having: Vec::new(),
            order_by: Vec::new(),
            limit: None,
            offset: None,
        }
    }

    /// Create a new INSERT query builder
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::db::QueryBuilder;
    ///
    /// let query = QueryBuilder::insert()
    ///     .into("users")
    ///     .columns(&["name", "email"])
    ///     .values(&["Alice", "alice@example.com"])
    ///     .build();
    /// ```
    pub fn insert() -> Self {
        Self {
            query_type: QueryType::Insert,
            table: None,
            columns: Vec::new(),
            values: Vec::new(),
            conditions: Vec::new(),
            joins: Vec::new(),
            group_by: Vec::new(),
            having: Vec::new(),
            order_by: Vec::new(),
            limit: None,
            offset: None,
        }
    }

    /// Create a new UPDATE query builder
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::db::QueryBuilder;
    ///
    /// let query = QueryBuilder::update()
    ///     .table("users")
    ///     .set("name", "Bob")
    ///     .where_eq("id", 1)
    ///     .build();
    /// ```
    pub fn update() -> Self {
        Self {
            query_type: QueryType::Update,
            table: None,
            columns: Vec::new(),
            values: Vec::new(),
            conditions: Vec::new(),
            joins: Vec::new(),
            group_by: Vec::new(),
            having: Vec::new(),
            order_by: Vec::new(),
            limit: None,
            offset: None,
        }
    }

    /// Create a new DELETE query builder
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::db::QueryBuilder;
    ///
    /// let query = QueryBuilder::delete()
    ///     .from("users")
    ///     .where_eq("active", false)
    ///     .build();
    /// ```
    pub fn delete() -> Self {
        Self {
            query_type: QueryType::Delete,
            table: None,
            columns: Vec::new(),
            values: Vec::new(),
            conditions: Vec::new(),
            joins: Vec::new(),
            group_by: Vec::new(),
            having: Vec::new(),
            order_by: Vec::new(),
            limit: None,
            offset: None,
        }
    }

    /// Set the table name (for SELECT, UPDATE, DELETE)
    pub fn from(mut self, table: &str) -> Self {
        self.table = Some(table.to_string());
        self
    }

    /// Set the table name (for INSERT, UPDATE)
    pub fn into(mut self, table: &str) -> Self {
        self.table = Some(table.to_string());
        self
    }

    /// Set the table name (for UPDATE)
    pub fn table(mut self, table: &str) -> Self {
        self.table = Some(table.to_string());
        self
    }

    /// Set the columns to select
    pub fn columns(mut self, columns: &[&str]) -> Self {
        self.columns = columns.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Add a single column to select
    pub fn column(mut self, column: &str) -> Self {
        self.columns.push(column.to_string());
        self
    }

    /// Set values for INSERT
    pub fn values(mut self, values: &[&str]) -> Self {
        for value in values {
            self.values.push(QueryValue::String(value.to_string()));
        }
        self
    }

    /// Set integer values for INSERT
    pub fn int_values(mut self, values: &[i64]) -> Self {
        for &value in values {
            self.values.push(QueryValue::Integer(value));
        }
        self
    }

    /// Set a single value for UPDATE
    pub fn set(mut self, column: &str, value: &str) -> Self {
        self.columns.push(column.to_string());
        self.values.push(QueryValue::String(value.to_string()));
        self
    }

    /// Set an integer value for UPDATE
    pub fn set_int(mut self, column: &str, value: i64) -> Self {
        self.columns.push(column.to_string());
        self.values.push(QueryValue::Integer(value));
        self
    }

    /// Set a float value for UPDATE
    pub fn set_float(mut self, column: &str, value: f64) -> Self {
        self.columns.push(column.to_string());
        self.values.push(QueryValue::Float(value));
        self
    }

    /// Set a boolean value for UPDATE
    pub fn set_bool(mut self, column: &str, value: bool) -> Self {
        self.columns.push(column.to_string());
        self.values.push(QueryValue::Boolean(value));
        self
    }

    /// Add a WHERE condition with equality
    pub fn where_eq(mut self, column: &str, value: impl Into<QueryValue>) -> Self {
        self.conditions.push(Condition {
            column: column.to_string(),
            operator: "=".to_string(),
            value: value.into(),
            connector: "AND".to_string(),
        });
        self
    }

    /// Add a WHERE condition with inequality
    pub fn where_ne(mut self, column: &str, value: impl Into<QueryValue>) -> Self {
        self.conditions.push(Condition {
            column: column.to_string(),
            operator: "!=".to_string(),
            value: value.into(),
            connector: "AND".to_string(),
        });
        self
    }

    /// Add a WHERE condition with greater than
    pub fn where_gt(mut self, column: &str, value: impl Into<QueryValue>) -> Self {
        self.conditions.push(Condition {
            column: column.to_string(),
            operator: ">".to_string(),
            value: value.into(),
            connector: "AND".to_string(),
        });
        self
    }

    /// Add a WHERE condition with less than
    pub fn where_lt(mut self, column: &str, value: impl Into<QueryValue>) -> Self {
        self.conditions.push(Condition {
            column: column.to_string(),
            operator: "<".to_string(),
            value: value.into(),
            connector: "AND".to_string(),
        });
        self
    }

    /// Add a WHERE condition with LIKE
    pub fn where_like(mut self, column: &str, pattern: &str) -> Self {
        self.conditions.push(Condition {
            column: column.to_string(),
            operator: "LIKE".to_string(),
            value: QueryValue::String(pattern.to_string()),
            connector: "AND".to_string(),
        });
        self
    }

    /// Add a WHERE condition with IN
    pub fn where_in(mut self, column: &str, values: &[&str]) -> Self {
        let values_str = values.iter()
            .map(|v| format!("'{}'", v.replace('\'', "''")))
            .collect::<Vec<_>>()
            .join(", ");
        
        self.conditions.push(Condition {
            column: column.to_string(),
            operator: "IN".to_string(),
            value: QueryValue::String(format!("({})", values_str)),
            connector: "AND".to_string(),
        });
        self
    }

    /// Add an OR WHERE condition with equality
    pub fn or_where_eq(mut self, column: &str, value: impl Into<QueryValue>) -> Self {
        self.conditions.push(Condition {
            column: column.to_string(),
            operator: "=".to_string(),
            value: value.into(),
            connector: "OR".to_string(),
        });
        self
    }

    /// Add an INNER JOIN
    pub fn inner_join(mut self, table: &str, on: &str) -> Self {
        self.joins.push(Join {
            join_type: "INNER".to_string(),
            table: table.to_string(),
            on_condition: on.to_string(),
        });
        self
    }

    /// Add a LEFT JOIN
    pub fn left_join(mut self, table: &str, on: &str) -> Self {
        self.joins.push(Join {
            join_type: "LEFT".to_string(),
            table: table.to_string(),
            on_condition: on.to_string(),
        });
        self
    }

    /// Add a RIGHT JOIN
    pub fn right_join(mut self, table: &str, on: &str) -> Self {
        self.joins.push(Join {
            join_type: "RIGHT".to_string(),
            table: table.to_string(),
            on_condition: on.to_string(),
        });
        self
    }

    /// Add GROUP BY clause
    pub fn group_by(mut self, columns: &[&str]) -> Self {
        self.group_by = columns.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Add HAVING clause
    pub fn having_eq(mut self, column: &str, value: impl Into<QueryValue>) -> Self {
        self.having.push(Condition {
            column: column.to_string(),
            operator: "=".to_string(),
            value: value.into(),
            connector: "AND".to_string(),
        });
        self
    }

    /// Add ORDER BY clause (ascending)
    pub fn order_by_asc(mut self, column: &str) -> Self {
        self.order_by.push(OrderBy {
            column: column.to_string(),
            direction: "ASC".to_string(),
        });
        self
    }

    /// Add ORDER BY clause (descending)
    pub fn order_by_desc(mut self, column: &str) -> Self {
        self.order_by.push(OrderBy {
            column: column.to_string(),
            direction: "DESC".to_string(),
        });
        self
    }

    /// Set LIMIT clause
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set OFFSET clause
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Build the SQL query string
    pub fn build(self) -> Result<String> {
        match self.query_type {
            QueryType::Select => self.build_select(),
            QueryType::Insert => self.build_insert(),
            QueryType::Update => self.build_update(),
            QueryType::Delete => self.build_delete(),
        }
    }

    fn build_select(&self) -> Result<String> {
        let mut query = String::new();
        
        // SELECT clause
        query.push_str("SELECT ");
        if self.columns.is_empty() {
            query.push('*');
        } else {
            query.push_str(&self.columns.join(", "));
        }
        
        // FROM clause
        if let Some(table) = &self.table {
            query.push_str(&format!(" FROM {}", table));
        } else {
            return Err(Error::validation("Table name is required for SELECT query".to_string()));
        }
        
        // JOIN clauses
        for join in &self.joins {
            query.push_str(&format!(" {} JOIN {} ON {}", 
                join.join_type, join.table, join.on_condition));
        }
        
        // WHERE clause
        if !self.conditions.is_empty() {
            query.push_str(" WHERE ");
            for (i, condition) in self.conditions.iter().enumerate() {
                if i > 0 {
                    query.push_str(&format!(" {} ", condition.connector));
                }
                query.push_str(&format!("{} {} {}", 
                    condition.column, condition.operator, self.format_value(&condition.value)));
            }
        }
        
        // GROUP BY clause
        if !self.group_by.is_empty() {
            query.push_str(&format!(" GROUP BY {}", self.group_by.join(", ")));
        }
        
        // HAVING clause
        if !self.having.is_empty() {
            query.push_str(" HAVING ");
            for (i, condition) in self.having.iter().enumerate() {
                if i > 0 {
                    query.push_str(&format!(" {} ", condition.connector));
                }
                query.push_str(&format!("{} {} {}", 
                    condition.column, condition.operator, self.format_value(&condition.value)));
            }
        }
        
        // ORDER BY clause
        if !self.order_by.is_empty() {
            query.push_str(" ORDER BY ");
            let order_parts: Vec<String> = self.order_by.iter()
                .map(|order| format!("{} {}", order.column, order.direction))
                .collect();
            query.push_str(&order_parts.join(", "));
        }
        
        // LIMIT clause
        if let Some(limit) = self.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }
        
        // OFFSET clause
        if let Some(offset) = self.offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }
        
        Ok(query)
    }

    fn build_insert(&self) -> Result<String> {
        let table = self.table.as_ref().ok_or_else(|| 
            Error::validation("Table name is required for INSERT query".to_string()))?;
        
        if self.columns.is_empty() {
            return Err(Error::validation("Columns are required for INSERT query".to_string()));
        }
        
        if self.values.len() != self.columns.len() {
            return Err(Error::validation("Number of values must match number of columns".to_string()));
        }
        
        let mut query = format!("INSERT INTO {} ({})", table, self.columns.join(", "));
        
        let values_str: Vec<String> = self.values.iter()
            .map(|value| self.format_value(value))
            .collect();
        
        query.push_str(&format!(" VALUES ({})", values_str.join(", ")));
        
        Ok(query)
    }

    fn build_update(&self) -> Result<String> {
        let table = self.table.as_ref().ok_or_else(|| 
            Error::validation("Table name is required for UPDATE query".to_string()))?;
        
        if self.columns.is_empty() {
            return Err(Error::validation("SET clause is required for UPDATE query".to_string()));
        }
        
        if self.values.len() != self.columns.len() {
            return Err(Error::validation("Number of values must match number of columns".to_string()));
        }
        
        let mut query = format!("UPDATE {}", table);
        
        // SET clause
        query.push_str(" SET ");
        let set_parts: Vec<String> = self.columns.iter().zip(self.values.iter())
            .map(|(col, val)| format!("{} = {}", col, self.format_value(val)))
            .collect();
        query.push_str(&set_parts.join(", "));
        
        // WHERE clause
        if !self.conditions.is_empty() {
            query.push_str(" WHERE ");
            for (i, condition) in self.conditions.iter().enumerate() {
                if i > 0 {
                    query.push_str(&format!(" {} ", condition.connector));
                }
                query.push_str(&format!("{} {} {}", 
                    condition.column, condition.operator, self.format_value(&condition.value)));
            }
        }
        
        Ok(query)
    }

    fn build_delete(&self) -> Result<String> {
        let table = self.table.as_ref().ok_or_else(|| 
            Error::validation("Table name is required for DELETE query".to_string()))?;
        
        let mut query = format!("DELETE FROM {}", table);
        
        // WHERE clause
        if !self.conditions.is_empty() {
            query.push_str(" WHERE ");
            for (i, condition) in self.conditions.iter().enumerate() {
                if i > 0 {
                    query.push_str(&format!(" {} ", condition.connector));
                }
                query.push_str(&format!("{} {} {}", 
                    condition.column, condition.operator, self.format_value(&condition.value)));
            }
        }
        
        Ok(query)
    }

    fn format_value(&self, value: &QueryValue) -> String {
        match value {
            QueryValue::String(s) => {
                if s.starts_with('(') && s.ends_with(')') {
                    // For IN clauses, don't add extra quotes
                    s.clone()
                } else {
                    format!("'{}'", s.replace('\'', "''"))
                }
            }
            QueryValue::Integer(i) => i.to_string(),
            QueryValue::Float(f) => f.to_string(),
            QueryValue::Boolean(b) => if *b { "TRUE" } else { "FALSE" }.to_string(),
            QueryValue::Null => "NULL".to_string(),
        }
    }
}

// Implement Into<QueryValue> for common types
impl From<&str> for QueryValue {
    fn from(s: &str) -> Self {
        QueryValue::String(s.to_string())
    }
}

impl From<String> for QueryValue {
    fn from(s: String) -> Self {
        QueryValue::String(s)
    }
}

impl From<i32> for QueryValue {
    fn from(i: i32) -> Self {
        QueryValue::Integer(i as i64)
    }
}

impl From<i64> for QueryValue {
    fn from(i: i64) -> Self {
        QueryValue::Integer(i)
    }
}

impl From<f32> for QueryValue {
    fn from(f: f32) -> Self {
        QueryValue::Float(f as f64)
    }
}

impl From<f64> for QueryValue {
    fn from(f: f64) -> Self {
        QueryValue::Float(f)
    }
}

impl From<bool> for QueryValue {
    fn from(b: bool) -> Self {
        QueryValue::Boolean(b)
    }
}

/// Query execution helper
pub struct QueryExecutor;

impl QueryExecutor {
    /// Execute a query and return a formatted result
    pub fn format_query_result(
        rows: Vec<HashMap<String, serde_json::Value>>
    ) -> Result<String> {
        if rows.is_empty() {
            return Ok("No results found.".to_string());
        }

        let mut result = String::new();
        
        // Get column names from first row
        let columns: Vec<String> = rows[0].keys().cloned().collect();
        
        // Calculate column widths
        let mut widths: HashMap<String, usize> = HashMap::new();
        for col in &columns {
            widths.insert(col.clone(), col.len());
        }
        
        for row in &rows {
            for (col, value) in row {
                let value_str = match value {
                    serde_json::Value::String(s) => s.clone(),
                    _ => value.to_string(),
                };
                let current_width = widths.get(col).unwrap_or(&0);
                if value_str.len() > *current_width {
                    widths.insert(col.clone(), value_str.len());
                }
            }
        }
        
        // Create header
        result.push('|');
        for col in &columns {
            let width = widths.get(col).unwrap_or(&0);
            result.push_str(&format!(" {:width$} |", col, width = width));
        }
        result.push('\n');
        
        // Create separator
        result.push('|');
        for col in &columns {
            let width = widths.get(col).unwrap_or(&0);
            result.push_str(&format!("{:-<width$}|", "", width = width + 2));
        }
        result.push('\n');
        
        // Create data rows
        for row in &rows {
            result.push('|');
            for col in &columns {
                let width = widths.get(col).unwrap_or(&0);
                let value = row.get(col).unwrap_or(&serde_json::Value::Null);
                let value_str = match value {
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Null => "NULL".to_string(),
                    _ => value.to_string(),
                };
                result.push_str(&format!(" {:width$} |", value_str, width = width));
            }
            result.push('\n');
        }
        
        Ok(result)
    }

    /// Convert query result to CSV format
    pub fn to_csv(rows: Vec<HashMap<String, serde_json::Value>>) -> Result<String> {
        if rows.is_empty() {
            return Ok(String::new());
        }

        let mut result = String::new();
        
        // Get column names from first row
        let columns: Vec<String> = rows[0].keys().cloned().collect();
        
        // Write header
        result.push_str(&columns.join(","));
        result.push('\n');
        
        // Write data rows
        for row in &rows {
            let values: Vec<String> = columns.iter().map(|col| {
                let value = row.get(col).unwrap_or(&serde_json::Value::Null);
                match value {
                    serde_json::Value::String(s) => {
                        if s.contains(',') || s.contains('"') || s.contains('\n') {
                            format!("\"{}\"", s.replace('"', "\"\""))
                        } else {
                            s.clone()
                        }
                    }
                    serde_json::Value::Null => String::new(),
                    _ => value.to_string(),
                }
            }).collect();
            
            result.push_str(&values.join(","));
            result.push('\n');
        }
        
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_query_builder() {
        let query = QueryBuilder::select()
            .columns(&["id", "name", "email"])
            .from("users")
            .where_eq("active", true)
            .where_gt("age", 18)
            .order_by_asc("name")
            .limit(10)
            .build()
            .unwrap();

        let expected = "SELECT id, name, email FROM users WHERE active = TRUE AND age > 18 ORDER BY name ASC LIMIT 10";
        assert_eq!(query, expected);
    }

    #[test]
    fn test_insert_query_builder() {
        let query = QueryBuilder::insert()
            .into("users")
            .columns(&["name", "email"])
            .values(&["Alice", "alice@example.com"])
            .build()
            .unwrap();

        let expected = "INSERT INTO users (name, email) VALUES ('Alice', 'alice@example.com')";
        assert_eq!(query, expected);
    }

    #[test]
    fn test_update_query_builder() {
        let query = QueryBuilder::update()
            .table("users")
            .set("name", "Bob")
            .set_int("age", 25)
            .where_eq("id", 1)
            .build()
            .unwrap();

        let expected = "UPDATE users SET name = 'Bob', age = 25 WHERE id = 1";
        assert_eq!(query, expected);
    }

    #[test]
    fn test_delete_query_builder() {
        let query = QueryBuilder::delete()
            .from("users")
            .where_eq("active", false)
            .build()
            .unwrap();

        let expected = "DELETE FROM users WHERE active = FALSE";
        assert_eq!(query, expected);
    }

    #[test]
    fn test_complex_select_with_joins() {
        let query = QueryBuilder::select()
            .columns(&["u.name", "p.title"])
            .from("users u")
            .inner_join("posts p", "u.id = p.user_id")
            .where_like("u.name", "%John%")
            .order_by_desc("p.created_at")
            .build()
            .unwrap();

        let expected = "SELECT u.name, p.title FROM users u INNER JOIN posts p ON u.id = p.user_id WHERE u.name LIKE '%John%' ORDER BY p.created_at DESC";
        assert_eq!(query, expected);
    }

    #[test]
    fn test_where_in_clause() {
        let query = QueryBuilder::select()
            .from("users")
            .where_in("status", &["active", "pending"])
            .build()
            .unwrap();

        let expected = "SELECT * FROM users WHERE status IN ('active', 'pending')";
        assert_eq!(query, expected);
    }

    #[test]
    fn test_group_by_having() {
        let query = QueryBuilder::select()
            .columns(&["department", "COUNT(*)"])
            .from("employees")
            .group_by(&["department"])
            .having_eq("COUNT(*)", "> 5")
            .build()
            .unwrap();

        let expected = "SELECT department, COUNT(*) FROM employees GROUP BY department HAVING COUNT(*) = '> 5'";
        assert_eq!(query, expected);
    }

    #[test]
    fn test_query_value_conversions() {
        let str_val: QueryValue = "test".into();
        let int_val: QueryValue = 42i32.into();
        let int64_val: QueryValue = 42i64.into();
        let float_val: QueryValue = 3.14f32.into();
        let float64_val: QueryValue = 3.14f64.into();
        let bool_val: QueryValue = true.into();

        assert!(matches!(str_val, QueryValue::String(_)));
        assert!(matches!(int_val, QueryValue::Integer(42)));
        assert!(matches!(int64_val, QueryValue::Integer(42)));
        assert!(matches!(float_val, QueryValue::Float(_)));
        assert!(matches!(float64_val, QueryValue::Float(_)));
        assert!(matches!(bool_val, QueryValue::Boolean(true)));
    }

    #[test]
    fn test_query_executor_format() {
        let mut rows = Vec::new();
        let mut row1 = HashMap::new();
        row1.insert("id".to_string(), serde_json::Value::Number(serde_json::Number::from(1)));
        row1.insert("name".to_string(), serde_json::Value::String("Alice".to_string()));
        rows.push(row1);

        let mut row2 = HashMap::new();
        row2.insert("id".to_string(), serde_json::Value::Number(serde_json::Number::from(2)));
        row2.insert("name".to_string(), serde_json::Value::String("Bob".to_string()));
        rows.push(row2);

        let result = QueryExecutor::format_query_result(rows).unwrap();
        assert!(result.contains("Alice"));
        assert!(result.contains("Bob"));
        assert!(result.contains("|"));
    }

    #[test]
    fn test_query_executor_csv() {
        let mut rows = Vec::new();
        let mut row1 = HashMap::new();
        row1.insert("id".to_string(), serde_json::Value::Number(serde_json::Number::from(1)));
        row1.insert("name".to_string(), serde_json::Value::String("Alice".to_string()));
        rows.push(row1);

        let csv = QueryExecutor::to_csv(rows).unwrap();
        assert!(csv.contains("id"));
        assert!(csv.contains("name"));
        assert!(csv.contains("1"));
        assert!(csv.contains("Alice"));
    }

    #[test]
    fn test_error_cases() {
        // Missing table name
        let result = QueryBuilder::select().build();
        assert!(result.is_err());

        // Missing columns for insert
        let result = QueryBuilder::insert().into("users").build();
        assert!(result.is_err());

        // Mismatched columns and values
        let result = QueryBuilder::insert()
            .into("users")
            .columns(&["name"])
            .values(&["Alice", "Bob"])
            .build();
        assert!(result.is_err());
    }
}
