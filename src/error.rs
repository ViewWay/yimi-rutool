//! Error types for the rutool library

/// Result type alias for rutool operations
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type for rutool operations
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// IO operation errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// UTF-8 conversion errors
    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),

    /// String conversion errors
    #[error("String conversion error: {0}")]
    FromUtf8(#[from] std::string::FromUtf8Error),

    /// JSON parsing/serialization errors
    #[cfg(feature = "json")]
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// HTTP client errors
    #[cfg(feature = "http")]
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// Cryptography errors
    #[cfg(feature = "crypto")]
    #[error("Crypto error: {0}")]
    Crypto(String),

    /// Database errors
    #[cfg(feature = "db")]
    #[error("Database error: {0}")]
    Database(String),

    /// Date/time parsing errors
    #[cfg(feature = "core")]
    #[error("Date/time error: {0}")]
    DateTime(String),

    /// Regex compilation errors
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    /// Generic error with custom message
    #[error("Rutool error: {0}")]
    Custom(String),

    /// Conversion errors
    #[error("Conversion error: {0}")]
    Conversion(String),

    /// Validation errors
    #[error("Validation error: {0}")]
    Validation(String),

    /// Not found errors
    #[error("Not found: {0}")]
    NotFound(String),

    /// Permission denied errors
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// Timeout errors
    #[error("Timeout error: {0}")]
    Timeout(String),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// Concurrency errors (lock poisoning, etc.)
    #[error("Concurrency error: {0}")]
    Concurrency(String),
}

impl Error {
    /// Create a new custom error
    pub fn custom<S: Into<String>>(message: S) -> Self {
        Self::Custom(message.into())
    }

    /// Create a new crypto error
    #[cfg(feature = "crypto")]
    pub fn crypto<S: Into<String>>(message: S) -> Self {
        Self::Crypto(message.into())
    }

    /// Create a new database error
    #[cfg(feature = "db")]
    pub fn database<S: Into<String>>(message: S) -> Self {
        Self::Database(message.into())
    }

    /// Create a new date/time error
    #[cfg(feature = "core")]
    pub fn datetime<S: Into<String>>(message: S) -> Self {
        Self::DateTime(message.into())
    }

    /// Create a new conversion error
    pub fn conversion<S: Into<String>>(message: S) -> Self {
        Self::Conversion(message.into())
    }

    /// Create a new validation error
    pub fn validation<S: Into<String>>(message: S) -> Self {
        Self::Validation(message.into())
    }

    /// Create a new not found error
    pub fn not_found<S: Into<String>>(message: S) -> Self {
        Self::NotFound(message.into())
    }

    /// Create a new permission denied error
    pub fn permission_denied<S: Into<String>>(message: S) -> Self {
        Self::PermissionDenied(message.into())
    }

    /// Create a new timeout error
    pub fn timeout<S: Into<String>>(message: S) -> Self {
        Self::Timeout(message.into())
    }

    /// Create a new configuration error
    pub fn config<S: Into<String>>(message: S) -> Self {
        Self::Config(message.into())
    }

    /// Create a new concurrency error
    pub fn concurrency<S: Into<String>>(message: S) -> Self {
        Self::Concurrency(message.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = Error::custom("test error");
        assert!(matches!(err, Error::Custom(_)));
    }

    #[test]
    fn test_error_display() {
        let err = Error::validation("invalid input");
        let msg = err.to_string();
        assert!(msg.contains("Validation error"));
        assert!(msg.contains("invalid input"));
    }
}
