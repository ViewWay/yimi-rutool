// Allow some less critical clippy lints for better development experience
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::module_name_repetitions)]

//! # yimi-rutool - A Comprehensive Rust Utility Library
//!
//! yimi-rutool is a comprehensive Rust utility library inspired by Hutool,
//! providing a rich set of tools for everyday development tasks.
//!
//! ## Features
//!
//! - **Core utilities**: String manipulation, date/time handling, type conversion
//! - **Cryptography**: Symmetric/asymmetric encryption, hashing, digital signatures
//! - **HTTP client**: Easy-to-use HTTP client with async support
//! - **JSON processing**: Fast JSON serialization/deserialization
//! - **Database**: Database operations and connection management
//! - **Caching**: In-memory and persistent caching solutions
//! - **Scheduling**: Cron-based task scheduling
//! - **Extra tools**: QR code generation, image processing, compression
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! rutool = "0.1"
//! ```
//!
//! ## Example
//!
//! ```rust
//! use yimi_rutool::core::{StrUtil, DateUtil};
//!
//! // String utilities
//! let result = StrUtil::is_blank("   ");
//! assert_eq!(result, true);
//!
//! // Date utilities
//! let now = DateUtil::now();
//! println!("Current time: {}", now);
//! ```
//!
//! ## Feature Flags
//!
//! - `core`: Core utility functions (enabled by default)
//! - `crypto`: Cryptography functions
//! - `http`: HTTP client functionality
//! - `json`: JSON processing
//! - `cache`: Caching functionality
//! - `db`: Database operations
//! - `cron`: Task scheduling
//! - `extra`: Additional utilities
//! - `full`: Enable all features (default)
//!
//! ## License
//!
//! This project is licensed under MIT OR Apache-2.0.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs, clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::must_use_candidate)]

/// Core utility modules
#[cfg(feature = "core")]
pub mod core;

/// Cryptography utilities
#[cfg(feature = "crypto")]
pub mod crypto;

/// HTTP client utilities
#[cfg(feature = "http")]
pub mod http;

/// JSON processing utilities
#[cfg(feature = "json")]
pub mod json;

/// Database utilities
#[cfg(feature = "db")]
pub mod db;

/// Caching utilities
#[cfg(feature = "cache")]
pub mod cache;

/// Cron scheduling utilities
#[cfg(feature = "cron")]
pub mod cron;

/// Extra utilities (QR codes, images, compression, etc.)
#[cfg(feature = "extra")]
pub mod extra;

/// JWT (JSON Web Token) utilities
#[cfg(feature = "jwt")]
pub mod jwt;

/// Error types used throughout the library
pub mod error;

/// Re-export commonly used types for convenience
pub use error::{Error, Result};

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
