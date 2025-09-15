//! JWT claims management and validation

use crate::jwt::errors::{JwtError, JwtResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Standard JWT claims according to RFC 7519
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Issuer (iss) - identifies the principal that issued the JWT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,

    /// Subject (sub) - identifies the principal that is the subject of the JWT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,

    /// Audience (aud) - identifies the recipients that the JWT is intended for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,

    /// Expiration Time (exp) - identifies the expiration time on or after which the JWT MUST NOT be accepted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,

    /// Not Before (nbf) - identifies the time before which the JWT MUST NOT be accepted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<i64>,

    /// Issued At (iat) - identifies the time at which the JWT was issued
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<i64>,

    /// JWT ID (jti) - provides a unique identifier for the JWT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_id: Option<String>,

    /// Custom claims
    #[serde(flatten)]
    pub custom: HashMap<String, serde_json::Value>,
}

impl Claims {
    /// Create new empty claims
    pub fn new() -> Self {
        Self {
            issuer: None,
            subject: None,
            audience: None,
            expires_at: None,
            not_before: None,
            issued_at: None,
            jwt_id: None,
            custom: HashMap::new(),
        }
    }

    /// Set issuer claim
    #[must_use]
    pub fn with_issuer(mut self, issuer: impl Into<String>) -> Self {
        self.issuer = Some(issuer.into());
        self
    }

    /// Set subject claim
    #[must_use]
    pub fn with_subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    /// Set audience claim
    #[must_use]
    pub fn with_audience(mut self, audience: impl Into<String>) -> Self {
        self.audience = Some(audience.into());
        self
    }

    /// Set expiration time (Unix timestamp)
    #[must_use]
    pub fn with_expiration(mut self, exp: i64) -> Self {
        self.expires_at = Some(exp);
        self
    }

    /// Set expiration time from duration
    #[cfg(feature = "chrono")]
    #[must_use]
    pub fn with_expiration_from_now(mut self, duration: chrono::Duration) -> Self {
        let exp = chrono::Utc::now() + duration;
        self.expires_at = Some(exp.timestamp());
        self
    }

    /// Set not before time (Unix timestamp)
    #[must_use]
    pub fn with_not_before(mut self, nbf: i64) -> Self {
        self.not_before = Some(nbf);
        self
    }

    /// Set issued at time (Unix timestamp)
    #[must_use]
    pub fn with_issued_at(mut self, iat: i64) -> Self {
        self.issued_at = Some(iat);
        self
    }

    /// Set issued at time to now
    #[cfg(feature = "chrono")]
    #[must_use]
    pub fn with_issued_at_now(mut self) -> Self {
        self.issued_at = Some(chrono::Utc::now().timestamp());
        self
    }

    /// Set JWT ID
    #[must_use]
    pub fn with_jwt_id(mut self, jti: impl Into<String>) -> Self {
        self.jwt_id = Some(jti.into());
        self
    }

    /// Add custom claim
    #[must_use]
    pub fn with_custom_claim(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.custom.insert(key.into(), value);
        self
    }

    /// Add custom string claim
    #[must_use]
    pub fn with_custom_string(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.custom
            .insert(key.into(), serde_json::Value::String(value.into()));
        self
    }

    /// Add custom number claim
    #[must_use]
    pub fn with_custom_number(
        mut self,
        key: impl Into<String>,
        value: impl Into<serde_json::Number>,
    ) -> Self {
        self.custom
            .insert(key.into(), serde_json::Value::Number(value.into()));
        self
    }

    /// Add custom boolean claim
    #[must_use]
    pub fn with_custom_bool(mut self, key: impl Into<String>, value: bool) -> Self {
        self.custom
            .insert(key.into(), serde_json::Value::Bool(value));
        self
    }

    /// Get custom claim as string
    pub fn get_custom_string(&self, key: &str) -> Option<&str> {
        self.custom.get(key)?.as_str()
    }

    /// Get custom claim as number
    pub fn get_custom_number(&self, key: &str) -> Option<f64> {
        self.custom.get(key)?.as_f64()
    }

    /// Get custom claim as boolean
    pub fn get_custom_bool(&self, key: &str) -> Option<bool> {
        self.custom.get(key)?.as_bool()
    }

    /// Validate claims for timing constraints
    ///
    /// # Errors
    ///
    /// Returns `JwtError` if:
    /// - Token has expired (`expires_at` is in the past)
    /// - Token is not yet valid (`not_before` is in the future)
    pub fn validate_time(&self) -> JwtResult<()> {
        self.validate_time_with_leeway(0)
    }

    /// Validate claims with time leeway (in seconds)
    ///
    /// # Errors
    ///
    /// Returns `JwtError` if:
    /// - Token has expired (considering leeway)
    /// - Token is not yet valid (considering leeway)
    pub fn validate_time_with_leeway(&self, leeway: i64) -> JwtResult<()> {
        let now = chrono::Utc::now().timestamp();

        // Check expiration
        if let Some(exp) = self.expires_at
            && now > exp + leeway
        {
            return Err(JwtError::TokenExpired);
        }

        // Check not before
        if let Some(nbf) = self.not_before
            && now < nbf - leeway
        {
            return Err(JwtError::TokenNotYetValid);
        }

        Ok(())
    }

    /// Check if token is expired
    pub fn is_expired(&self) -> bool {
        if let Some(exp) = self.expires_at {
            chrono::Utc::now().timestamp() > exp
        } else {
            false
        }
    }

    /// Check if token is not yet valid
    pub fn is_not_yet_valid(&self) -> bool {
        if let Some(nbf) = self.not_before {
            chrono::Utc::now().timestamp() < nbf
        } else {
            false
        }
    }

    /// Get time until expiration in seconds
    pub fn time_until_expiration(&self) -> Option<i64> {
        self.expires_at
            .map(|exp| exp - chrono::Utc::now().timestamp())
    }

    /// Convert claims to JSON string
    ///
    /// # Errors
    ///
    /// Returns `JwtError` if JSON serialization fails
    pub fn to_json(&self) -> JwtResult<String> {
        serde_json::to_string(self).map_err(JwtError::from)
    }

    /// Parse claims from JSON string
    ///
    /// # Errors
    ///
    /// Returns `JwtError` if JSON parsing fails
    pub fn from_json(json: &str) -> JwtResult<Self> {
        serde_json::from_str(json).map_err(JwtError::from)
    }
}

impl Default for Claims {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder pattern for Claims construction
pub struct ClaimsBuilder {
    claims: Claims,
}

impl ClaimsBuilder {
    /// Create new claims builder
    pub fn new() -> Self {
        Self {
            claims: Claims::new(),
        }
    }

    /// Set issuer
    #[must_use]
    pub fn issuer(mut self, issuer: impl Into<String>) -> Self {
        self.claims.issuer = Some(issuer.into());
        self
    }

    /// Set subject
    #[must_use]
    pub fn subject(mut self, subject: impl Into<String>) -> Self {
        self.claims.subject = Some(subject.into());
        self
    }

    /// Set audience
    #[must_use]
    pub fn audience(mut self, audience: impl Into<String>) -> Self {
        self.claims.audience = Some(audience.into());
        self
    }

    /// Set expiration time
    #[must_use]
    pub fn expires_at(mut self, exp: i64) -> Self {
        self.claims.expires_at = Some(exp);
        self
    }

    /// Set expiration from duration
    #[cfg(feature = "chrono")]
    #[must_use]
    pub fn expires_in(mut self, duration: chrono::Duration) -> Self {
        let exp = chrono::Utc::now() + duration;
        self.claims.expires_at = Some(exp.timestamp());
        self
    }

    /// Set not before time
    #[must_use]
    pub fn not_before(mut self, nbf: i64) -> Self {
        self.claims.not_before = Some(nbf);
        self
    }

    /// Set issued at time
    #[must_use]
    pub fn issued_at(mut self, iat: i64) -> Self {
        self.claims.issued_at = Some(iat);
        self
    }

    /// Set issued at to now
    #[cfg(feature = "chrono")]
    #[must_use]
    pub fn issued_now(mut self) -> Self {
        self.claims.issued_at = Some(chrono::Utc::now().timestamp());
        self
    }

    /// Set JWT ID
    #[must_use]
    pub fn jwt_id(mut self, jti: impl Into<String>) -> Self {
        self.claims.jwt_id = Some(jti.into());
        self
    }

    /// Add custom claim
    #[must_use]
    pub fn custom_claim(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.claims.custom.insert(key.into(), value);
        self
    }

    /// Add custom string claim
    #[must_use]
    pub fn custom_string(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.claims
            .custom
            .insert(key.into(), serde_json::Value::String(value.into()));
        self
    }

    /// Build the claims
    pub fn build(self) -> Claims {
        self.claims
    }
}

impl Default for ClaimsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claims_creation() {
        let claims = Claims::new()
            .with_subject("user123")
            .with_issuer("https://example.com")
            .with_audience("api")
            .with_custom_string("role", "admin")
            .with_custom_bool("active", true);

        assert_eq!(claims.subject, Some("user123".to_string()));
        assert_eq!(claims.issuer, Some("https://example.com".to_string()));
        assert_eq!(claims.audience, Some("api".to_string()));
        assert_eq!(claims.get_custom_string("role"), Some("admin"));
        assert_eq!(claims.get_custom_bool("active"), Some(true));
    }

    #[test]
    fn test_claims_builder() {
        let claims = ClaimsBuilder::new()
            .subject("user456")
            .issuer("https://example.com")
            .custom_string("department", "engineering")
            .build();

        assert_eq!(claims.subject, Some("user456".to_string()));
        assert_eq!(claims.issuer, Some("https://example.com".to_string()));
        assert_eq!(claims.get_custom_string("department"), Some("engineering"));
    }

    #[test]
    fn test_time_validation() {
        let now = chrono::Utc::now().timestamp();

        // Valid token
        let claims = Claims::new()
            .with_issued_at(now - 100)
            .with_expiration(now + 3600);
        assert!(claims.validate_time().is_ok());

        // Expired token
        let claims = Claims::new().with_expiration(now - 100);
        assert!(claims.validate_time().is_err());
        assert!(claims.is_expired());

        // Not yet valid token
        let claims = Claims::new().with_not_before(now + 100);
        assert!(claims.validate_time().is_err());
        assert!(claims.is_not_yet_valid());
    }

    #[test]
    fn test_json_serialization() {
        let claims = Claims::new()
            .with_subject("test")
            .with_custom_string("role", "user");

        let json = claims.to_json().unwrap();
        let parsed = Claims::from_json(&json).unwrap();

        assert_eq!(parsed.subject, Some("test".to_string()));
        assert_eq!(parsed.get_custom_string("role"), Some("user"));
    }

    #[test]
    fn test_time_until_expiration() {
        let now = chrono::Utc::now().timestamp();

        let claims = Claims::new().with_expiration(now + 3600);
        let time_left = claims.time_until_expiration().unwrap();
        assert!(time_left > 3500 && time_left <= 3600);

        let claims = Claims::new();
        assert!(claims.time_until_expiration().is_none());
    }
}
