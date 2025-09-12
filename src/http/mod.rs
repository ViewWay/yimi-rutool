//! HTTP client utilities for rutool
//!
//! This module provides comprehensive HTTP client functionality including:
//! - Simple HTTP requests (GET, POST, PUT, DELETE, etc.)
//! - Async and blocking HTTP clients
//! - Request/response handling with headers and cookies
//! - File upload and download
//! - JSON and form data support

pub mod http_util;

/// Re-export commonly used types for convenience
pub use http_util::HttpUtil;
