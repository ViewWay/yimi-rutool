//! Symmetric encryption utilities
//!
//! This module provides AES encryption and decryption functionality
//! with various modes including AES-256-GCM for authenticated encryption.

use crate::error::{Error, Result};
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce
};
use base64::Engine;
use rand::{RngCore, thread_rng};

/// AES encryption utility
pub struct AesUtil;

impl AesUtil {
    /// AES-256 key size in bytes
    pub const KEY_SIZE: usize = 32;
    
    /// AES-GCM nonce size in bytes
    pub const NONCE_SIZE: usize = 12;

    /// Generate a random AES-256 key
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::crypto::AesUtil;
    ///
    /// let key = AesUtil::generate_key();
    /// assert_eq!(key.len(), 32); // AES-256 key is 32 bytes
    /// ```
    pub fn generate_key() -> Vec<u8> {
        let mut key = vec![0u8; Self::KEY_SIZE];
        thread_rng().fill_bytes(&mut key);
        key
    }

    /// Generate a random nonce for AES-GCM
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::crypto::AesUtil;
    ///
    /// let nonce = AesUtil::generate_nonce();
    /// assert_eq!(nonce.len(), 12); // AES-GCM nonce is 12 bytes
    /// ```
    pub fn generate_nonce() -> Vec<u8> {
        let mut nonce = vec![0u8; Self::NONCE_SIZE];
        thread_rng().fill_bytes(&mut nonce);
        nonce
    }

    /// Encrypt data using AES-256-GCM
    ///
    /// # Arguments
    ///
    /// * `data` - The data to encrypt
    /// * `key` - The 32-byte AES-256 key
    /// * `nonce` - The 12-byte nonce (optional, will generate if None)
    ///
    /// # Returns
    ///
    /// Returns a tuple of (encrypted_data, nonce_used)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::crypto::AesUtil;
    ///
    /// let key = AesUtil::generate_key();
    /// let plaintext = b"Hello, World!";
    /// let (ciphertext, nonce) = AesUtil::encrypt(plaintext, &key, None).unwrap();
    /// 
    /// assert_ne!(ciphertext, plaintext); // Should be encrypted
    /// assert_eq!(nonce.len(), 12);
    /// ```
    pub fn encrypt(data: &[u8], key: &[u8], nonce: Option<&[u8]>) -> Result<(Vec<u8>, Vec<u8>)> {
        if key.len() != Self::KEY_SIZE {
            return Err(Error::crypto(format!("Invalid key size: expected {}, got {}", Self::KEY_SIZE, key.len())));
        }

        let aes_key = Key::<Aes256Gcm>::from_slice(key);
        let cipher = Aes256Gcm::new(aes_key);

        let nonce_bytes = match nonce {
            Some(n) => {
                if n.len() != Self::NONCE_SIZE {
                    return Err(Error::crypto(format!("Invalid nonce size: expected {}, got {}", Self::NONCE_SIZE, n.len())));
                }
                n.to_vec()
            }
            None => Self::generate_nonce(),
        };

        let nonce_obj = Nonce::from_slice(&nonce_bytes);
        let ciphertext = cipher.encrypt(nonce_obj, data)
            .map_err(|e| Error::crypto(format!("Encryption failed: {}", e)))?;

        Ok((ciphertext, nonce_bytes))
    }

    /// Decrypt data using AES-256-GCM
    ///
    /// # Arguments
    ///
    /// * `ciphertext` - The encrypted data
    /// * `key` - The 32-byte AES-256 key
    /// * `nonce` - The 12-byte nonce used for encryption
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::crypto::AesUtil;
    ///
    /// let key = AesUtil::generate_key();
    /// let plaintext = b"Hello, World!";
    /// let (ciphertext, nonce) = AesUtil::encrypt(plaintext, &key, None).unwrap();
    /// let decrypted = AesUtil::decrypt(&ciphertext, &key, &nonce).unwrap();
    /// 
    /// assert_eq!(decrypted, plaintext);
    /// ```
    pub fn decrypt(ciphertext: &[u8], key: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
        if key.len() != Self::KEY_SIZE {
            return Err(Error::crypto(format!("Invalid key size: expected {}, got {}", Self::KEY_SIZE, key.len())));
        }

        if nonce.len() != Self::NONCE_SIZE {
            return Err(Error::crypto(format!("Invalid nonce size: expected {}, got {}", Self::NONCE_SIZE, nonce.len())));
        }

        let aes_key = Key::<Aes256Gcm>::from_slice(key);
        let cipher = Aes256Gcm::new(aes_key);
        let nonce_obj = Nonce::from_slice(nonce);

        let plaintext = cipher.decrypt(nonce_obj, ciphertext)
            .map_err(|e| Error::crypto(format!("Decryption failed: {}", e)))?;

        Ok(plaintext)
    }

    /// Encrypt string using AES-256-GCM
    ///
    /// # Arguments
    ///
    /// * `data` - The string to encrypt
    /// * `key` - The 32-byte AES-256 key
    ///
    /// # Returns
    ///
    /// Returns a base64-encoded string containing nonce + ciphertext
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::crypto::AesUtil;
    ///
    /// let key = AesUtil::generate_key();
    /// let encrypted = AesUtil::encrypt_str("Hello, World!", &key).unwrap();
    /// let decrypted = AesUtil::decrypt_str(&encrypted, &key).unwrap();
    /// 
    /// assert_eq!(decrypted, "Hello, World!");
    /// ```
    pub fn encrypt_str(data: &str, key: &[u8]) -> Result<String> {
        let (ciphertext, nonce) = Self::encrypt(data.as_bytes(), key, None)?;
        
        // Combine nonce + ciphertext
        let mut combined = Vec::with_capacity(Self::NONCE_SIZE + ciphertext.len());
        combined.extend_from_slice(&nonce);
        combined.extend_from_slice(&ciphertext);
        
        // Encode as base64
        Ok(base64::engine::general_purpose::STANDARD.encode(&combined))
    }

    /// Decrypt base64-encoded string using AES-256-GCM
    ///
    /// # Arguments
    ///
    /// * `encrypted_data` - The base64-encoded encrypted data
    /// * `key` - The 32-byte AES-256 key
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::crypto::AesUtil;
    ///
    /// let key = AesUtil::generate_key();
    /// let encrypted = AesUtil::encrypt_str("Hello, World!", &key).unwrap();
    /// let decrypted = AesUtil::decrypt_str(&encrypted, &key).unwrap();
    /// 
    /// assert_eq!(decrypted, "Hello, World!");
    /// ```
    pub fn decrypt_str(encrypted_data: &str, key: &[u8]) -> Result<String> {
        // Decode from base64
        let combined = base64::engine::general_purpose::STANDARD.decode(encrypted_data)
            .map_err(|e| Error::crypto(format!("Invalid base64: {}", e)))?;

        if combined.len() < Self::NONCE_SIZE {
            return Err(Error::crypto("Encrypted data too short".to_string()));
        }

        // Extract nonce and ciphertext
        let (nonce, ciphertext) = combined.split_at(Self::NONCE_SIZE);
        
        // Decrypt
        let plaintext = Self::decrypt(ciphertext, key, nonce)?;
        
        // Convert to string
        String::from_utf8(plaintext)
            .map_err(|e| Error::crypto(format!("Invalid UTF-8: {}", e)))
    }

    /// Encrypt data with password-based key derivation
    ///
    /// # Arguments
    ///
    /// * `data` - The data to encrypt
    /// * `password` - The password to derive key from
    /// * `salt` - Optional salt (will generate if None)
    ///
    /// # Returns
    ///
    /// Returns a tuple of (encrypted_data, salt_used)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::crypto::AesUtil;
    ///
    /// let (encrypted, salt) = AesUtil::encrypt_with_password(b"Hello, World!", "my_password", None).unwrap();
    /// let decrypted = AesUtil::decrypt_with_password(&encrypted, "my_password", &salt).unwrap();
    /// 
    /// assert_eq!(decrypted, b"Hello, World!");
    /// ```
    pub fn encrypt_with_password(data: &[u8], password: &str, salt: Option<&[u8]>) -> Result<(Vec<u8>, Vec<u8>)> {
        let salt_bytes = match salt {
            Some(s) => s.to_vec(),
            None => {
                let mut salt = vec![0u8; 16]; // 16-byte salt
                thread_rng().fill_bytes(&mut salt);
                salt
            }
        };

        let key = Self::derive_key_from_password(password, &salt_bytes)?;
        let (ciphertext, nonce) = Self::encrypt(data, &key, None)?;
        
        // Combine nonce + ciphertext (without salt for this function)
        let mut combined = Vec::with_capacity(Self::NONCE_SIZE + ciphertext.len());
        combined.extend_from_slice(&nonce);
        combined.extend_from_slice(&ciphertext);
        
        Ok((combined, salt_bytes))
    }

    /// Decrypt data encrypted with password-based key derivation
    ///
    /// # Arguments
    ///
    /// * `encrypted_data` - The encrypted data containing salt + nonce + ciphertext
    /// * `password` - The password to derive key from
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::crypto::{AesUtil, SecureUtil};
    ///
    /// // Create combined format manually for the example
    /// let password = "my_password";
    /// let plaintext = b"Hello, World!";
    /// let salt = SecureUtil::random_bytes(16);
    /// let key = AesUtil::derive_key_from_password(password, &salt).unwrap();
    /// let (ciphertext, nonce) = AesUtil::encrypt(plaintext, &key, None).unwrap();
    /// 
    /// // Combine salt + nonce + ciphertext
    /// let mut combined = Vec::new();
    /// combined.extend_from_slice(&salt);
    /// combined.extend_from_slice(&nonce);
    /// combined.extend_from_slice(&ciphertext);
    /// 
    /// let decrypted = AesUtil::decrypt_with_password_combined(&combined, password).unwrap();
    /// assert_eq!(decrypted, plaintext);
    /// ```
    pub fn decrypt_with_password_combined(encrypted_data: &[u8], password: &str) -> Result<Vec<u8>> {
        if encrypted_data.len() < 16 + Self::NONCE_SIZE {
            return Err(Error::crypto("Encrypted data too short".to_string()));
        }

        // Extract salt, nonce, and ciphertext
        let (salt, rest) = encrypted_data.split_at(16);
        let (nonce, ciphertext) = rest.split_at(Self::NONCE_SIZE);
        
        let key = Self::derive_key_from_password(password, salt)?;
        Self::decrypt(ciphertext, &key, nonce)
    }

    /// Decrypt data with password-based key derivation (separate salt)
    ///
    /// # Arguments
    ///
    /// * `encrypted_data` - The encrypted data (nonce + ciphertext)
    /// * `password` - The password to derive key from
    /// * `salt` - The salt used for key derivation
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::crypto::AesUtil;
    ///
    /// let (encrypted, salt) = AesUtil::encrypt_with_password(b"Hello, World!", "my_password", None).unwrap();
    /// let decrypted = AesUtil::decrypt_with_password(&encrypted, "my_password", &salt).unwrap();
    /// 
    /// assert_eq!(decrypted, b"Hello, World!");
    /// ```
    pub fn decrypt_with_password(encrypted_data: &[u8], password: &str, salt: &[u8]) -> Result<Vec<u8>> {
        if encrypted_data.len() < Self::NONCE_SIZE {
            return Err(Error::crypto("Encrypted data too short".to_string()));
        }

        // Extract nonce and ciphertext
        let (nonce, ciphertext) = encrypted_data.split_at(Self::NONCE_SIZE);
        
        let key = Self::derive_key_from_password(password, salt)?;
        Self::decrypt(ciphertext, &key, nonce)
    }

    /// Derive AES key from password using PBKDF2
    ///
    /// # Arguments
    ///
    /// * `password` - The password
    /// * `salt` - The salt for key derivation
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::crypto::AesUtil;
    ///
    /// let salt = b"my_salt_12345678";
    /// let key = AesUtil::derive_key_from_password("my_password", salt).unwrap();
    /// assert_eq!(key.len(), 32); // AES-256 key
    /// ```
    pub fn derive_key_from_password(password: &str, salt: &[u8]) -> Result<Vec<u8>> {
        use pbkdf2::pbkdf2_hmac;
        use sha2::Sha256;
        
        let mut key = vec![0u8; Self::KEY_SIZE];
        pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, 100_000, &mut key);
        Ok(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_key() {
        let key = AesUtil::generate_key();
        assert_eq!(key.len(), AesUtil::KEY_SIZE);
        
        // Generate another key and ensure they're different
        let key2 = AesUtil::generate_key();
        assert_ne!(key, key2);
    }

    #[test]
    fn test_generate_nonce() {
        let nonce = AesUtil::generate_nonce();
        assert_eq!(nonce.len(), AesUtil::NONCE_SIZE);
        
        // Generate another nonce and ensure they're different
        let nonce2 = AesUtil::generate_nonce();
        assert_ne!(nonce, nonce2);
    }

    #[test]
    fn test_encrypt_decrypt() {
        let key = AesUtil::generate_key();
        let plaintext = b"Hello, World! This is a test message.";
        
        let (ciphertext, nonce) = AesUtil::encrypt(plaintext, &key, None).unwrap();
        assert_ne!(ciphertext.as_slice(), plaintext);
        assert_eq!(nonce.len(), AesUtil::NONCE_SIZE);
        
        let decrypted = AesUtil::decrypt(&ciphertext, &key, &nonce).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_encrypt_decrypt_with_custom_nonce() {
        let key = AesUtil::generate_key();
        let nonce = AesUtil::generate_nonce();
        let plaintext = b"Hello, World!";
        
        let (ciphertext, used_nonce) = AesUtil::encrypt(plaintext, &key, Some(&nonce)).unwrap();
        assert_eq!(used_nonce, nonce);
        
        let decrypted = AesUtil::decrypt(&ciphertext, &key, &nonce).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_encrypt_decrypt_str() {
        let key = AesUtil::generate_key();
        let plaintext = "Hello, World! 你好世界!";
        
        let encrypted = AesUtil::encrypt_str(plaintext, &key).unwrap();
        assert_ne!(encrypted, plaintext);
        
        let decrypted = AesUtil::decrypt_str(&encrypted, &key).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_encrypt_decrypt_with_password() {
        let password = "my_secret_password";
        let plaintext = b"Hello, World! This is a secret message.";
        
        let (encrypted, salt) = AesUtil::encrypt_with_password(plaintext, password, None).unwrap();
        assert_ne!(encrypted.as_slice(), plaintext);
        assert_eq!(salt.len(), 16);
        
        let decrypted = AesUtil::decrypt_with_password(&encrypted, password, &salt).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_encrypt_decrypt_with_password_combined() {
        let password = "my_secret_password";
        let plaintext = b"Hello, World! This is a secret message.";
        
        // For combined mode, we need to use the combined format with salt
        let salt = crate::crypto::SecureUtil::random_bytes(16);
        let key = AesUtil::derive_key_from_password(password, &salt).unwrap();
        let (ciphertext, nonce) = AesUtil::encrypt(plaintext, &key, None).unwrap();
        
        // Combine salt + nonce + ciphertext for combined mode
        let mut combined = Vec::with_capacity(salt.len() + AesUtil::NONCE_SIZE + ciphertext.len());
        combined.extend_from_slice(&salt);
        combined.extend_from_slice(&nonce);
        combined.extend_from_slice(&ciphertext);
        
        let decrypted = AesUtil::decrypt_with_password_combined(&combined, password).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_derive_key_from_password() {
        let password = "my_password";
        let salt = b"my_salt_12345678";
        
        let key1 = AesUtil::derive_key_from_password(password, salt).unwrap();
        let key2 = AesUtil::derive_key_from_password(password, salt).unwrap();
        
        assert_eq!(key1.len(), AesUtil::KEY_SIZE);
        assert_eq!(key1, key2); // Same password and salt should generate same key
        
        // Different salt should generate different key
        let different_salt = b"different_salt12";
        let key3 = AesUtil::derive_key_from_password(password, different_salt).unwrap();
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_invalid_key_size() {
        let short_key = vec![0u8; 16]; // Too short
        let plaintext = b"test";
        
        let result = AesUtil::encrypt(plaintext, &short_key, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_nonce_size() {
        let key = AesUtil::generate_key();
        let short_nonce = vec![0u8; 8]; // Too short
        let plaintext = b"test";
        
        let result = AesUtil::encrypt(plaintext, &key, Some(&short_nonce));
        assert!(result.is_err());
    }

    #[test]
    fn test_wrong_key_decryption() {
        let key1 = AesUtil::generate_key();
        let key2 = AesUtil::generate_key();
        let plaintext = b"Hello, World!";
        
        let (ciphertext, nonce) = AesUtil::encrypt(plaintext, &key1, None).unwrap();
        let result = AesUtil::decrypt(&ciphertext, &key2, &nonce);
        assert!(result.is_err()); // Should fail with wrong key
    }
}