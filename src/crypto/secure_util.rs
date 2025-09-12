//! Security utilities
//!
//! This module provides various security utilities including
//! secure random number generation, password generation, and key generation.

use base64::Engine;
use rand::{RngCore, thread_rng};
use rand::distributions::{Alphanumeric, Distribution};

/// Security utility functions
pub struct SecureUtil;

impl SecureUtil {
    /// Generate secure random bytes
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::crypto::SecureUtil;
    ///
    /// let random_bytes = SecureUtil::random_bytes(32);
    /// assert_eq!(random_bytes.len(), 32);
    /// ```
    pub fn random_bytes(len: usize) -> Vec<u8> {
        let mut bytes = vec![0u8; len];
        thread_rng().fill_bytes(&mut bytes);
        bytes
    }

    /// Generate secure random string with custom charset
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::crypto::SecureUtil;
    ///
    /// let random_str = SecureUtil::random_string(16, "abcdef0123456789");
    /// assert_eq!(random_str.len(), 16);
    /// ```
    pub fn random_string(len: usize, charset: &str) -> String {
        use rand::Rng;
        let mut rng = thread_rng();
        let chars: Vec<char> = charset.chars().collect();
        
        (0..len)
            .map(|_| chars[rng.gen_range(0..chars.len())])
            .collect()
    }

    /// Generate secure alphanumeric string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::crypto::SecureUtil;
    ///
    /// let random_str = SecureUtil::random_alphanumeric(16);
    /// assert_eq!(random_str.len(), 16);
    /// assert!(random_str.chars().all(|c| c.is_alphanumeric()));
    /// ```
    pub fn random_alphanumeric(len: usize) -> String {
        (0..len).map(|_| Alphanumeric.sample(&mut thread_rng()) as char).collect()
    }

    /// Generate secure numeric string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::crypto::SecureUtil;
    ///
    /// let random_num = SecureUtil::random_numeric(8);
    /// assert_eq!(random_num.len(), 8);
    /// assert!(random_num.chars().all(|c| c.is_numeric()));
    /// ```
    pub fn random_numeric(len: usize) -> String {
        Self::random_string(len, "0123456789")
    }

    /// Generate secure hex string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::crypto::SecureUtil;
    ///
    /// let random_hex = SecureUtil::random_hex(16);
    /// assert_eq!(random_hex.len(), 16);
    /// assert!(random_hex.chars().all(|c| c.is_ascii_hexdigit()));
    /// ```
    pub fn random_hex(len: usize) -> String {
        Self::random_string(len, "0123456789abcdef")
    }

    /// Generate secure password with mixed characters
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::crypto::SecureUtil;
    ///
    /// let password = SecureUtil::generate_password(12, true, true, true, false);
    /// assert_eq!(password.len(), 12);
    /// ```
    pub fn generate_password(
        len: usize,
        include_lowercase: bool,
        include_uppercase: bool,
        include_numbers: bool,
        include_symbols: bool,
    ) -> String {
        let mut charset = String::new();
        
        if include_lowercase {
            charset.push_str("abcdefghijklmnopqrstuvwxyz");
        }
        if include_uppercase {
            charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        }
        if include_numbers {
            charset.push_str("0123456789");
        }
        if include_symbols {
            charset.push_str("!@#$%^&*()_+-=[]{}|;:,.<>?");
        }
        
        if charset.is_empty() {
            // Default to alphanumeric if no options selected
            charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".to_string();
        }
        
        Self::random_string(len, &charset)
    }

    /// Generate UUID v4 (random)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::crypto::SecureUtil;
    ///
    /// let uuid = SecureUtil::generate_uuid();
    /// assert_eq!(uuid.len(), 36); // Standard UUID format: 8-4-4-4-12
    /// ```
    pub fn generate_uuid() -> String {
        let bytes = Self::random_bytes(16);
        
        // Set version (4) and variant bits according to RFC 4122
        let mut uuid_bytes = bytes;
        uuid_bytes[6] = (uuid_bytes[6] & 0x0f) | 0x40; // Version 4
        uuid_bytes[8] = (uuid_bytes[8] & 0x3f) | 0x80; // Variant 10
        
        format!(
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            uuid_bytes[0], uuid_bytes[1], uuid_bytes[2], uuid_bytes[3],
            uuid_bytes[4], uuid_bytes[5],
            uuid_bytes[6], uuid_bytes[7],
            uuid_bytes[8], uuid_bytes[9],
            uuid_bytes[10], uuid_bytes[11], uuid_bytes[12], uuid_bytes[13], uuid_bytes[14], uuid_bytes[15]
        )
    }

    /// Generate secure salt for password hashing
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::crypto::SecureUtil;
    ///
    /// let salt = SecureUtil::generate_salt(16);
    /// assert_eq!(salt.len(), 16);
    /// ```
    pub fn generate_salt(len: usize) -> Vec<u8> {
        Self::random_bytes(len)
    }

    /// Generate secure token (base64 encoded random bytes)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::crypto::SecureUtil;
    ///
    /// let token = SecureUtil::generate_token(32);
    /// assert!(!token.is_empty());
    /// ```
    pub fn generate_token(byte_len: usize) -> String {
        use base64::Engine;
        let bytes = Self::random_bytes(byte_len);
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&bytes)
    }

    /// Constant-time comparison of byte slices (prevents timing attacks)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::crypto::SecureUtil;
    ///
    /// let a = b"secret";
    /// let b = b"secret";
    /// let c = b"public";
    ///
    /// assert!(SecureUtil::constant_time_eq(a, b));
    /// assert!(!SecureUtil::constant_time_eq(a, c));
    /// ```
    pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }
        
        let mut result = 0u8;
        for (x, y) in a.iter().zip(b.iter()) {
            result |= x ^ y;
        }
        result == 0
    }

    /// Generate cryptographically secure random integer in range [min, max)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::crypto::SecureUtil;
    ///
    /// let random_int = SecureUtil::random_int(1, 100);
    /// assert!(random_int >= 1 && random_int < 100);
    /// ```
    pub fn random_int(min: u64, max: u64) -> u64 {
        use rand::Rng;
        if min >= max {
            return min;
        }
        thread_rng().gen_range(min..max)
    }

    /// Check if a string is a valid UUID
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::crypto::SecureUtil;
    ///
    /// let uuid = SecureUtil::generate_uuid();
    /// assert!(SecureUtil::is_valid_uuid(&uuid));
    /// assert!(!SecureUtil::is_valid_uuid("not-a-uuid"));
    /// ```
    pub fn is_valid_uuid(uuid: &str) -> bool {
        let uuid_regex = regex::Regex::new(
            r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$"
        ).unwrap();
        uuid_regex.is_match(uuid)
    }

    /// Generate a secure session ID
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::crypto::SecureUtil;
    ///
    /// let session_id = SecureUtil::generate_session_id();
    /// assert_eq!(session_id.len(), 64); // 32 bytes = 64 hex chars
    /// ```
    pub fn generate_session_id() -> String {
        let bytes = Self::random_bytes(32);
        hex::encode(bytes)
    }

    /// Generate a secure API key
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::crypto::SecureUtil;
    ///
    /// let api_key = SecureUtil::generate_api_key(32);
    /// assert!(!api_key.is_empty());
    /// ```
    pub fn generate_api_key(byte_len: usize) -> String {
        let bytes = Self::random_bytes(byte_len);
        base64::engine::general_purpose::STANDARD.encode(&bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_bytes() {
        let bytes1 = SecureUtil::random_bytes(32);
        let bytes2 = SecureUtil::random_bytes(32);
        
        assert_eq!(bytes1.len(), 32);
        assert_eq!(bytes2.len(), 32);
        assert_ne!(bytes1, bytes2); // Should be different
    }

    #[test]
    fn test_random_string() {
        let charset = "abc123";
        let random_str = SecureUtil::random_string(10, charset);
        
        assert_eq!(random_str.len(), 10);
        assert!(random_str.chars().all(|c| charset.contains(c)));
    }

    #[test]
    fn test_random_alphanumeric() {
        let random_str = SecureUtil::random_alphanumeric(16);
        assert_eq!(random_str.len(), 16);
        assert!(random_str.chars().all(|c| c.is_alphanumeric()));
    }

    #[test]
    fn test_random_numeric() {
        let random_num = SecureUtil::random_numeric(8);
        assert_eq!(random_num.len(), 8);
        assert!(random_num.chars().all(|c| c.is_numeric()));
    }

    #[test]
    fn test_random_hex() {
        let random_hex = SecureUtil::random_hex(16);
        assert_eq!(random_hex.len(), 16);
        assert!(random_hex.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_generate_password() {
        let password = SecureUtil::generate_password(12, true, true, true, false);
        assert_eq!(password.len(), 12);
        
        // Test with symbols
        let password_with_symbols = SecureUtil::generate_password(12, true, true, true, true);
        assert_eq!(password_with_symbols.len(), 12);
    }

    #[test]
    fn test_generate_uuid() {
        let uuid1 = SecureUtil::generate_uuid();
        let uuid2 = SecureUtil::generate_uuid();
        
        assert_eq!(uuid1.len(), 36);
        assert_eq!(uuid2.len(), 36);
        assert_ne!(uuid1, uuid2);
        
        // Check format: 8-4-4-4-12
        let parts: Vec<&str> = uuid1.split('-').collect();
        assert_eq!(parts.len(), 5);
        assert_eq!(parts[0].len(), 8);
        assert_eq!(parts[1].len(), 4);
        assert_eq!(parts[2].len(), 4);
        assert_eq!(parts[3].len(), 4);
        assert_eq!(parts[4].len(), 12);
    }

    #[test]
    fn test_generate_salt() {
        let salt1 = SecureUtil::generate_salt(16);
        let salt2 = SecureUtil::generate_salt(16);
        
        assert_eq!(salt1.len(), 16);
        assert_eq!(salt2.len(), 16);
        assert_ne!(salt1, salt2);
    }

    #[test]
    fn test_generate_token() {
        let token1 = SecureUtil::generate_token(32);
        let token2 = SecureUtil::generate_token(32);
        
        assert!(!token1.is_empty());
        assert!(!token2.is_empty());
        assert_ne!(token1, token2);
    }

    #[test]
    fn test_constant_time_eq() {
        let a = b"secret";
        let b = b"secret";
        let c = b"public";
        let d = b"secrets"; // Different length
        
        assert!(SecureUtil::constant_time_eq(a, b));
        assert!(!SecureUtil::constant_time_eq(a, c));
        assert!(!SecureUtil::constant_time_eq(a, d));
    }

    #[test]
    fn test_random_int() {
        let random_int = SecureUtil::random_int(1, 100);
        assert!(random_int >= 1 && random_int < 100);
        
        // Test edge case
        let edge = SecureUtil::random_int(5, 5);
        assert_eq!(edge, 5);
    }

    #[test]
    fn test_is_valid_uuid() {
        let valid_uuid = SecureUtil::generate_uuid();
        assert!(SecureUtil::is_valid_uuid(&valid_uuid));
        
        assert!(!SecureUtil::is_valid_uuid("not-a-uuid"));
        assert!(!SecureUtil::is_valid_uuid("123e4567-e89b-12d3-a456-42661417400")); // Too short
        assert!(!SecureUtil::is_valid_uuid("123e4567-e89b-12d3-a456-4266141740000")); // Too long
    }

    #[test]
    fn test_generate_session_id() {
        let session_id1 = SecureUtil::generate_session_id();
        let session_id2 = SecureUtil::generate_session_id();
        
        assert_eq!(session_id1.len(), 64);
        assert_eq!(session_id2.len(), 64);
        assert_ne!(session_id1, session_id2);
        
        // Should be valid hex
        assert!(session_id1.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_generate_api_key() {
        let api_key1 = SecureUtil::generate_api_key(32);
        let api_key2 = SecureUtil::generate_api_key(32);
        
        assert!(!api_key1.is_empty());
        assert!(!api_key2.is_empty());
        assert_ne!(api_key1, api_key2);
    }
}
