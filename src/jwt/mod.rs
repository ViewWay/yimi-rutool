//! JWT (JSON Web Token) utilities for yimi-rutool
//!
//! This module provides functionality for creating, validating, and managing JWT tokens.
//! It supports various signing algorithms including HMAC, RSA, and ECDSA.
//!
//! # Features
//!
//! - JWT token creation and validation
//! - Multiple signing algorithms (HS256, HS384, HS512, RS256, RS384, RS512, ES256, ES384, ES512)
//! - Claims management with standard and custom claims
//! - Token expiration and validation
//! - Base64 encoding/decoding utilities
//!
//! # Quick Start
//!
//! ```rust
//! use yimi_rutool::jwt::{JwtUtil, Claims};
//! use std::collections::HashMap;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a simple JWT token
//! let mut claims = Claims::new();
//! claims.subject = Some("user123".to_string());
//! claims.expires_at = Some(chrono::Utc::now().timestamp() + 3600); // 1 hour
//!
//! let secret = "your-secret-key";
//! let token = JwtUtil::create_token(&claims, secret)?;
//!
//! // Validate the token
//! let decoded_claims = JwtUtil::validate_token(&token, secret)?;
//! println!("Subject: {:?}", decoded_claims.subject);
//! # Ok(())
//! # }
//! ```

pub mod jwt_util;
pub mod claims;
pub mod algorithms;
pub mod errors;

// Re-export main types for convenience
pub use jwt_util::JwtUtil;
pub use claims::{Claims, ClaimsBuilder};
pub use algorithms::{Algorithm, SigningKey};
pub use errors::{JwtError, JwtResult};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_module_imports() {
        // Basic test to ensure all imports work
        let _claims = Claims::new();
        let _algorithm = Algorithm::HS256;
    }
}
