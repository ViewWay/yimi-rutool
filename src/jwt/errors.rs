//! JWT error types and result handling

use thiserror::Error;

/// JWT operation result type
pub type JwtResult<T> = Result<T, JwtError>;

/// JWT-specific error types
#[derive(Error, Debug)]
pub enum JwtError {
    /// Invalid token format or structure
    #[error("Invalid token format: {0}")]
    InvalidToken(String),

    /// Token has expired
    #[error("Token has expired")]
    TokenExpired,

    /// Token is not yet valid (nbf claim)
    #[error("Token is not yet valid")]
    TokenNotYetValid,

    /// Invalid signature
    #[error("Invalid token signature")]
    InvalidSignature,

    /// Invalid algorithm
    #[error("Invalid algorithm: {0}")]
    InvalidAlgorithm(String),

    /// Invalid key format
    #[error("Invalid key format: {0}")]
    InvalidKey(String),

    /// Missing required claim
    #[error("Missing required claim: {0}")]
    MissingClaim(String),

    /// Invalid claim value
    #[error("Invalid claim value for '{0}': {1}")]
    InvalidClaim(String, String),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Base64 decoding error
    #[error("Base64 decode error: {0}")]
    Base64Error(#[from] base64::DecodeError),

    /// Cryptographic error
    #[cfg(feature = "jsonwebtoken")]
    #[error("JWT library error: {0}")]
    JwtLibError(#[from] jsonwebtoken::errors::Error),

    /// Generic error
    #[error("JWT error: {0}")]
    Other(String),
}

impl JwtError {
    /// Create a new invalid token error
    pub fn invalid_token(msg: impl Into<String>) -> Self {
        Self::InvalidToken(msg.into())
    }

    /// Create a new invalid algorithm error
    pub fn invalid_algorithm(alg: impl Into<String>) -> Self {
        Self::InvalidAlgorithm(alg.into())
    }

    /// Create a new invalid key error
    pub fn invalid_key(msg: impl Into<String>) -> Self {
        Self::InvalidKey(msg.into())
    }

    /// Create a new missing claim error
    pub fn missing_claim(claim: impl Into<String>) -> Self {
        Self::MissingClaim(claim.into())
    }

    /// Create a new invalid claim error
    pub fn invalid_claim(claim: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::InvalidClaim(claim.into(), reason.into())
    }

    /// Create a generic error
    pub fn other(msg: impl Into<String>) -> Self {
        Self::Other(msg.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = JwtError::invalid_token("malformed");
        assert!(err.to_string().contains("malformed"));

        let err = JwtError::missing_claim("sub");
        assert!(err.to_string().contains("sub"));

        let err = JwtError::invalid_claim("exp", "not a number");
        assert!(err.to_string().contains("exp"));
        assert!(err.to_string().contains("not a number"));
    }

    #[test]
    fn test_error_display() {
        assert_eq!(JwtError::TokenExpired.to_string(), "Token has expired");
        assert_eq!(
            JwtError::TokenNotYetValid.to_string(),
            "Token is not yet valid"
        );
        assert_eq!(
            JwtError::InvalidSignature.to_string(),
            "Invalid token signature"
        );
    }
}
