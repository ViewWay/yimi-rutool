//! Database utilities for rutool
//!
//! This module provides comprehensive database functionality including:
//! - Connection management for SQLite, PostgreSQL, and MySQL
//! - SQL query builder and executor
//! - Database migrations
//! - Connection pooling
//! - Transaction management

pub mod connection;
pub mod migration;
pub mod query_builder;

/// Re-export commonly used types for convenience
pub use connection::{ConnectionPool, DatabaseConfig, DatabaseConnection, DatabaseType};
pub use migration::{Migration, MigrationRunner, MigrationTimestamp};
pub use query_builder::QueryBuilder;
