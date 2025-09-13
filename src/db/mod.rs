//! Database utilities for rutool
//!
//! This module provides comprehensive database functionality including:
//! - Connection management for SQLite, PostgreSQL, and MySQL
//! - SQL query builder and executor
//! - Database migrations
//! - Connection pooling
//! - Transaction management

pub mod connection;
pub mod query_builder;
pub mod migration;

/// Re-export commonly used types for convenience
pub use connection::{DatabaseConnection, ConnectionPool, DatabaseConfig, DatabaseType};
pub use query_builder::QueryBuilder;
pub use migration::{Migration, MigrationRunner, MigrationTimestamp};
