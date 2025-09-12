//! JSON utility functions
//!
//! This module provides comprehensive JSON processing utilities,
//! inspired by Hutool's JSONUtil.

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;

/// JSON utility functions
pub struct JsonUtil;

impl JsonUtil {
    /// Serialize object to JSON string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::json::JsonUtil;
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Serialize)]
    /// struct Person {
    ///     name: String,
    ///     age: u32,
    /// }
    ///
    /// let person = Person {
    ///     name: "Alice".to_string(),
    ///     age: 30,
    /// };
    ///
    /// let json = JsonUtil::to_string(&person).unwrap();
    /// assert!(json.contains("Alice"));
    /// ```
    pub fn to_string<T: Serialize>(value: &T) -> Result<String> {
        serde_json::to_string(value)
            .map_err(|e| Error::conversion(format!("JSON serialization failed: {}", e)))
    }

    /// Serialize object to pretty-formatted JSON string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::json::JsonUtil;
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Serialize)]
    /// struct Person {
    ///     name: String,
    ///     age: u32,
    /// }
    ///
    /// let person = Person {
    ///     name: "Alice".to_string(),
    ///     age: 30,
    /// };
    ///
    /// let json = JsonUtil::to_string_pretty(&person).unwrap();
    /// assert!(json.contains("  \"name\": \"Alice\""));
    /// ```
    pub fn to_string_pretty<T: Serialize>(value: &T) -> Result<String> {
        serde_json::to_string_pretty(value)
            .map_err(|e| Error::conversion(format!("JSON pretty serialization failed: {}", e)))
    }

    /// Deserialize JSON string to object
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::json::JsonUtil;
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Deserialize, PartialEq, Debug)]
    /// struct Person {
    ///     name: String,
    ///     age: u32,
    /// }
    ///
    /// let json = r#"{"name": "Alice", "age": 30}"#;
    /// let person: Person = JsonUtil::from_str(json).unwrap();
    /// assert_eq!(person.name, "Alice");
    /// assert_eq!(person.age, 30);
    /// ```
    pub fn from_str<'a, T: Deserialize<'a>>(s: &'a str) -> Result<T> {
        serde_json::from_str(s)
            .map_err(|e| Error::conversion(format!("JSON deserialization failed: {}", e)))
    }

    /// Parse JSON string to serde_json::Value
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::json::JsonUtil;
    /// use serde_json::Value;
    ///
    /// let json = r#"{"name": "Alice", "age": 30}"#;
    /// let value = JsonUtil::parse(json).unwrap();
    /// assert_eq!(value["name"], "Alice");
    /// assert_eq!(value["age"], 30);
    /// ```
    pub fn parse(s: &str) -> Result<Value> {
        serde_json::from_str(s)
            .map_err(|e| Error::conversion(format!("JSON parsing failed: {}", e)))
    }

    /// Convert serde_json::Value to string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::json::JsonUtil;
    /// use serde_json::{json, Value};
    ///
    /// let value = json!({"name": "Alice", "age": 30});
    /// let json_str = JsonUtil::stringify(&value).unwrap();
    /// assert!(json_str.contains("Alice"));
    /// ```
    pub fn stringify(value: &Value) -> Result<String> {
        serde_json::to_string(value)
            .map_err(|e| Error::conversion(format!("JSON stringify failed: {}", e)))
    }

    /// Check if string is valid JSON
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::json::JsonUtil;
    ///
    /// assert!(JsonUtil::is_valid(r#"{"name": "Alice"}"#));
    /// assert!(JsonUtil::is_valid(r#"[1, 2, 3]"#));
    /// assert!(!JsonUtil::is_valid(r#"{"name": "Alice""#)); // Invalid JSON
    /// ```
    pub fn is_valid(s: &str) -> bool {
        serde_json::from_str::<Value>(s).is_ok()
    }

    /// Minify JSON string (remove whitespace)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::json::JsonUtil;
    ///
    /// let pretty_json = r#"{
    ///     "name": "Alice",
    ///     "age": 30
    /// }"#;
    /// let minified = JsonUtil::minify(pretty_json).unwrap();
    /// // JSON key order might vary, so check that both keys are present
    /// assert!(minified.contains(r#""name":"Alice""#));
    /// assert!(minified.contains(r#""age":30"#));
    /// ```
    pub fn minify(s: &str) -> Result<String> {
        let value: Value = Self::parse(s)?;
        Self::stringify(&value)
    }

    /// Pretty format JSON string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::json::JsonUtil;
    ///
    /// let compact_json = r#"{"name":"Alice","age":30}"#;
    /// let pretty = JsonUtil::prettify(compact_json).unwrap();
    /// assert!(pretty.contains("  \"name\": \"Alice\""));
    /// ```
    pub fn prettify(s: &str) -> Result<String> {
        let value: Value = Self::parse(s)?;
        serde_json::to_string_pretty(&value)
            .map_err(|e| Error::conversion(format!("JSON prettify failed: {}", e)))
    }

    /// Get value by JSON path (simplified dot notation)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::json::JsonUtil;
    ///
    /// let json = r#"{"user": {"name": "Alice", "address": {"city": "New York"}}}"#;
    /// let value = JsonUtil::parse(json).unwrap();
    /// 
    /// let name = JsonUtil::get_by_path(&value, "user.name").unwrap();
    /// assert_eq!(name, "Alice");
    /// 
    /// let city = JsonUtil::get_by_path(&value, "user.address.city").unwrap();
    /// assert_eq!(city, "New York");
    /// ```
    pub fn get_by_path<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = value;
        
        for part in parts {
            match current {
                Value::Object(map) => {
                    current = map.get(part)?;
                }
                Value::Array(arr) => {
                    if let Ok(index) = part.parse::<usize>() {
                        current = arr.get(index)?;
                    } else {
                        return None;
                    }
                }
                _ => return None,
            }
        }
        
        Some(current)
    }

    /// Set value by JSON path (simplified dot notation)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::json::JsonUtil;
    /// use serde_json::{json, Value};
    ///
    /// let mut value = json!({"user": {"name": "Alice"}});
    /// JsonUtil::set_by_path(&mut value, "user.age", json!(30)).unwrap();
    /// 
    /// assert_eq!(value["user"]["age"], 30);
    /// ```
    pub fn set_by_path(value: &mut Value, path: &str, new_value: Value) -> Result<()> {
        let parts: Vec<&str> = path.split('.').collect();
        if parts.is_empty() {
            return Err(Error::validation("Empty path".to_string()));
        }
        
        let mut current = value;
        
        for (i, part) in parts.iter().enumerate() {
            if i == parts.len() - 1 {
                // Last part, set the value
                match current {
                    Value::Object(map) => {
                        map.insert(part.to_string(), new_value);
                        return Ok(());
                    }
                    Value::Array(arr) => {
                        if let Ok(index) = part.parse::<usize>() {
                            if index < arr.len() {
                                arr[index] = new_value;
                                return Ok(());
                            }
                        }
                        return Err(Error::validation("Invalid array index".to_string()));
                    }
                    _ => return Err(Error::validation("Cannot set value on non-object/array".to_string())),
                }
            } else {
                // Navigate deeper
                match current {
                    Value::Object(map) => {
                        if !map.contains_key(*part) {
                            map.insert(part.to_string(), Value::Object(Map::new()));
                        }
                        current = map.get_mut(*part).unwrap();
                    }
                    Value::Array(arr) => {
                        if let Ok(index) = part.parse::<usize>() {
                            if index >= arr.len() {
                                return Err(Error::validation("Array index out of bounds".to_string()));
                            }
                            current = &mut arr[index];
                        } else {
                            return Err(Error::validation("Invalid array index".to_string()));
                        }
                    }
                    _ => return Err(Error::validation("Cannot navigate through non-object/array".to_string())),
                }
            }
        }
        
        Ok(())
    }

    /// Remove value by JSON path
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::json::JsonUtil;
    /// use serde_json::json;
    ///
    /// let mut value = json!({"user": {"name": "Alice", "age": 30}});
    /// JsonUtil::remove_by_path(&mut value, "user.age").unwrap();
    /// 
    /// assert!(value["user"]["age"].is_null());
    /// ```
    pub fn remove_by_path(value: &mut Value, path: &str) -> Result<Value> {
        let parts: Vec<&str> = path.split('.').collect();
        if parts.is_empty() {
            return Err(Error::validation("Empty path".to_string()));
        }
        
        let mut current = value;
        
        // Navigate to parent
        for part in &parts[..parts.len() - 1] {
            match current {
                Value::Object(map) => {
                    current = map.get_mut(*part)
                        .ok_or_else(|| Error::not_found(format!("Path not found: {}", part)))?;
                }
                Value::Array(arr) => {
                    if let Ok(index) = part.parse::<usize>() {
                        current = arr.get_mut(index)
                            .ok_or_else(|| Error::not_found(format!("Array index not found: {}", index)))?;
                    } else {
                        return Err(Error::validation("Invalid array index".to_string()));
                    }
                }
                _ => return Err(Error::validation("Cannot navigate through non-object/array".to_string())),
            }
        }
        
        // Remove the final key
        let final_key = parts[parts.len() - 1];
        match current {
            Value::Object(map) => {
                map.remove(final_key)
                    .ok_or_else(|| Error::not_found(format!("Key not found: {}", final_key)))
            }
            Value::Array(arr) => {
                if let Ok(index) = final_key.parse::<usize>() {
                    if index < arr.len() {
                        Ok(arr.remove(index))
                    } else {
                        Err(Error::not_found(format!("Array index not found: {}", index)))
                    }
                } else {
                    Err(Error::validation("Invalid array index".to_string()))
                }
            }
            _ => Err(Error::validation("Cannot remove from non-object/array".to_string())),
        }
    }

    /// Merge two JSON values
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::json::JsonUtil;
    /// use serde_json::json;
    ///
    /// let mut base = json!({"a": 1, "b": {"c": 2}});
    /// let overlay = json!({"b": {"d": 3}, "e": 4});
    /// 
    /// JsonUtil::merge(&mut base, &overlay);
    /// 
    /// assert_eq!(base["a"], 1);
    /// assert_eq!(base["b"]["c"], 2);
    /// assert_eq!(base["b"]["d"], 3);
    /// assert_eq!(base["e"], 4);
    /// ```
    pub fn merge(base: &mut Value, overlay: &Value) {
        match (base, overlay) {
            (Value::Object(base_map), Value::Object(overlay_map)) => {
                for (key, value) in overlay_map {
                    if base_map.contains_key(key) {
                        Self::merge(base_map.get_mut(key).unwrap(), value);
                    } else {
                        base_map.insert(key.clone(), value.clone());
                    }
                }
            }
            (base_value, overlay_value) => {
                *base_value = overlay_value.clone();
            }
        }
    }

    /// Convert JSON to HashMap<String, String> (flattened)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::json::JsonUtil;
    /// use serde_json::json;
    /// use std::collections::HashMap;
    ///
    /// let value = json!({"user": {"name": "Alice", "age": 30}});
    /// let map = JsonUtil::to_flat_map(&value);
    /// 
    /// assert_eq!(map.get("user.name"), Some(&"Alice".to_string()));
    /// assert_eq!(map.get("user.age"), Some(&"30".to_string()));
    /// ```
    pub fn to_flat_map(value: &Value) -> HashMap<String, String> {
        let mut result = HashMap::new();
        Self::flatten_value(value, String::new(), &mut result);
        result
    }

    fn flatten_value(value: &Value, prefix: String, result: &mut HashMap<String, String>) {
        match value {
            Value::Object(map) => {
                for (key, val) in map {
                    let new_prefix = if prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", prefix, key)
                    };
                    Self::flatten_value(val, new_prefix, result);
                }
            }
            Value::Array(arr) => {
                for (index, val) in arr.iter().enumerate() {
                    let new_prefix = if prefix.is_empty() {
                        index.to_string()
                    } else {
                        format!("{}.{}", prefix, index)
                    };
                    Self::flatten_value(val, new_prefix, result);
                }
            }
            _ => {
                let value_str = match value {
                    Value::String(s) => s.clone(),
                    Value::Number(n) => n.to_string(),
                    Value::Bool(b) => b.to_string(),
                    Value::Null => "null".to_string(),
                    _ => unreachable!(),
                };
                result.insert(prefix, value_str);
            }
        }
    }

    /// Create JSON object from key-value pairs
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::json::JsonUtil;
    /// use serde_json::Value;
    ///
    /// let pairs = vec![
    ///     ("name", "Alice"),
    ///     ("city", "New York"),
    /// ];
    /// 
    /// let json = JsonUtil::from_pairs(&pairs);
    /// assert_eq!(json["name"], "Alice");
    /// assert_eq!(json["city"], "New York");
    /// ```
    pub fn from_pairs(pairs: &[(&str, &str)]) -> Value {
        let mut map = Map::new();
        for (key, value) in pairs {
            map.insert(key.to_string(), Value::String(value.to_string()));
        }
        Value::Object(map)
    }

    /// Get all keys from JSON object (recursive)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::json::JsonUtil;
    /// use serde_json::json;
    ///
    /// let value = json!({"user": {"name": "Alice", "age": 30}, "status": "active"});
    /// let keys = JsonUtil::get_all_keys(&value);
    /// 
    /// assert!(keys.contains(&"user.name".to_string()));
    /// assert!(keys.contains(&"user.age".to_string()));
    /// assert!(keys.contains(&"status".to_string()));
    /// ```
    pub fn get_all_keys(value: &Value) -> Vec<String> {
        let mut keys = Vec::new();
        Self::collect_keys(value, String::new(), &mut keys);
        keys
    }

    fn collect_keys(value: &Value, prefix: String, keys: &mut Vec<String>) {
        match value {
            Value::Object(map) => {
                for (key, val) in map {
                    let full_key = if prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", prefix, key)
                    };
                    
                    match val {
                        Value::Object(_) | Value::Array(_) => {
                            Self::collect_keys(val, full_key, keys);
                        }
                        _ => {
                            keys.push(full_key);
                        }
                    }
                }
            }
            Value::Array(arr) => {
                for (index, val) in arr.iter().enumerate() {
                    let full_key = if prefix.is_empty() {
                        index.to_string()
                    } else {
                        format!("{}.{}", prefix, index)
                    };
                    
                    match val {
                        Value::Object(_) | Value::Array(_) => {
                            Self::collect_keys(val, full_key, keys);
                        }
                        _ => {
                            keys.push(full_key);
                        }
                    }
                }
            }
            _ => {
                if !prefix.is_empty() {
                    keys.push(prefix);
                }
            }
        }
    }

    /// Count total number of elements in JSON (recursive)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::json::JsonUtil;
    /// use serde_json::json;
    ///
    /// let value = json!({"user": {"name": "Alice", "age": 30}, "status": "active"});
    /// let count = JsonUtil::count_elements(&value);
    /// assert_eq!(count, 3); // name, age, status
    /// ```
    pub fn count_elements(value: &Value) -> usize {
        match value {
            Value::Object(map) => {
                map.values().map(|v| Self::count_elements(v)).sum()
            }
            Value::Array(arr) => {
                arr.iter().map(|v| Self::count_elements(v)).sum()
            }
            _ => 1,
        }
    }

    /// Convert Value to specific type with error handling
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::json::JsonUtil;
    /// use serde_json::json;
    ///
    /// let value = json!(42);
    /// let num: i32 = JsonUtil::convert_to(&value).unwrap();
    /// assert_eq!(num, 42);
    /// ```
    pub fn convert_to<T>(value: &Value) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        serde_json::from_value(value.clone())
            .map_err(|e| Error::conversion(format!("Type conversion failed: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestPerson {
        name: String,
        age: u32,
    }

    #[test]
    fn test_to_string_from_str() {
        let person = TestPerson {
            name: "Alice".to_string(),
            age: 30,
        };
        
        let json_str = JsonUtil::to_string(&person).unwrap();
        let parsed: TestPerson = JsonUtil::from_str(&json_str).unwrap();
        
        assert_eq!(parsed, person);
    }

    #[test]
    fn test_pretty_formatting() {
        let person = TestPerson {
            name: "Alice".to_string(),
            age: 30,
        };
        
        let pretty = JsonUtil::to_string_pretty(&person).unwrap();
        assert!(pretty.contains("  \"name\": \"Alice\""));
        assert!(pretty.contains("  \"age\": 30"));
    }

    #[test]
    fn test_parse_and_stringify() {
        let json_str = r#"{"name": "Alice", "age": 30}"#;
        let value = JsonUtil::parse(json_str).unwrap();
        let stringified = JsonUtil::stringify(&value).unwrap();
        
        assert!(stringified.contains("Alice"));
        assert!(stringified.contains("30"));
    }

    #[test]
    fn test_is_valid() {
        assert!(JsonUtil::is_valid(r#"{"name": "Alice"}"#));
        assert!(JsonUtil::is_valid(r#"[1, 2, 3]"#));
        assert!(JsonUtil::is_valid(r#""string""#));
        assert!(JsonUtil::is_valid("42"));
        assert!(JsonUtil::is_valid("true"));
        
        assert!(!JsonUtil::is_valid(r#"{"name": "Alice""#));
        assert!(!JsonUtil::is_valid(r#"invalid json"#));
    }

    #[test]
    fn test_minify_prettify() {
        let pretty_json = r#"{
            "name": "Alice",
            "age": 30
        }"#;
        
        let minified = JsonUtil::minify(pretty_json).unwrap();
        // JSON key order might vary, so check that both keys are present
        assert!(minified.contains(r#""name":"Alice""#));
        assert!(minified.contains(r#""age":30"#));
        
        let prettified = JsonUtil::prettify(&minified).unwrap();
        assert!(prettified.contains("  \"name\": \"Alice\""));
    }

    #[test]
    fn test_path_operations() {
        let mut value = json!({
            "user": {
                "name": "Alice",
                "address": {
                    "city": "New York"
                }
            }
        });
        
        // Test get_by_path
        let name = JsonUtil::get_by_path(&value, "user.name").unwrap();
        assert_eq!(name, "Alice");
        
        let city = JsonUtil::get_by_path(&value, "user.address.city").unwrap();
        assert_eq!(city, "New York");
        
        // Test set_by_path
        JsonUtil::set_by_path(&mut value, "user.age", json!(30)).unwrap();
        assert_eq!(value["user"]["age"], 30);
        
        // Test remove_by_path
        let removed = JsonUtil::remove_by_path(&mut value, "user.address.city").unwrap();
        assert_eq!(removed, "New York");
        assert!(value["user"]["address"]["city"].is_null());
    }

    #[test]
    fn test_merge() {
        let mut base = json!({"a": 1, "b": {"c": 2}});
        let overlay = json!({"b": {"d": 3}, "e": 4});
        
        JsonUtil::merge(&mut base, &overlay);
        
        assert_eq!(base["a"], 1);
        assert_eq!(base["b"]["c"], 2);
        assert_eq!(base["b"]["d"], 3);
        assert_eq!(base["e"], 4);
    }

    #[test]
    fn test_to_flat_map() {
        let value = json!({
            "user": {
                "name": "Alice",
                "age": 30
            },
            "status": "active"
        });
        
        let flat_map = JsonUtil::to_flat_map(&value);
        
        assert_eq!(flat_map.get("user.name"), Some(&"Alice".to_string()));
        assert_eq!(flat_map.get("user.age"), Some(&"30".to_string()));
        assert_eq!(flat_map.get("status"), Some(&"active".to_string()));
    }

    #[test]
    fn test_from_pairs() {
        let pairs = vec![
            ("name", "Alice"),
            ("city", "New York"),
        ];
        
        let json = JsonUtil::from_pairs(&pairs);
        assert_eq!(json["name"], "Alice");
        assert_eq!(json["city"], "New York");
    }

    #[test]
    fn test_get_all_keys() {
        let value = json!({
            "user": {
                "name": "Alice",
                "age": 30
            },
            "status": "active"
        });
        
        let keys = JsonUtil::get_all_keys(&value);
        
        assert!(keys.contains(&"user.name".to_string()));
        assert!(keys.contains(&"user.age".to_string()));
        assert!(keys.contains(&"status".to_string()));
        assert_eq!(keys.len(), 3);
    }

    #[test]
    fn test_count_elements() {
        let value = json!({
            "user": {
                "name": "Alice",
                "age": 30
            },
            "status": "active"
        });
        
        let count = JsonUtil::count_elements(&value);
        assert_eq!(count, 3);
    }

    #[test]
    fn test_convert_to() {
        let value = json!(42);
        let num: i32 = JsonUtil::convert_to(&value).unwrap();
        assert_eq!(num, 42);
        
        let value = json!({"name": "Alice", "age": 30});
        let person: TestPerson = JsonUtil::convert_to(&value).unwrap();
        assert_eq!(person.name, "Alice");
        assert_eq!(person.age, 30);
    }

    #[test]
    fn test_array_path_operations() {
        let mut value = json!({
            "users": [
                {"name": "Alice", "age": 30},
                {"name": "Bob", "age": 25}
            ]
        });
        
        // Test get from array
        let name = JsonUtil::get_by_path(&value, "users.0.name").unwrap();
        assert_eq!(name, "Alice");
        
        // Test set in array
        JsonUtil::set_by_path(&mut value, "users.1.age", json!(26)).unwrap();
        assert_eq!(value["users"][1]["age"], 26);
    }
}
