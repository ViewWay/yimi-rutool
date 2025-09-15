//! Database migration utilities
//!
//! This module provides tools for managing database schema changes
//! through versioned migrations.

use crate::db::connection::DatabaseConnection;
use crate::error::{Error, Result};
use std::fmt;

/// Database migration
#[derive(Debug, Clone)]
pub struct Migration {
    /// Migration version (timestamp or sequential number)
    pub version: String,
    /// Migration name/description
    pub name: String,
    /// SQL statements to apply the migration
    pub up_sql: Vec<String>,
    /// SQL statements to rollback the migration
    pub down_sql: Vec<String>,
}

impl Migration {
    /// Create a new migration
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::db::Migration;
    ///
    /// let migration = Migration::new(
    ///     "20231201000001",
    ///     "create_users_table",
    ///     vec!["CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)".to_string()],
    ///     vec!["DROP TABLE users".to_string()]
    /// );
    /// ```
    pub fn new(version: &str, name: &str, up_sql: Vec<String>, down_sql: Vec<String>) -> Self {
        Self {
            version: version.to_string(),
            name: name.to_string(),
            up_sql,
            down_sql,
        }
    }

    /// Create a migration with single up and down statements
    pub fn simple(version: &str, name: &str, up_sql: &str, down_sql: &str) -> Self {
        Self {
            version: version.to_string(),
            name: name.to_string(),
            up_sql: vec![up_sql.to_string()],
            down_sql: vec![down_sql.to_string()],
        }
    }

    /// Get the migration's unique identifier
    pub fn id(&self) -> String {
        format!("{}_{}", self.version, self.name)
    }
}

impl fmt::Display for Migration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.version, self.name)
    }
}

/// Migration runner for managing database schema changes
#[derive(Debug)]
pub struct MigrationRunner {
    connection: DatabaseConnection,
    migrations: Vec<Migration>,
    migrations_table: String,
}

impl MigrationRunner {
    /// Create a new migration runner
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::db::{DatabaseConnection, DatabaseConfig, DatabaseType, MigrationRunner};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let config = DatabaseConfig::new(DatabaseType::SQLite, ":memory:");
    ///     let conn = DatabaseConnection::new(config).await?;
    ///     let runner = MigrationRunner::new(conn);
    ///     Ok(())
    /// }
    /// ```
    pub fn new(connection: DatabaseConnection) -> Self {
        Self {
            connection,
            migrations: Vec::new(),
            migrations_table: "schema_migrations".to_string(),
        }
    }

    /// Set a custom migrations table name
    pub fn with_migrations_table(mut self, table_name: &str) -> Self {
        self.migrations_table = table_name.to_string();
        self
    }

    /// Add a migration to the runner
    pub fn add_migration(&mut self, migration: Migration) {
        self.migrations.push(migration);
    }

    /// Add multiple migrations to the runner
    pub fn add_migrations(&mut self, migrations: Vec<Migration>) {
        self.migrations.extend(migrations);
    }

    /// Initialize the migrations table
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::db::{DatabaseConnection, MigrationRunner};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let conn = DatabaseConnection::Mock;
    ///     let mut runner = MigrationRunner::new(conn);
    ///     runner.init().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn init(&self) -> Result<()> {
        let create_table_sql = format!(
            r"CREATE TABLE IF NOT EXISTS {} (
                version TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
            self.migrations_table
        );

        self.connection.execute(&create_table_sql).await?;
        Ok(())
    }

    /// Get list of applied migrations
    pub async fn get_applied_migrations(&self) -> Result<Vec<String>> {
        let sql = format!(
            "SELECT version FROM {} ORDER BY version",
            self.migrations_table
        );

        let rows = self.connection.fetch_all(&sql).await?;
        let versions: Vec<String> = rows
            .into_iter()
            .filter_map(|row| {
                row.get("version")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
            })
            .collect();

        Ok(versions)
    }

    /// Get list of pending migrations
    pub async fn get_pending_migrations(&self) -> Result<Vec<&Migration>> {
        let applied = self.get_applied_migrations().await?;
        let applied_set: std::collections::HashSet<_> = applied.into_iter().collect();

        let pending: Vec<&Migration> = self
            .migrations
            .iter()
            .filter(|migration| !applied_set.contains(&migration.version))
            .collect();

        Ok(pending)
    }

    /// Run all pending migrations
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::db::{DatabaseConnection, Migration, MigrationRunner};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let conn = DatabaseConnection::Mock;
    ///     let mut runner = MigrationRunner::new(conn);
    ///     
    ///     let migration = Migration::simple(
    ///         "001",
    ///         "create_users",
    ///         "CREATE TABLE users (id INTEGER PRIMARY KEY)",
    ///         "DROP TABLE users"
    ///     );
    ///     
    ///     runner.add_migration(migration);
    ///     runner.init().await?;
    ///     
    ///     let results = runner.migrate_up().await?;
    ///     println!("Applied {} migrations", results.len());
    ///     Ok(())
    /// }
    /// ```
    pub async fn migrate_up(&self) -> Result<Vec<MigrationResult>> {
        self.init().await?;

        let pending = self.get_pending_migrations().await?;
        let mut results = Vec::new();

        for migration in pending {
            let result = self.apply_migration(migration).await;
            let migration_result = MigrationResult {
                migration: migration.clone(),
                success: result.is_ok(),
                error: result.err().map(|e| e.to_string()),
            };

            results.push(migration_result.clone());

            if !migration_result.success {
                break; // Stop on first failure
            }
        }

        Ok(results)
    }

    /// Run migrations up to a specific version
    pub async fn migrate_to(&self, target_version: &str) -> Result<Vec<MigrationResult>> {
        self.init().await?;

        let pending = self.get_pending_migrations().await?;
        let mut results = Vec::new();

        for migration in pending {
            if migration.version.as_str() > target_version {
                break;
            }

            let result = self.apply_migration(migration).await;
            let migration_result = MigrationResult {
                migration: migration.clone(),
                success: result.is_ok(),
                error: result.err().map(|e| e.to_string()),
            };

            results.push(migration_result.clone());

            if !migration_result.success {
                break;
            }
        }

        Ok(results)
    }

    /// Rollback the last migration
    pub async fn rollback(&self) -> Result<Option<MigrationResult>> {
        let applied = self.get_applied_migrations().await?;

        if let Some(last_version) = applied.last() {
            if let Some(migration) = self.migrations.iter().find(|m| &m.version == last_version) {
                let result = self.rollback_migration(migration).await;
                let migration_result = MigrationResult {
                    migration: migration.clone(),
                    success: result.is_ok(),
                    error: result.err().map(|e| e.to_string()),
                };
                return Ok(Some(migration_result));
            }
        }

        Ok(None)
    }

    /// Rollback to a specific version
    pub async fn rollback_to(&self, target_version: &str) -> Result<Vec<MigrationResult>> {
        let applied = self.get_applied_migrations().await?;
        let mut results = Vec::new();

        // Find migrations to rollback (in reverse order)
        let to_rollback: Vec<&String> = applied
            .iter()
            .rev()
            .take_while(|&version| version.as_str() > target_version)
            .collect();

        for version in to_rollback {
            if let Some(migration) = self.migrations.iter().find(|m| &m.version == version) {
                let result = self.rollback_migration(migration).await;
                let migration_result = MigrationResult {
                    migration: migration.clone(),
                    success: result.is_ok(),
                    error: result.err().map(|e| e.to_string()),
                };

                results.push(migration_result.clone());

                if !migration_result.success {
                    break;
                }
            }
        }

        Ok(results)
    }

    /// Get migration status
    pub async fn status(&self) -> Result<MigrationStatus> {
        self.init().await?;

        let applied = self.get_applied_migrations().await?;
        let pending = self.get_pending_migrations().await?;

        Ok(MigrationStatus {
            total_migrations: self.migrations.len(),
            applied_count: applied.len(),
            pending_count: pending.len(),
            applied_versions: applied,
            pending_versions: pending.iter().map(|m| m.version.clone()).collect(),
        })
    }

    /// Apply a single migration
    async fn apply_migration(&self, migration: &Migration) -> Result<()> {
        // Begin transaction
        let tx = self.connection.begin_transaction().await?;

        // Execute up SQL statements
        for sql in &migration.up_sql {
            if let Err(e) = self.connection.execute(sql).await {
                tx.rollback().await?;
                return Err(Error::database(format!(
                    "Failed to apply migration {}: {}",
                    migration.version, e
                )));
            }
        }

        // Record migration as applied
        let record_sql = format!(
            "INSERT INTO {} (version, name) VALUES ('{}', '{}')",
            self.migrations_table, migration.version, migration.name
        );

        if let Err(e) = self.connection.execute(&record_sql).await {
            tx.rollback().await?;
            return Err(Error::database(format!(
                "Failed to record migration {}: {}",
                migration.version, e
            )));
        }

        // Commit transaction
        tx.commit().await?;
        Ok(())
    }

    /// Rollback a single migration
    async fn rollback_migration(&self, migration: &Migration) -> Result<()> {
        // Begin transaction
        let tx = self.connection.begin_transaction().await?;

        // Execute down SQL statements
        for sql in &migration.down_sql {
            if let Err(e) = self.connection.execute(sql).await {
                tx.rollback().await?;
                return Err(Error::database(format!(
                    "Failed to rollback migration {}: {}",
                    migration.version, e
                )));
            }
        }

        // Remove migration record
        let remove_sql = format!(
            "DELETE FROM {} WHERE version = '{}'",
            self.migrations_table, migration.version
        );

        if let Err(e) = self.connection.execute(&remove_sql).await {
            tx.rollback().await?;
            return Err(Error::database(format!(
                "Failed to remove migration record {}: {}",
                migration.version, e
            )));
        }

        // Commit transaction
        tx.commit().await?;
        Ok(())
    }

    /// Generate SQL for all pending migrations
    pub async fn generate_sql(&self) -> Result<String> {
        let pending = self.get_pending_migrations().await?;
        let mut sql = String::new();

        for migration in pending {
            sql.push_str(&format!("-- Migration: {}\n", migration));
            for up_sql in &migration.up_sql {
                sql.push_str(up_sql);
                if !up_sql.ends_with(';') {
                    sql.push(';');
                }
                sql.push('\n');
            }
            sql.push('\n');
        }

        Ok(sql)
    }

    /// Validate all migrations (check for syntax errors, dependencies, etc.)
    pub fn validate(&self) -> Result<Vec<ValidationError>> {
        let mut errors = Vec::new();

        // Check for duplicate versions
        let mut versions = std::collections::HashSet::new();
        for migration in &self.migrations {
            if !versions.insert(&migration.version) {
                errors.push(ValidationError {
                    migration_version: migration.version.clone(),
                    error_type: ValidationErrorType::DuplicateVersion,
                    message: format!("Duplicate version: {}", migration.version),
                });
            }
        }

        // Check for empty SQL statements
        for migration in &self.migrations {
            if migration.up_sql.is_empty() {
                errors.push(ValidationError {
                    migration_version: migration.version.clone(),
                    error_type: ValidationErrorType::EmptyUpSql,
                    message: "Migration has no up SQL statements".to_string(),
                });
            }

            if migration.down_sql.is_empty() {
                errors.push(ValidationError {
                    migration_version: migration.version.clone(),
                    error_type: ValidationErrorType::EmptyDownSql,
                    message: "Migration has no down SQL statements".to_string(),
                });
            }
        }

        // Check for invalid version format (should be sortable)
        let mut sorted_versions: Vec<_> = self.migrations.iter().map(|m| &m.version).collect();
        sorted_versions.sort();

        let original_order: Vec<_> = self.migrations.iter().map(|m| &m.version).collect();
        if sorted_versions != original_order {
            errors.push(ValidationError {
                migration_version: "multiple".to_string(),
                error_type: ValidationErrorType::InvalidVersionOrder,
                message: "Migration versions are not in sorted order".to_string(),
            });
        }

        Ok(errors)
    }
}

/// Result of a migration operation
#[derive(Debug, Clone)]
pub struct MigrationResult {
    /// The migration that was processed
    pub migration: Migration,
    /// Whether the operation was successful
    pub success: bool,
    /// Error message if the operation failed
    pub error: Option<String>,
}

impl fmt::Display for MigrationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.success {
            write!(f, "✓ {}", self.migration)
        } else {
            write!(
                f,
                "✗ {}: {}",
                self.migration,
                self.error.as_ref().unwrap_or(&"Unknown error".to_string())
            )
        }
    }
}

/// Migration status information
#[derive(Debug)]
pub struct MigrationStatus {
    /// Total number of migrations
    pub total_migrations: usize,
    /// Number of applied migrations
    pub applied_count: usize,
    /// Number of pending migrations
    pub pending_count: usize,
    /// List of applied migration versions
    pub applied_versions: Vec<String>,
    /// List of pending migration versions
    pub pending_versions: Vec<String>,
}

impl fmt::Display for MigrationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Migration Status:")?;
        writeln!(f, "  Total migrations: {}", self.total_migrations)?;
        writeln!(f, "  Applied: {}", self.applied_count)?;
        writeln!(f, "  Pending: {}", self.pending_count)?;

        if !self.applied_versions.is_empty() {
            writeln!(
                f,
                "  Applied versions: {}",
                self.applied_versions.join(", ")
            )?;
        }

        if !self.pending_versions.is_empty() {
            writeln!(
                f,
                "  Pending versions: {}",
                self.pending_versions.join(", ")
            )?;
        }

        Ok(())
    }
}

/// Migration validation error
#[derive(Debug)]
pub struct ValidationError {
    /// Version of the migration with the error
    pub migration_version: String,
    /// Type of validation error
    pub error_type: ValidationErrorType,
    /// Error message
    pub message: String,
}

/// Types of validation errors
#[derive(Debug)]
pub enum ValidationErrorType {
    /// Duplicate migration version
    DuplicateVersion,
    /// Migration has no up SQL statements
    EmptyUpSql,
    /// Migration has no down SQL statements
    EmptyDownSql,
    /// Migration versions are not in correct order
    InvalidVersionOrder,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.migration_version, self.message)
    }
}

/// Helper for generating migration timestamps
pub struct MigrationTimestamp;

impl MigrationTimestamp {
    /// Generate a timestamp-based version for a new migration
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::db::MigrationTimestamp;
    ///
    /// let version = MigrationTimestamp::generate();
    /// println!("New migration version: {}", version);
    /// ```
    pub fn generate() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();

        format!("{}", now.as_secs())
    }

    /// Generate a formatted timestamp (YYYYMMDDHHMMSS)
    pub fn generate_formatted() -> String {
        #[cfg(feature = "chrono")]
        {
            use chrono::Utc;
            Utc::now().format("%Y%m%d%H%M%S").to_string()
        }

        #[cfg(not(feature = "chrono"))]
        {
            Self::generate()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connection::DatabaseConnection;

    #[test]
    fn test_migration_creation() {
        let migration = Migration::new(
            "001",
            "create_users",
            vec!["CREATE TABLE users (id INTEGER)".to_string()],
            vec!["DROP TABLE users".to_string()],
        );

        assert_eq!(migration.version, "001");
        assert_eq!(migration.name, "create_users");
        assert_eq!(migration.id(), "001_create_users");
    }

    #[test]
    fn test_migration_simple() {
        let migration = Migration::simple(
            "001",
            "create_users",
            "CREATE TABLE users (id INTEGER)",
            "DROP TABLE users",
        );

        assert_eq!(migration.up_sql.len(), 1);
        assert_eq!(migration.down_sql.len(), 1);
    }

    #[tokio::test]
    async fn test_migration_runner_with_mock() {
        let connection = DatabaseConnection::Mock;
        let mut runner = MigrationRunner::new(connection);

        let migration = Migration::simple(
            "001",
            "create_users",
            "CREATE TABLE users (id INTEGER PRIMARY KEY)",
            "DROP TABLE users",
        );

        runner.add_migration(migration);

        // Test initialization
        let init_result = runner.init().await;
        assert!(init_result.is_ok());

        // Test status
        let status = runner.status().await;
        assert!(status.is_ok());
        let status = status.unwrap();
        assert_eq!(status.total_migrations, 1);
    }

    #[test]
    fn test_migration_validation() {
        let mut runner = MigrationRunner::new(DatabaseConnection::Mock);

        // Add valid migration
        runner.add_migration(Migration::simple(
            "001",
            "create_users",
            "CREATE TABLE users (id INTEGER)",
            "DROP TABLE users",
        ));

        // Add duplicate version
        runner.add_migration(Migration::simple(
            "001",
            "create_posts",
            "CREATE TABLE posts (id INTEGER)",
            "DROP TABLE posts",
        ));

        // Add migration with empty SQL
        runner.add_migration(Migration::new("002", "empty_migration", vec![], vec![]));

        let errors = runner.validate().unwrap();
        assert!(!errors.is_empty());

        // Check for duplicate version error
        let has_duplicate = errors
            .iter()
            .any(|e| matches!(e.error_type, ValidationErrorType::DuplicateVersion));
        assert!(has_duplicate);

        // Check for empty SQL errors
        let has_empty_up = errors
            .iter()
            .any(|e| matches!(e.error_type, ValidationErrorType::EmptyUpSql));
        assert!(has_empty_up);
    }

    #[test]
    fn test_migration_timestamp() {
        let version1 = MigrationTimestamp::generate();
        let version2 = MigrationTimestamp::generate();

        assert!(!version1.is_empty());
        assert!(!version2.is_empty());

        // Generated timestamps should be different (assuming some time passed)
        // In practice, they might be the same if generated too quickly
        // So we just test that they're valid numeric strings
        assert!(version1.parse::<u64>().is_ok());
        assert!(version2.parse::<u64>().is_ok());
    }

    #[test]
    fn test_migration_result_display() {
        let migration = Migration::simple(
            "001",
            "test_migration",
            "CREATE TABLE test (id INTEGER)",
            "DROP TABLE test",
        );

        let success_result = MigrationResult {
            migration: migration.clone(),
            success: true,
            error: None,
        };

        let error_result = MigrationResult {
            migration,
            success: false,
            error: Some("Table already exists".to_string()),
        };

        let success_display = success_result.to_string();
        let error_display = error_result.to_string();

        assert!(success_display.contains("✓"));
        assert!(error_display.contains("✗"));
        assert!(error_display.contains("Table already exists"));
    }

    #[test]
    fn test_migration_status_display() {
        let status = MigrationStatus {
            total_migrations: 5,
            applied_count: 3,
            pending_count: 2,
            applied_versions: vec!["001".to_string(), "002".to_string(), "003".to_string()],
            pending_versions: vec!["004".to_string(), "005".to_string()],
        };

        let display = status.to_string();
        assert!(display.contains("Total migrations: 5"));
        assert!(display.contains("Applied: 3"));
        assert!(display.contains("Pending: 2"));
    }
}
