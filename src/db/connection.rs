//! Database connection management
//!
//! This module provides utilities for managing database connections,
//! connection pooling, and database-specific operations.

use crate::error::{Error, Result};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

#[cfg(feature = "db")]
use sqlx::{Pool, Sqlite, Postgres, MySql, Row, Column};

/// Database type enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatabaseType {
    /// SQLite database
    SQLite,
    /// PostgreSQL database
    PostgreSQL,
    /// MySQL database
    MySQL,
}

/// Database connection configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    /// Database type
    pub db_type: DatabaseType,
    /// Connection URL or path
    pub url: String,
    /// Maximum number of connections in pool
    pub max_connections: u32,
    /// Minimum number of connections in pool
    pub min_connections: u32,
    /// Connection timeout
    pub connect_timeout: Duration,
    /// Idle timeout for connections
    pub idle_timeout: Option<Duration>,
    /// Maximum lifetime of connections
    pub max_lifetime: Option<Duration>,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            db_type: DatabaseType::SQLite,
            url: ":memory:".to_string(),
            max_connections: 10,
            min_connections: 1,
            connect_timeout: Duration::from_secs(30),
            idle_timeout: Some(Duration::from_secs(600)),
            max_lifetime: Some(Duration::from_secs(1800)),
        }
    }
}

impl DatabaseConfig {
    /// Create a new database configuration
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::db::{DatabaseConfig, DatabaseType};
    ///
    /// let config = DatabaseConfig::new(
    ///     DatabaseType::SQLite,
    ///     "database.db"
    /// );
    /// ```
    pub fn new(db_type: DatabaseType, url: &str) -> Self {
        Self {
            db_type,
            url: url.to_string(),
            ..Default::default()
        }
    }

    /// Set maximum connections in pool
    pub fn with_max_connections(mut self, max: u32) -> Self {
        self.max_connections = max;
        self
    }

    /// Set minimum connections in pool
    pub fn with_min_connections(mut self, min: u32) -> Self {
        self.min_connections = min;
        self
    }

    /// Set connection timeout
    pub fn with_connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    /// Set idle timeout
    pub fn with_idle_timeout(mut self, timeout: Duration) -> Self {
        self.idle_timeout = Some(timeout);
        self
    }

    /// Set maximum connection lifetime
    pub fn with_max_lifetime(mut self, lifetime: Duration) -> Self {
        self.max_lifetime = Some(lifetime);
        self
    }
}

/// Generic database connection wrapper
#[derive(Debug)]
pub enum DatabaseConnection {
    #[cfg(feature = "db")]
    /// SQLite connection pool
    SQLite(Pool<Sqlite>),
    #[cfg(feature = "db")]
    /// PostgreSQL connection pool
    PostgreSQL(Pool<Postgres>),
    #[cfg(feature = "db")]
    /// MySQL connection pool
    MySQL(Pool<MySql>),
    /// Mock connection for testing
    Mock,
}

impl DatabaseConnection {
    /// Create a new database connection
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::db::{DatabaseConnection, DatabaseConfig, DatabaseType};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let config = DatabaseConfig::new(DatabaseType::SQLite, ":memory:");
    ///     let conn = DatabaseConnection::new(config).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn new(config: DatabaseConfig) -> Result<Self> {
        #[cfg(feature = "db")]
        {
            use sqlx::sqlite::SqlitePoolOptions;
            use sqlx::postgres::PgPoolOptions;
            use sqlx::mysql::MySqlPoolOptions;

            match config.db_type {
                DatabaseType::SQLite => {
                    let pool = SqlitePoolOptions::new()
                        .max_connections(config.max_connections)
                        .min_connections(config.min_connections)
                        .acquire_timeout(config.connect_timeout)
                        .idle_timeout(config.idle_timeout)
                        .max_lifetime(config.max_lifetime)
                        .connect(&config.url)
                        .await
                        .map_err(|e| Error::database(format!("Failed to connect to SQLite: {}", e)))?;
                    Ok(DatabaseConnection::SQLite(pool))
                }
                DatabaseType::PostgreSQL => {
                    let pool = PgPoolOptions::new()
                        .max_connections(config.max_connections)
                        .min_connections(config.min_connections)
                        .acquire_timeout(config.connect_timeout)
                        .idle_timeout(config.idle_timeout)
                        .max_lifetime(config.max_lifetime)
                        .connect(&config.url)
                        .await
                        .map_err(|e| Error::database(format!("Failed to connect to PostgreSQL: {}", e)))?;
                    Ok(DatabaseConnection::PostgreSQL(pool))
                }
                DatabaseType::MySQL => {
                    let pool = MySqlPoolOptions::new()
                        .max_connections(config.max_connections)
                        .min_connections(config.min_connections)
                        .acquire_timeout(config.connect_timeout)
                        .idle_timeout(config.idle_timeout)
                        .max_lifetime(config.max_lifetime)
                        .connect(&config.url)
                        .await
                        .map_err(|e| Error::database(format!("Failed to connect to MySQL: {}", e)))?;
                    Ok(DatabaseConnection::MySQL(pool))
                }
            }
        }

        #[cfg(not(feature = "db"))]
        {
            let _ = config; // Avoid unused variable warning
            Ok(DatabaseConnection::Mock)
        }
    }

    /// Execute a SQL query and return the number of affected rows
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::db::{DatabaseConnection, DatabaseConfig, DatabaseType};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let config = DatabaseConfig::new(DatabaseType::SQLite, ":memory:");
    ///     let conn = DatabaseConnection::new(config).await?;
    ///     
    ///     let affected = conn.execute(
    ///         "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)"
    ///     ).await?;
    ///     
    ///     println!("Affected rows: {}", affected);
    ///     Ok(())
    /// }
    /// ```
    pub async fn execute(&self, sql: &str) -> Result<u64> {
        #[cfg(feature = "db")]
        {

            match self {
                DatabaseConnection::SQLite(pool) => {
                    let result = sqlx::query(sql)
                        .execute(pool)
                        .await
                        .map_err(|e| Error::database(format!("SQL execution failed: {}", e)))?;
                    Ok(result.rows_affected())
                }
                DatabaseConnection::PostgreSQL(pool) => {
                    let result = sqlx::query(sql)
                        .execute(pool)
                        .await
                        .map_err(|e| Error::database(format!("SQL execution failed: {}", e)))?;
                    Ok(result.rows_affected())
                }
                DatabaseConnection::MySQL(pool) => {
                    let result = sqlx::query(sql)
                        .execute(pool)
                        .await
                        .map_err(|e| Error::database(format!("SQL execution failed: {}", e)))?;
                    Ok(result.rows_affected())
                }
                DatabaseConnection::Mock => Ok(0),
            }
        }

        #[cfg(not(feature = "db"))]
        {
            let _ = sql; // Avoid unused variable warning
            Ok(0)
        }
    }

    /// Execute a SQL query with parameters and return the number of affected rows
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::db::{DatabaseConnection, DatabaseConfig, DatabaseType};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let config = DatabaseConfig::new(DatabaseType::SQLite, ":memory:");
    ///     let conn = DatabaseConnection::new(config).await?;
    ///     
    ///     // First create the table
    ///     conn.execute("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)").await?;
    ///     
    ///     // Then insert with parameters
    ///     let affected = conn.execute_with_params(
    ///         "INSERT INTO users (name) VALUES (?)",
    ///         &[&"Alice"]
    ///     ).await?;
    ///     
    ///     println!("Affected rows: {}", affected);
    ///     Ok(())
    /// }
    /// ```
    pub async fn execute_with_params(&self, sql: &str, params: &[&str]) -> Result<u64> {
        #[cfg(feature = "db")]
        {
            match self {
                DatabaseConnection::SQLite(pool) => {
                    let mut query = sqlx::query(sql);
                    for &param in params {
                        query = query.bind(param);
                    }
                    let result = query
                        .execute(pool)
                        .await
                        .map_err(|e| Error::database(format!("SQL execution failed: {}", e)))?;
                    Ok(result.rows_affected())
                }
                DatabaseConnection::PostgreSQL(pool) => {
                    let mut query = sqlx::query(sql);
                    for &param in params {
                        query = query.bind(param);
                    }
                    let result = query
                        .execute(pool)
                        .await
                        .map_err(|e| Error::database(format!("SQL execution failed: {}", e)))?;
                    Ok(result.rows_affected())
                }
                DatabaseConnection::MySQL(pool) => {
                    let mut query = sqlx::query(sql);
                    for &param in params {
                        query = query.bind(param);
                    }
                    let result = query
                        .execute(pool)
                        .await
                        .map_err(|e| Error::database(format!("SQL execution failed: {}", e)))?;
                    Ok(result.rows_affected())
                }
                DatabaseConnection::Mock => Ok(0),
            }
        }

        #[cfg(not(feature = "db"))]
        {
            let _ = (sql, params); // Avoid unused variable warnings
            Ok(0)
        }
    }

    /// Fetch all rows from a SQL query
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::db::{DatabaseConnection, DatabaseConfig, DatabaseType};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let config = DatabaseConfig::new(DatabaseType::SQLite, ":memory:");
    ///     let conn = DatabaseConnection::new(config).await?;
    ///     
    ///     let rows = conn.fetch_all("SELECT name FROM sqlite_master WHERE type='table'").await?;
    ///     println!("Found {} tables", rows.len());
    ///     Ok(())
    /// }
    /// ```
    pub async fn fetch_all(&self, sql: &str) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        #[cfg(feature = "db")]
        {
            match self {
                DatabaseConnection::SQLite(pool) => {
                    let rows = sqlx::query(sql)
                        .fetch_all(pool)
                        .await
                        .map_err(|e| Error::database(format!("SQL fetch failed: {}", e)))?;
                    
                    let mut result = Vec::new();
                    for row in rows {
                        let mut map = HashMap::new();
                        for (i, column) in row.columns().iter().enumerate() {
                            let column_name = Column::name(column).to_string();
                            // Simplified value extraction - convert everything to string for now
                            let value: serde_json::Value = match row.try_get::<String, _>(i) {
                                Ok(s) => serde_json::Value::String(s),
                                Err(_) => serde_json::Value::Null,
                            };
                            map.insert(column_name, value);
                        }
                        result.push(map);
                    }
                    Ok(result)
                }
                DatabaseConnection::PostgreSQL(pool) => {
                    let rows = sqlx::query(sql)
                        .fetch_all(pool)
                        .await
                        .map_err(|e| Error::database(format!("SQL fetch failed: {}", e)))?;
                    
                    let mut result = Vec::new();
                    for row in rows {
                        let mut map = HashMap::new();
                        for (i, column) in row.columns().iter().enumerate() {
                            let column_name = Column::name(column).to_string();
                            let value: serde_json::Value = match row.try_get::<String, _>(i) {
                                Ok(s) => serde_json::Value::String(s),
                                Err(_) => serde_json::Value::Null,
                            };
                            map.insert(column_name, value);
                        }
                        result.push(map);
                    }
                    Ok(result)
                }
                DatabaseConnection::MySQL(pool) => {
                    let rows = sqlx::query(sql)
                        .fetch_all(pool)
                        .await
                        .map_err(|e| Error::database(format!("SQL fetch failed: {}", e)))?;
                    
                    let mut result = Vec::new();
                    for row in rows {
                        let mut map = HashMap::new();
                        for (i, column) in row.columns().iter().enumerate() {
                            let column_name = Column::name(column).to_string();
                            let value: serde_json::Value = match row.try_get::<String, _>(i) {
                                Ok(s) => serde_json::Value::String(s),
                                Err(_) => serde_json::Value::Null,
                            };
                            map.insert(column_name, value);
                        }
                        result.push(map);
                    }
                    Ok(result)
                }
                DatabaseConnection::Mock => Ok(vec![]),
            }
        }

        #[cfg(not(feature = "db"))]
        {
            let _ = sql; // Avoid unused variable warning
            Ok(vec![])
        }
    }

    /// Fetch a single row from a SQL query
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::db::{DatabaseConnection, DatabaseConfig, DatabaseType};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let config = DatabaseConfig::new(DatabaseType::SQLite, ":memory:");
    ///     let conn = DatabaseConnection::new(config).await?;
    ///     
    ///     let row = conn.fetch_one("SELECT 'Hello' as greeting").await?;
    ///     if let Some(greeting) = row.get("greeting") {
    ///         println!("Greeting: {}", greeting);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn fetch_one(&self, sql: &str) -> Result<Option<HashMap<String, serde_json::Value>>> {
        #[cfg(feature = "db")]
        {
            match self {
                DatabaseConnection::SQLite(pool) => {
                    let row = sqlx::query(sql)
                        .fetch_optional(pool)
                        .await
                        .map_err(|e| Error::database(format!("SQL fetch failed: {}", e)))?;
                    
                    if let Some(row) = row {
                        let mut map = HashMap::new();
                        for (i, column) in row.columns().iter().enumerate() {
                            let column_name = Column::name(column).to_string();
                            let value: serde_json::Value = match row.try_get::<String, _>(i) {
                                Ok(s) => serde_json::Value::String(s),
                                Err(_) => serde_json::Value::Null,
                            };
                            map.insert(column_name, value);
                        }
                        Ok(Some(map))
                    } else {
                        Ok(None)
                    }
                }
                DatabaseConnection::PostgreSQL(pool) => {
                    let row = sqlx::query(sql)
                        .fetch_optional(pool)
                        .await
                        .map_err(|e| Error::database(format!("SQL fetch failed: {}", e)))?;
                    
                    if let Some(row) = row {
                        let mut map = HashMap::new();
                        for (i, column) in row.columns().iter().enumerate() {
                            let column_name = Column::name(column).to_string();
                            let value: serde_json::Value = match row.try_get::<String, _>(i) {
                                Ok(s) => serde_json::Value::String(s),
                                Err(_) => serde_json::Value::Null,
                            };
                            map.insert(column_name, value);
                        }
                        Ok(Some(map))
                    } else {
                        Ok(None)
                    }
                }
                DatabaseConnection::MySQL(pool) => {
                    let row = sqlx::query(sql)
                        .fetch_optional(pool)
                        .await
                        .map_err(|e| Error::database(format!("SQL fetch failed: {}", e)))?;
                    
                    if let Some(row) = row {
                        let mut map = HashMap::new();
                        for (i, column) in row.columns().iter().enumerate() {
                            let column_name = Column::name(column).to_string();
                            let value: serde_json::Value = match row.try_get::<String, _>(i) {
                                Ok(s) => serde_json::Value::String(s),
                                Err(_) => serde_json::Value::Null,
                            };
                            map.insert(column_name, value);
                        }
                        Ok(Some(map))
                    } else {
                        Ok(None)
                    }
                }
                DatabaseConnection::Mock => Ok(None),
            }
        }

        #[cfg(not(feature = "db"))]
        {
            let _ = sql; // Avoid unused variable warning
            Ok(None)
        }
    }

    /// Begin a database transaction
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::db::{DatabaseConnection, DatabaseConfig, DatabaseType};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let config = DatabaseConfig::new(DatabaseType::SQLite, ":memory:");
    ///     let conn = DatabaseConnection::new(config).await?;
    ///     
    ///     let tx = conn.begin_transaction().await?;
    ///     // Perform operations within transaction
    ///     tx.commit().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn begin_transaction(&self) -> Result<DatabaseTransaction> {
        #[cfg(feature = "db")]
        {
            match self {
                DatabaseConnection::SQLite(pool) => {
                    let tx = pool.begin()
                        .await
                        .map_err(|e| Error::database(format!("Failed to begin transaction: {}", e)))?;
                    Ok(DatabaseTransaction::SQLite(tx))
                }
                DatabaseConnection::PostgreSQL(pool) => {
                    let tx = pool.begin()
                        .await
                        .map_err(|e| Error::database(format!("Failed to begin transaction: {}", e)))?;
                    Ok(DatabaseTransaction::PostgreSQL(tx))
                }
                DatabaseConnection::MySQL(pool) => {
                    let tx = pool.begin()
                        .await
                        .map_err(|e| Error::database(format!("Failed to begin transaction: {}", e)))?;
                    Ok(DatabaseTransaction::MySQL(tx))
                }
                DatabaseConnection::Mock => Ok(DatabaseTransaction::Mock),
            }
        }

        #[cfg(not(feature = "db"))]
        {
            Ok(DatabaseTransaction::Mock)
        }
    }

    /// Check if the connection is healthy
    pub async fn is_healthy(&self) -> bool {
        #[cfg(feature = "db")]
        {
            match self {
                DatabaseConnection::SQLite(pool) => !pool.is_closed(),
                DatabaseConnection::PostgreSQL(pool) => !pool.is_closed(),
                DatabaseConnection::MySQL(pool) => !pool.is_closed(),
                DatabaseConnection::Mock => true,
            }
        }

        #[cfg(not(feature = "db"))]
        true
    }

    /// Close the database connection
    pub async fn close(&self) {
        #[cfg(feature = "db")]
        {
            match self {
                DatabaseConnection::SQLite(pool) => pool.close().await,
                DatabaseConnection::PostgreSQL(pool) => pool.close().await,
                DatabaseConnection::MySQL(pool) => pool.close().await,
                DatabaseConnection::Mock => {},
            }
        }
    }
}

/// Database transaction wrapper
pub enum DatabaseTransaction {
    #[cfg(feature = "db")]
    /// SQLite transaction
    SQLite(sqlx::Transaction<'static, Sqlite>),
    #[cfg(feature = "db")]
    /// PostgreSQL transaction
    PostgreSQL(sqlx::Transaction<'static, Postgres>),
    #[cfg(feature = "db")]
    /// MySQL transaction
    MySQL(sqlx::Transaction<'static, MySql>),
    /// Mock transaction for testing
    Mock,
}

impl DatabaseTransaction {
    /// Commit the transaction
    pub async fn commit(self) -> Result<()> {
        #[cfg(feature = "db")]
        {
            match self {
                DatabaseTransaction::SQLite(tx) => {
                    tx.commit()
                        .await
                        .map_err(|e| Error::database(format!("Failed to commit transaction: {}", e)))?;
                }
                DatabaseTransaction::PostgreSQL(tx) => {
                    tx.commit()
                        .await
                        .map_err(|e| Error::database(format!("Failed to commit transaction: {}", e)))?;
                }
                DatabaseTransaction::MySQL(tx) => {
                    tx.commit()
                        .await
                        .map_err(|e| Error::database(format!("Failed to commit transaction: {}", e)))?;
                }
                DatabaseTransaction::Mock => {},
            }
        }
        Ok(())
    }

    /// Rollback the transaction
    pub async fn rollback(self) -> Result<()> {
        #[cfg(feature = "db")]
        {
            match self {
                DatabaseTransaction::SQLite(tx) => {
                    tx.rollback()
                        .await
                        .map_err(|e| Error::database(format!("Failed to rollback transaction: {}", e)))?;
                }
                DatabaseTransaction::PostgreSQL(tx) => {
                    tx.rollback()
                        .await
                        .map_err(|e| Error::database(format!("Failed to rollback transaction: {}", e)))?;
                }
                DatabaseTransaction::MySQL(tx) => {
                    tx.rollback()
                        .await
                        .map_err(|e| Error::database(format!("Failed to rollback transaction: {}", e)))?;
                }
                DatabaseTransaction::Mock => {},
            }
        }
        Ok(())
    }
}

/// Connection pool manager
#[derive(Debug)]
pub struct ConnectionPool {
    connections: HashMap<String, Arc<DatabaseConnection>>,
}

impl ConnectionPool {
    /// Create a new connection pool
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
        }
    }

    /// Add a connection to the pool
    pub fn add_connection(&mut self, name: String, connection: DatabaseConnection) {
        self.connections.insert(name, Arc::new(connection));
    }

    /// Get a connection from the pool
    pub fn get_connection(&self, name: &str) -> Option<Arc<DatabaseConnection>> {
        self.connections.get(name).cloned()
    }

    /// Remove a connection from the pool
    pub fn remove_connection(&mut self, name: &str) -> Option<Arc<DatabaseConnection>> {
        self.connections.remove(name)
    }

    /// Check if pool contains a connection
    pub fn contains(&self, name: &str) -> bool {
        self.connections.contains_key(name)
    }

    /// Get all connection names
    pub fn connection_names(&self) -> Vec<String> {
        self.connections.keys().cloned().collect()
    }

    /// Close all connections in the pool
    pub async fn close_all(&self) {
        for connection in self.connections.values() {
            connection.close().await;
        }
    }
}

impl Default for ConnectionPool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_config_creation() {
        let config = DatabaseConfig::new(DatabaseType::SQLite, "test.db");
        assert_eq!(config.db_type, DatabaseType::SQLite);
        assert_eq!(config.url, "test.db");
        assert_eq!(config.max_connections, 10);
    }

    #[test]
    fn test_database_config_builder() {
        let config = DatabaseConfig::new(DatabaseType::PostgreSQL, "postgresql://localhost/test")
            .with_max_connections(20)
            .with_min_connections(5)
            .with_connect_timeout(Duration::from_secs(60));

        assert_eq!(config.max_connections, 20);
        assert_eq!(config.min_connections, 5);
        assert_eq!(config.connect_timeout, Duration::from_secs(60));
    }

    #[test]
    fn test_connection_pool() {
        let mut pool = ConnectionPool::new();
        assert!(!pool.contains("test"));
        
        let _config = DatabaseConfig::new(DatabaseType::SQLite, ":memory:");
        // Note: We can't actually create a real connection in tests without tokio runtime
        // So we'll use a mock connection
        let connection = DatabaseConnection::Mock;
        
        pool.add_connection("test".to_string(), connection);
        assert!(pool.contains("test"));
        
        let names = pool.connection_names();
        assert_eq!(names.len(), 1);
        assert_eq!(names[0], "test");
        
        let conn = pool.get_connection("test");
        assert!(conn.is_some());
        
        let removed = pool.remove_connection("test");
        assert!(removed.is_some());
        assert!(!pool.contains("test"));
    }

    #[tokio::test]
    async fn test_mock_connection_operations() {
        let connection = DatabaseConnection::Mock;
        
        // Test basic operations with mock connection
        let result = connection.execute("CREATE TABLE test (id INTEGER)").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
        
        let rows = connection.fetch_all("SELECT * FROM test").await;
        assert!(rows.is_ok());
        let rows_data = rows.unwrap();
        assert_eq!(rows_data.len(), 0);
        
        let row = connection.fetch_one("SELECT 1 as test").await;
        assert!(row.is_ok());
        assert!(row.unwrap().is_none());
        
        assert!(connection.is_healthy().await);
    }

    #[tokio::test]
    async fn test_mock_transaction() {
        let connection = DatabaseConnection::Mock;
        let tx = connection.begin_transaction().await;
        assert!(tx.is_ok());
        
        let tx = tx.unwrap();
        let commit_result = tx.commit().await;
        assert!(commit_result.is_ok());
    }
}
