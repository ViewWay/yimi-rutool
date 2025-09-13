//! JWT utility functions for token creation and validation

use crate::jwt::{Algorithm, Claims, JwtError, JwtResult, SigningKey};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use serde::{Deserialize, Serialize};

/// JWT header structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtHeader {
    /// Algorithm used for signing
    pub alg: String,
    /// Token type (always "JWT")
    pub typ: String,
    /// Key ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
}

impl JwtHeader {
    /// Create new JWT header
    pub fn new(algorithm: Algorithm) -> Self {
        Self {
            alg: algorithm.as_str().to_string(),
            typ: "JWT".to_string(),
            kid: None,
        }
    }
    
    /// Create header with key ID
    pub fn with_key_id(mut self, kid: impl Into<String>) -> Self {
        self.kid = Some(kid.into());
        self
    }
}

impl Default for JwtHeader {
    fn default() -> Self {
        Self::new(Algorithm::HS256)
    }
}

/// Main JWT utility struct
pub struct JwtUtil;

impl JwtUtil {
    /// Create a JWT token with HMAC signature
    pub fn create_token(claims: &Claims, secret: &str) -> JwtResult<String> {
        Self::create_token_with_algorithm(claims, secret, Algorithm::HS256)
    }
    
    /// Create a JWT token with specified algorithm and HMAC secret
    pub fn create_token_with_algorithm(claims: &Claims, secret: &str, algorithm: Algorithm) -> JwtResult<String> {
        if !algorithm.is_hmac() {
            return Err(JwtError::invalid_algorithm("Only HMAC algorithms supported with string secret"));
        }
        
        let key = SigningKey::hmac_from_string(secret);
        Self::create_token_with_key(claims, &key, algorithm)
    }
    
    /// Create a JWT token with signing key
    pub fn create_token_with_key(claims: &Claims, key: &SigningKey, algorithm: Algorithm) -> JwtResult<String> {
        if !key.is_compatible_with(algorithm) {
            return Err(JwtError::invalid_key("Key not compatible with algorithm"));
        }
        
        if !key.can_sign() {
            return Err(JwtError::invalid_key("Key cannot be used for signing"));
        }
        
        #[cfg(feature = "jsonwebtoken")]
        {
            Self::create_token_with_jsonwebtoken(claims, key, algorithm)
        }
        
        #[cfg(not(feature = "jsonwebtoken"))]
        {
            Self::create_token_manual(claims, key, algorithm)
        }
    }
    
    #[cfg(feature = "jsonwebtoken")]
    fn create_token_with_jsonwebtoken(claims: &Claims, key: &SigningKey, algorithm: Algorithm) -> JwtResult<String> {
        use jsonwebtoken::{encode, Header, EncodingKey};
        
        let header = Header::new(algorithm.try_into()?);
        let encoding_key = match key {
            SigningKey::Hmac(secret) => EncodingKey::from_secret(secret),
            SigningKey::RsaPrivate(pem) => EncodingKey::from_rsa_pem(pem.as_bytes())?,
            SigningKey::EcdsaPrivate(pem) => EncodingKey::from_ec_pem(pem.as_bytes())?,
            _ => return Err(JwtError::invalid_key("Invalid key type for signing")),
        };
        
        encode(&header, claims, &encoding_key).map_err(JwtError::from)
    }
    
    #[cfg(not(feature = "jsonwebtoken"))]
    fn create_token_manual(claims: &Claims, key: &SigningKey, algorithm: Algorithm) -> JwtResult<String> {
        // Manual implementation for when jsonwebtoken is not available
        let header = JwtHeader::new(algorithm);
        
        // Encode header
        let header_json = serde_json::to_string(&header)?;
        let header_b64 = URL_SAFE_NO_PAD.encode(header_json.as_bytes());
        
        // Encode payload
        let payload_json = serde_json::to_string(claims)?;
        let payload_b64 = URL_SAFE_NO_PAD.encode(payload_json.as_bytes());
        
        // Create signing input
        let signing_input = format!("{}.{}", header_b64, payload_b64);
        
        // Sign
        let signature = Self::sign(&signing_input, key, algorithm)?;
        let signature_b64 = URL_SAFE_NO_PAD.encode(&signature);
        
        Ok(format!("{}.{}", signing_input, signature_b64))
    }
    
    /// Validate a JWT token with HMAC secret
    pub fn validate_token(token: &str, secret: &str) -> JwtResult<Claims> {
        Self::validate_token_with_algorithm(token, secret, Algorithm::HS256)
    }
    
    /// Validate a JWT token with specified algorithm and HMAC secret
    pub fn validate_token_with_algorithm(token: &str, secret: &str, algorithm: Algorithm) -> JwtResult<Claims> {
        if !algorithm.is_hmac() {
            return Err(JwtError::invalid_algorithm("Only HMAC algorithms supported with string secret"));
        }
        
        let key = SigningKey::hmac_from_string(secret);
        Self::validate_token_with_key(token, &key, algorithm)
    }
    
    /// Validate a JWT token with signing key
    pub fn validate_token_with_key(token: &str, key: &SigningKey, algorithm: Algorithm) -> JwtResult<Claims> {
        if !key.is_compatible_with(algorithm) {
            return Err(JwtError::invalid_key("Key not compatible with algorithm"));
        }
        
        #[cfg(feature = "jsonwebtoken")]
        {
            Self::validate_token_with_jsonwebtoken(token, key, algorithm)
        }
        
        #[cfg(not(feature = "jsonwebtoken"))]
        {
            Self::validate_token_manual(token, key, algorithm)
        }
    }
    
    #[cfg(feature = "jsonwebtoken")]
    fn validate_token_with_jsonwebtoken(token: &str, key: &SigningKey, algorithm: Algorithm) -> JwtResult<Claims> {
        use jsonwebtoken::{decode, DecodingKey, Validation};
        
        let mut validation = Validation::new(algorithm.try_into()?);
        validation.validate_exp = false; // We'll validate manually for better error messages
        validation.validate_nbf = false;
        validation.required_spec_claims.clear(); // Don't require any specific claims
        
        let decoding_key = match key {
            SigningKey::Hmac(secret) => DecodingKey::from_secret(secret),
            SigningKey::RsaPrivate(pem) | SigningKey::RsaPublic(pem) => {
                DecodingKey::from_rsa_pem(pem.as_bytes())?
            },
            SigningKey::EcdsaPrivate(pem) | SigningKey::EcdsaPublic(pem) => {
                DecodingKey::from_ec_pem(pem.as_bytes())?
            },
        };
        
        let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
        let claims = token_data.claims;
        
        // Validate timing manually for better error handling
        claims.validate_time()?;
        
        Ok(claims)
    }
    
    #[cfg(not(feature = "jsonwebtoken"))]
    fn validate_token_manual(token: &str, key: &SigningKey, algorithm: Algorithm) -> JwtResult<Claims> {
        // Parse token parts
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(JwtError::invalid_token("Token must have 3 parts"));
        }
        
        let header_b64 = parts[0];
        let payload_b64 = parts[1];
        let signature_b64 = parts[2];
        
        // Decode and verify header
        let header_bytes = URL_SAFE_NO_PAD.decode(header_b64)?;
        let header: JwtHeader = serde_json::from_slice(&header_bytes)?;
        
        if header.alg != algorithm.as_str() {
            return Err(JwtError::invalid_algorithm(format!("Expected {}, got {}", algorithm.as_str(), header.alg)));
        }
        
        // Verify signature
        let signing_input = format!("{}.{}", header_b64, payload_b64);
        let signature = URL_SAFE_NO_PAD.decode(signature_b64)?;
        
        if !Self::verify(&signing_input, &signature, key, algorithm)? {
            return Err(JwtError::InvalidSignature);
        }
        
        // Decode payload
        let payload_bytes = URL_SAFE_NO_PAD.decode(payload_b64)?;
        let claims: Claims = serde_json::from_slice(&payload_bytes)?;
        
        // Validate timing
        claims.validate_time()?;
        
        Ok(claims)
    }
    
    /// Decode token without verification (for inspection)
    pub fn decode_without_verification(token: &str) -> JwtResult<(JwtHeader, Claims)> {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(JwtError::invalid_token("Token must have 3 parts"));
        }
        
        // Decode header
        let header_bytes = URL_SAFE_NO_PAD.decode(parts[0])?;
        let header: JwtHeader = serde_json::from_slice(&header_bytes)?;
        
        // Decode payload
        let payload_bytes = URL_SAFE_NO_PAD.decode(parts[1])?;
        let claims: Claims = serde_json::from_slice(&payload_bytes)?;
        
        Ok((header, claims))
    }
    
    /// Get token expiration time without verification
    pub fn get_expiration(token: &str) -> JwtResult<Option<i64>> {
        let (_, claims) = Self::decode_without_verification(token)?;
        Ok(claims.expires_at)
    }
    
    /// Check if token is expired without verification
    pub fn is_expired(token: &str) -> JwtResult<bool> {
        let (_, claims) = Self::decode_without_verification(token)?;
        Ok(claims.is_expired())
    }
    
    /// Get token subject without verification
    pub fn get_subject(token: &str) -> JwtResult<Option<String>> {
        let (_, claims) = Self::decode_without_verification(token)?;
        Ok(claims.subject)
    }
    
    /// Create a refresh token (long-lived token with minimal claims)
    pub fn create_refresh_token(user_id: &str, secret: &str, duration_hours: i64) -> JwtResult<String> {
        let claims = Claims::new()
            .with_subject(user_id)
            .with_issued_at_now()
            .with_expiration_from_now(chrono::Duration::hours(duration_hours))
            .with_custom_string("type", "refresh");
            
        Self::create_token(&claims, secret)
    }
    
    /// Validate refresh token and extract user ID
    pub fn validate_refresh_token(token: &str, secret: &str) -> JwtResult<String> {
        let claims = Self::validate_token(token, secret)?;
        
        // Check if it's a refresh token
        if claims.get_custom_string("type") != Some("refresh") {
            return Err(JwtError::invalid_claim("type", "not a refresh token"));
        }
        
        // Extract user ID
        claims.subject.ok_or_else(|| JwtError::missing_claim("sub"))
    }
    
    #[cfg(not(feature = "jsonwebtoken"))]
    fn sign(data: &str, key: &SigningKey, algorithm: Algorithm) -> JwtResult<Vec<u8>> {
        match (key, algorithm) {
            (SigningKey::Hmac(secret), Algorithm::HS256) => {
                use hmac::{Hmac, Mac};
                use sha2::Sha256;
                
                let mut mac = Hmac::<Sha256>::new_from_slice(secret)
                    .map_err(|_| JwtError::invalid_key("Invalid HMAC key"))?;
                mac.update(data.as_bytes());
                Ok(mac.finalize().into_bytes().to_vec())
            },
            (SigningKey::Hmac(secret), Algorithm::HS384) => {
                use hmac::{Hmac, Mac};
                use sha2::Sha384;
                
                let mut mac = Hmac::<Sha384>::new_from_slice(secret)
                    .map_err(|_| JwtError::invalid_key("Invalid HMAC key"))?;
                mac.update(data.as_bytes());
                Ok(mac.finalize().into_bytes().to_vec())
            },
            (SigningKey::Hmac(secret), Algorithm::HS512) => {
                use hmac::{Hmac, Mac};
                use sha2::Sha512;
                
                let mut mac = Hmac::<Sha512>::new_from_slice(secret)
                    .map_err(|_| JwtError::invalid_key("Invalid HMAC key"))?;
                mac.update(data.as_bytes());
                Ok(mac.finalize().into_bytes().to_vec())
            },
            _ => Err(JwtError::invalid_algorithm("Algorithm not supported in manual mode")),
        }
    }
    
    #[cfg(not(feature = "jsonwebtoken"))]
    fn verify(data: &str, signature: &[u8], key: &SigningKey, algorithm: Algorithm) -> JwtResult<bool> {
        let expected_signature = Self::sign(data, key, algorithm)?;
        Ok(signature == expected_signature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_jwt_header() {
        let header = JwtHeader::new(Algorithm::HS256);
        assert_eq!(header.alg, "HS256");
        assert_eq!(header.typ, "JWT");
        assert!(header.kid.is_none());
        
        let header = JwtHeader::new(Algorithm::RS256).with_key_id("key1");
        assert_eq!(header.alg, "RS256");
        assert_eq!(header.kid, Some("key1".to_string()));
    }
    
    #[test]
    fn test_create_and_validate_token() {
        let secret = "test-secret-key";
        let claims = Claims::new()
            .with_subject("user123")
            .with_issued_at_now()
            .with_expiration_from_now(chrono::Duration::hours(1))
            .with_custom_string("role", "admin");
        
        // Create token
        let token = JwtUtil::create_token(&claims, secret).unwrap();
        assert!(!token.is_empty());
        assert_eq!(token.matches('.').count(), 2); // Should have 3 parts
        
        // Validate token
        let decoded_claims = JwtUtil::validate_token(&token, secret).unwrap();
        assert_eq!(decoded_claims.subject, Some("user123".to_string()));
        assert_eq!(decoded_claims.get_custom_string("role"), Some("admin"));
    }
    
    #[test]
    fn test_expired_token() {
        let secret = "test-secret";
        let claims = Claims::new()
            .with_subject("user123")
            .with_expiration(chrono::Utc::now().timestamp() - 100); // Expired 100 seconds ago
        
        let token = JwtUtil::create_token(&claims, secret).unwrap();
        let result = JwtUtil::validate_token(&token, secret);
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), JwtError::TokenExpired));
    }
    
    #[test]
    fn test_invalid_signature() {
        let secret1 = "secret1";
        let secret2 = "secret2";
        
        let claims = Claims::new().with_subject("user123");
        let token = JwtUtil::create_token(&claims, secret1).unwrap();
        
        // Try to validate with different secret
        let result = JwtUtil::validate_token(&token, secret2);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_decode_without_verification() {
        let secret = "test-secret";
        let claims = Claims::new()
            .with_subject("user123")
            .with_custom_string("role", "admin");
        
        let token = JwtUtil::create_token(&claims, secret).unwrap();
        let (header, decoded_claims) = JwtUtil::decode_without_verification(&token).unwrap();
        
        assert_eq!(header.alg, "HS256");
        assert_eq!(decoded_claims.subject, Some("user123".to_string()));
        assert_eq!(decoded_claims.get_custom_string("role"), Some("admin"));
    }
    
    #[test]
    fn test_refresh_token() {
        let secret = "refresh-secret";
        let user_id = "user456";
        
        // Create refresh token
        let token = JwtUtil::create_refresh_token(user_id, secret, 24).unwrap();
        
        // Validate refresh token
        let extracted_user_id = JwtUtil::validate_refresh_token(&token, secret).unwrap();
        assert_eq!(extracted_user_id, user_id);
        
        // Regular validation should also work
        let claims = JwtUtil::validate_token(&token, secret).unwrap();
        assert_eq!(claims.subject, Some(user_id.to_string()));
        assert_eq!(claims.get_custom_string("type"), Some("refresh"));
    }
    
    #[test]
    fn test_token_inspection() {
        let secret = "inspect-secret";
        let claims = Claims::new()
            .with_subject("user789")
            .with_expiration_from_now(chrono::Duration::hours(2));
        
        let token = JwtUtil::create_token(&claims, secret).unwrap();
        
        // Test various inspection methods
        let subject = JwtUtil::get_subject(&token).unwrap();
        assert_eq!(subject, Some("user789".to_string()));
        
        let expiration = JwtUtil::get_expiration(&token).unwrap();
        assert!(expiration.is_some());
        
        let is_expired = JwtUtil::is_expired(&token).unwrap();
        assert!(!is_expired);
    }
}
