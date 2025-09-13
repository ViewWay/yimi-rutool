//! Message digest algorithms
//!
//! This module provides various hash functions including MD5, SHA-1, SHA-256, SHA-512,
//! and HMAC message authentication codes.

use crate::error::{Error, Result};
use hmac::{Hmac, Mac};
use md5::Md5;
use sha2::{Sha256, Sha512, Digest};

/// MD5 digest utility
pub struct Md5Util;

impl Md5Util {
    /// Calculate MD5 hash of input data
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::crypto::Md5Util;
    ///
    /// let hash = Md5Util::digest(b"hello world");
    /// assert_eq!(hash.len(), 16); // MD5 produces 16 bytes
    /// ```
    pub fn digest(data: &[u8]) -> Vec<u8> {
        let mut hasher = Md5::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }

    /// Calculate MD5 hash and return as hexadecimal string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::crypto::Md5Util;
    ///
    /// let hash_hex = Md5Util::digest_hex(b"hello world");
    /// assert_eq!(hash_hex, "5eb63bbbe01eeed093cb22bb8f5acdc3");
    /// ```
    pub fn digest_hex(data: &[u8]) -> String {
        let hash = Self::digest(data);
        hex::encode(hash)
    }

    /// Calculate MD5 hash of string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::crypto::Md5Util;
    ///
    /// let hash_hex = Md5Util::digest_str("hello world");
    /// assert_eq!(hash_hex, "5eb63bbbe01eeed093cb22bb8f5acdc3");
    /// ```
    pub fn digest_str(data: &str) -> String {
        Self::digest_hex(data.as_bytes())
    }
}

/// SHA digest utility
pub struct ShaUtil;

impl ShaUtil {
    /// Calculate SHA-256 hash of input data
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::crypto::ShaUtil;
    ///
    /// let hash = ShaUtil::sha256(b"hello world");
    /// assert_eq!(hash.len(), 32); // SHA-256 produces 32 bytes
    /// ```
    pub fn sha256(data: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }

    /// Calculate SHA-256 hash and return as hexadecimal string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::crypto::ShaUtil;
    ///
    /// let hash_hex = ShaUtil::sha256_hex(b"hello world");
    /// assert_eq!(hash_hex, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
    /// ```
    pub fn sha256_hex(data: &[u8]) -> String {
        let hash = Self::sha256(data);
        hex::encode(hash)
    }

    /// Calculate SHA-256 hash of string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::crypto::ShaUtil;
    ///
    /// let hash_hex = ShaUtil::sha256_str("hello world");
    /// assert_eq!(hash_hex, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
    /// ```
    pub fn sha256_str(data: &str) -> String {
        Self::sha256_hex(data.as_bytes())
    }

    /// Calculate SHA-512 hash of input data
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::crypto::ShaUtil;
    ///
    /// let hash = ShaUtil::sha512(b"hello world");
    /// assert_eq!(hash.len(), 64); // SHA-512 produces 64 bytes
    /// ```
    pub fn sha512(data: &[u8]) -> Vec<u8> {
        let mut hasher = Sha512::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }

    /// Calculate SHA-512 hash and return as hexadecimal string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::crypto::ShaUtil;
    ///
    /// let hash_hex = ShaUtil::sha512_hex(b"hello world");
    /// assert!(hash_hex.len() == 128); // SHA-512 hex is 128 characters
    /// ```
    pub fn sha512_hex(data: &[u8]) -> String {
        let hash = Self::sha512(data);
        hex::encode(hash)
    }

    /// Calculate SHA-512 hash of string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::crypto::ShaUtil;
    ///
    /// let hash_hex = ShaUtil::sha512_str("hello world");
    /// assert!(hash_hex.len() == 128);
    /// ```
    pub fn sha512_str(data: &str) -> String {
        Self::sha512_hex(data.as_bytes())
    }
}

/// HMAC utility for message authentication codes
pub struct HmacUtil;

impl HmacUtil {
    /// Calculate HMAC-SHA256
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::crypto::HmacUtil;
    ///
    /// let key = b"my-secret-key";
    /// let message = b"hello world";
    /// let hmac = HmacUtil::hmac_sha256(key, message).unwrap();
    /// assert_eq!(hmac.len(), 32);
    /// ```
    pub fn hmac_sha256(key: &[u8], message: &[u8]) -> Result<Vec<u8>> {
        type HmacSha256 = Hmac<Sha256>;
        
        let mut mac = HmacSha256::new_from_slice(key)
            .map_err(|e| Error::crypto(format!("Invalid key length: {}", e)))?;
        mac.update(message);
        let result = mac.finalize();
        Ok(result.into_bytes().to_vec())
    }

    /// Calculate HMAC-SHA256 and return as hexadecimal string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::crypto::HmacUtil;
    ///
    /// let key = b"my-secret-key";
    /// let message = b"hello world";
    /// let hmac_hex = HmacUtil::hmac_sha256_hex(key, message).unwrap();
    /// assert_eq!(hmac_hex.len(), 64); // 32 bytes = 64 hex characters
    /// ```
    pub fn hmac_sha256_hex(key: &[u8], message: &[u8]) -> Result<String> {
        let hmac = Self::hmac_sha256(key, message)?;
        Ok(hex::encode(hmac))
    }

    /// Calculate HMAC-SHA256 for string inputs
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::crypto::HmacUtil;
    ///
    /// let hmac_hex = HmacUtil::hmac_sha256_str("my-secret-key", "hello world").unwrap();
    /// assert_eq!(hmac_hex.len(), 64);
    /// ```
    pub fn hmac_sha256_str(key: &str, message: &str) -> Result<String> {
        Self::hmac_sha256_hex(key.as_bytes(), message.as_bytes())
    }

    /// Verify HMAC-SHA256
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::crypto::HmacUtil;
    ///
    /// let key = b"my-secret-key";
    /// let message = b"hello world";
    /// let hmac = HmacUtil::hmac_sha256(key, message).unwrap();
    /// 
    /// assert!(HmacUtil::verify_hmac_sha256(key, message, &hmac).unwrap());
    /// ```
    pub fn verify_hmac_sha256(key: &[u8], message: &[u8], expected_hmac: &[u8]) -> Result<bool> {
        type HmacSha256 = Hmac<Sha256>;
        
        let mut mac = HmacSha256::new_from_slice(key)
            .map_err(|e| Error::crypto(format!("Invalid key length: {}", e)))?;
        mac.update(message);
        
        match mac.verify_slice(expected_hmac) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Verify HMAC-SHA256 with hex string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::crypto::HmacUtil;
    ///
    /// let key = b"my-secret-key";
    /// let message = b"hello world";
    /// let hmac_hex = HmacUtil::hmac_sha256_hex(key, message).unwrap();
    /// 
    /// assert!(HmacUtil::verify_hmac_sha256_hex(key, message, &hmac_hex).unwrap());
    /// ```
    pub fn verify_hmac_sha256_hex(key: &[u8], message: &[u8], expected_hmac_hex: &str) -> Result<bool> {
        let expected_hmac = hex::decode(expected_hmac_hex)
            .map_err(|e| Error::crypto(format!("Invalid hex string: {}", e)))?;
        Self::verify_hmac_sha256(key, message, &expected_hmac)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5_digest() {
        let hash = Md5Util::digest(b"hello world");
        assert_eq!(hash.len(), 16);
        
        let hash_hex = Md5Util::digest_hex(b"hello world");
        assert_eq!(hash_hex, "5eb63bbbe01eeed093cb22bb8f5acdc3");
    }

    #[test]
    fn test_md5_string() {
        let hash_hex = Md5Util::digest_str("hello world");
        assert_eq!(hash_hex, "5eb63bbbe01eeed093cb22bb8f5acdc3");
    }

    #[test]
    fn test_sha256_digest() {
        let hash = ShaUtil::sha256(b"hello world");
        assert_eq!(hash.len(), 32);
        
        let hash_hex = ShaUtil::sha256_hex(b"hello world");
        assert_eq!(hash_hex, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
    }

    #[test]
    fn test_sha256_string() {
        let hash_hex = ShaUtil::sha256_str("hello world");
        assert_eq!(hash_hex, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
    }

    #[test]
    fn test_sha512_digest() {
        let hash = ShaUtil::sha512(b"hello world");
        assert_eq!(hash.len(), 64);
        
        let hash_hex = ShaUtil::sha512_hex(b"hello world");
        assert_eq!(hash_hex.len(), 128);
    }

    #[test]
    fn test_hmac_sha256() {
        let key = b"my-secret-key";
        let message = b"hello world";
        
        let hmac = HmacUtil::hmac_sha256(key, message).unwrap();
        assert_eq!(hmac.len(), 32);
        
        let hmac_hex = HmacUtil::hmac_sha256_hex(key, message).unwrap();
        assert_eq!(hmac_hex.len(), 64);
        
        // Test verification
        assert!(HmacUtil::verify_hmac_sha256(key, message, &hmac).unwrap());
        assert!(HmacUtil::verify_hmac_sha256_hex(key, message, &hmac_hex).unwrap());
        
        // Test with wrong message
        assert!(!HmacUtil::verify_hmac_sha256(key, b"wrong message", &hmac).unwrap());
    }

    #[test]
    fn test_hmac_string() {
        let hmac_hex = HmacUtil::hmac_sha256_str("my-secret-key", "hello world").unwrap();
        assert_eq!(hmac_hex.len(), 64);
    }
}
