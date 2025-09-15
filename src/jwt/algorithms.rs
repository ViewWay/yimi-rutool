//! JWT signing algorithms and key management

use crate::jwt::errors::{JwtError, JwtResult};
use serde::{Deserialize, Serialize};

/// Supported JWT signing algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Algorithm {
    /// HMAC using SHA-256
    #[default]
    HS256,
    /// HMAC using SHA-384  
    HS384,
    /// HMAC using SHA-512
    HS512,
    /// RSA using SHA-256
    RS256,
    /// RSA using SHA-384
    RS384,
    /// RSA using SHA-512
    RS512,
    /// ECDSA using SHA-256
    ES256,
    /// ECDSA using SHA-384
    ES384,
    /// ECDSA using SHA-512
    ES512,
}

impl Algorithm {
    /// Get the algorithm name as used in JWT headers
    pub fn as_str(&self) -> &'static str {
        match self {
            Algorithm::HS256 => "HS256",
            Algorithm::HS384 => "HS384",
            Algorithm::HS512 => "HS512",
            Algorithm::RS256 => "RS256",
            Algorithm::RS384 => "RS384",
            Algorithm::RS512 => "RS512",
            Algorithm::ES256 => "ES256",
            Algorithm::ES384 => "ES384",
            Algorithm::ES512 => "ES512",
        }
    }

    /// Parse algorithm from string
    ///
    /// # Errors
    ///
    /// Returns `JwtError::InvalidAlgorithm` if the string is not a valid algorithm name
    pub fn from_str(s: &str) -> JwtResult<Self> {
        match s {
            "HS256" => Ok(Algorithm::HS256),
            "HS384" => Ok(Algorithm::HS384),
            "HS512" => Ok(Algorithm::HS512),
            "RS256" => Ok(Algorithm::RS256),
            "RS384" => Ok(Algorithm::RS384),
            "RS512" => Ok(Algorithm::RS512),
            "ES256" => Ok(Algorithm::ES256),
            "ES384" => Ok(Algorithm::ES384),
            "ES512" => Ok(Algorithm::ES512),
            _ => Err(JwtError::invalid_algorithm(s)),
        }
    }

    /// Check if algorithm uses HMAC (symmetric)
    pub fn is_hmac(&self) -> bool {
        matches!(self, Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512)
    }

    /// Check if algorithm uses RSA (asymmetric)
    pub fn is_rsa(&self) -> bool {
        matches!(self, Algorithm::RS256 | Algorithm::RS384 | Algorithm::RS512)
    }

    /// Check if algorithm uses ECDSA (asymmetric)
    pub fn is_ecdsa(&self) -> bool {
        matches!(self, Algorithm::ES256 | Algorithm::ES384 | Algorithm::ES512)
    }

    /// Check if algorithm is asymmetric
    pub fn is_asymmetric(&self) -> bool {
        self.is_rsa() || self.is_ecdsa()
    }
}

impl std::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Signing key for JWT operations
#[derive(Debug, Clone)]
pub enum SigningKey {
    /// HMAC secret key (for HS* algorithms)
    Hmac(Vec<u8>),
    /// RSA private key in PEM format (for RS* algorithms)
    RsaPrivate(String),
    /// RSA public key in PEM format (for verification)
    RsaPublic(String),
    /// ECDSA private key in PEM format (for ES* algorithms)
    EcdsaPrivate(String),
    /// ECDSA public key in PEM format (for verification)  
    EcdsaPublic(String),
}

impl SigningKey {
    /// Create HMAC key from string
    pub fn hmac_from_string(secret: impl Into<String>) -> Self {
        Self::Hmac(secret.into().into_bytes())
    }

    /// Create HMAC key from bytes
    pub fn hmac_from_bytes(secret: Vec<u8>) -> Self {
        Self::Hmac(secret)
    }

    /// Create RSA private key from PEM string
    pub fn rsa_private_from_pem(pem: impl Into<String>) -> Self {
        Self::RsaPrivate(pem.into())
    }

    /// Create RSA public key from PEM string
    pub fn rsa_public_from_pem(pem: impl Into<String>) -> Self {
        Self::RsaPublic(pem.into())
    }

    /// Create ECDSA private key from PEM string
    pub fn ecdsa_private_from_pem(pem: impl Into<String>) -> Self {
        Self::EcdsaPrivate(pem.into())
    }

    /// Create ECDSA public key from PEM string
    pub fn ecdsa_public_from_pem(pem: impl Into<String>) -> Self {
        Self::EcdsaPublic(pem.into())
    }

    /// Check if key is compatible with algorithm
    pub fn is_compatible_with(&self, algorithm: Algorithm) -> bool {
        match (self, algorithm) {
            (SigningKey::Hmac(_), alg) if alg.is_hmac() => true,
            (SigningKey::RsaPrivate(_) | SigningKey::RsaPublic(_), alg) if alg.is_rsa() => true,
            (SigningKey::EcdsaPrivate(_) | SigningKey::EcdsaPublic(_), alg) if alg.is_ecdsa() => {
                true
            }
            _ => false,
        }
    }

    /// Check if key can be used for signing (private keys)
    pub fn can_sign(&self) -> bool {
        matches!(
            self,
            SigningKey::Hmac(_) | SigningKey::RsaPrivate(_) | SigningKey::EcdsaPrivate(_)
        )
    }

    /// Check if key can be used for verification
    pub fn can_verify(&self) -> bool {
        // All keys can verify, but private keys are typically used for signing
        true
    }
}

#[cfg(feature = "jsonwebtoken")]
impl TryFrom<Algorithm> for jsonwebtoken::Algorithm {
    type Error = JwtError;

    fn try_from(alg: Algorithm) -> Result<Self, Self::Error> {
        match alg {
            Algorithm::HS256 => Ok(jsonwebtoken::Algorithm::HS256),
            Algorithm::HS384 => Ok(jsonwebtoken::Algorithm::HS384),
            Algorithm::HS512 => Ok(jsonwebtoken::Algorithm::HS512),
            Algorithm::RS256 => Ok(jsonwebtoken::Algorithm::RS256),
            Algorithm::RS384 => Ok(jsonwebtoken::Algorithm::RS384),
            Algorithm::RS512 => Ok(jsonwebtoken::Algorithm::RS512),
            Algorithm::ES256 => Ok(jsonwebtoken::Algorithm::ES256),
            Algorithm::ES384 => Ok(jsonwebtoken::Algorithm::ES384),
            // ES512 not supported by jsonwebtoken crate
            Algorithm::ES512 => Err(JwtError::invalid_algorithm(
                "ES512 not supported by jsonwebtoken",
            )),
        }
    }
}

#[cfg(feature = "jsonwebtoken")]
impl TryFrom<jsonwebtoken::Algorithm> for Algorithm {
    type Error = JwtError;

    fn try_from(alg: jsonwebtoken::Algorithm) -> Result<Self, Self::Error> {
        match alg {
            jsonwebtoken::Algorithm::HS256 => Ok(Algorithm::HS256),
            jsonwebtoken::Algorithm::HS384 => Ok(Algorithm::HS384),
            jsonwebtoken::Algorithm::HS512 => Ok(Algorithm::HS512),
            jsonwebtoken::Algorithm::RS256 => Ok(Algorithm::RS256),
            jsonwebtoken::Algorithm::RS384 => Ok(Algorithm::RS384),
            jsonwebtoken::Algorithm::RS512 => Ok(Algorithm::RS512),
            jsonwebtoken::Algorithm::ES256 => Ok(Algorithm::ES256),
            jsonwebtoken::Algorithm::ES384 => Ok(Algorithm::ES384),
            // ES512 not supported by jsonwebtoken crate
            // jsonwebtoken::Algorithm::ES512 => Ok(Algorithm::ES512),
            _ => Err(JwtError::invalid_algorithm("Unsupported algorithm")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algorithm_string_conversion() {
        assert_eq!(Algorithm::HS256.as_str(), "HS256");
        assert_eq!(Algorithm::RS256.as_str(), "RS256");
        assert_eq!(Algorithm::ES256.as_str(), "ES256");

        assert_eq!(Algorithm::from_str("HS256").unwrap(), Algorithm::HS256);
        assert_eq!(Algorithm::from_str("RS256").unwrap(), Algorithm::RS256);
        assert_eq!(Algorithm::from_str("ES256").unwrap(), Algorithm::ES256);

        assert!(Algorithm::from_str("INVALID").is_err());
    }

    #[test]
    fn test_algorithm_types() {
        assert!(Algorithm::HS256.is_hmac());
        assert!(!Algorithm::HS256.is_rsa());
        assert!(!Algorithm::HS256.is_ecdsa());
        assert!(!Algorithm::HS256.is_asymmetric());

        assert!(!Algorithm::RS256.is_hmac());
        assert!(Algorithm::RS256.is_rsa());
        assert!(!Algorithm::RS256.is_ecdsa());
        assert!(Algorithm::RS256.is_asymmetric());

        assert!(!Algorithm::ES256.is_hmac());
        assert!(!Algorithm::ES256.is_rsa());
        assert!(Algorithm::ES256.is_ecdsa());
        assert!(Algorithm::ES256.is_asymmetric());
    }

    #[test]
    fn test_signing_key_compatibility() {
        let hmac_key = SigningKey::hmac_from_string("secret");
        let rsa_private = SigningKey::rsa_private_from_pem("-----BEGIN PRIVATE KEY-----");
        let rsa_public = SigningKey::rsa_public_from_pem("-----BEGIN PUBLIC KEY-----");

        assert!(hmac_key.is_compatible_with(Algorithm::HS256));
        assert!(!hmac_key.is_compatible_with(Algorithm::RS256));

        assert!(rsa_private.is_compatible_with(Algorithm::RS256));
        assert!(!rsa_private.is_compatible_with(Algorithm::HS256));

        assert!(rsa_public.is_compatible_with(Algorithm::RS256));
        assert!(!rsa_public.is_compatible_with(Algorithm::ES256));
    }

    #[test]
    fn test_signing_capabilities() {
        let hmac_key = SigningKey::hmac_from_string("secret");
        let rsa_private = SigningKey::rsa_private_from_pem("private");
        let rsa_public = SigningKey::rsa_public_from_pem("public");

        assert!(hmac_key.can_sign());
        assert!(hmac_key.can_verify());

        assert!(rsa_private.can_sign());
        assert!(rsa_private.can_verify());

        assert!(!rsa_public.can_sign());
        assert!(rsa_public.can_verify());
    }
}
