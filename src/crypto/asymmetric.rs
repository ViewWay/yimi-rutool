//! Asymmetric encryption algorithms
//!
//! This module provides RSA encryption, decryption, signing, and verification functionality.

use crate::error::{Error, Result};
use rsa::{
    Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
    pkcs1::DecodeRsaPublicKey, pkcs1::EncodeRsaPublicKey,
    traits::PublicKeyParts,
    signature::SignatureEncoding,
};
use base64::Engine;
use rand::rng;

/// RSA utility functions
pub struct RsaUtil;

impl RsaUtil {
    /// Generate RSA key pair with specified bit size
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::crypto::RsaUtil;
    ///
    /// let (private_key, public_key) = RsaUtil::generate_keypair(2048).unwrap();
    /// ```
    pub fn generate_keypair(bits: usize) -> Result<(RsaPrivateKey, RsaPublicKey)> {
        let mut rng = rng();
        let private_key = RsaPrivateKey::new(&mut rng, bits)
            .map_err(|e| Error::crypto(format!("Failed to generate private key: {}", e)))?;
        let public_key = RsaPublicKey::from(&private_key);
        Ok((private_key, public_key))
    }

    /// Export private key to PEM format
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::crypto::RsaUtil;
    ///
    /// let (private_key, _) = RsaUtil::generate_keypair(2048).unwrap();
    /// let pem = RsaUtil::private_key_to_pem(&private_key).unwrap();
    /// assert!(pem.starts_with("-----BEGIN RSA PRIVATE KEY-----"));
    /// ```
    pub fn private_key_to_pem(private_key: &RsaPrivateKey) -> Result<String> {
        use rsa::pkcs1::EncodeRsaPrivateKey;
        private_key.to_pkcs1_pem(rsa::pkcs1::LineEnding::LF)
            .map(|s| s.to_string())
            .map_err(|e| Error::crypto(format!("Failed to encode private key: {}", e)))
    }

    /// Export public key to PEM format
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::crypto::RsaUtil;
    ///
    /// let (_, public_key) = RsaUtil::generate_keypair(2048).unwrap();
    /// let pem = RsaUtil::public_key_to_pem(&public_key).unwrap();
    /// assert!(pem.starts_with("-----BEGIN RSA PUBLIC KEY-----"));
    /// ```
    pub fn public_key_to_pem(public_key: &RsaPublicKey) -> Result<String> {
        public_key.to_pkcs1_pem(rsa::pkcs1::LineEnding::LF)
            .map_err(|e| Error::crypto(format!("Failed to encode public key: {}", e)))
    }

    /// Import private key from PEM format
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::crypto::RsaUtil;
    ///
    /// let (original_key, _) = RsaUtil::generate_keypair(2048).unwrap();
    /// let pem = RsaUtil::private_key_to_pem(&original_key).unwrap();
    /// let imported_key = RsaUtil::private_key_from_pem(&pem).unwrap();
    /// ```
    pub fn private_key_from_pem(pem: &str) -> Result<RsaPrivateKey> {
        use rsa::pkcs1::DecodeRsaPrivateKey;
        RsaPrivateKey::from_pkcs1_pem(pem)
            .map_err(|e| Error::crypto(format!("Failed to decode private key: {}", e)))
    }

    /// Import public key from PEM format
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::crypto::RsaUtil;
    ///
    /// let (_, original_key) = RsaUtil::generate_keypair(2048).unwrap();
    /// let pem = RsaUtil::public_key_to_pem(&original_key).unwrap();
    /// let imported_key = RsaUtil::public_key_from_pem(&pem).unwrap();
    /// ```
    pub fn public_key_from_pem(pem: &str) -> Result<RsaPublicKey> {
        RsaPublicKey::from_pkcs1_pem(pem)
            .map_err(|e| Error::crypto(format!("Failed to decode public key: {}", e)))
    }

    /// Encrypt data using RSA public key (PKCS#1 v1.5 padding)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::crypto::RsaUtil;
    ///
    /// let (private_key, public_key) = RsaUtil::generate_keypair(2048).unwrap();
    /// let message = b"Hello, World!";
    /// let encrypted = RsaUtil::encrypt(&public_key, message).unwrap();
    /// let decrypted = RsaUtil::decrypt(&private_key, &encrypted).unwrap();
    /// assert_eq!(decrypted, message);
    /// ```
    pub fn encrypt(public_key: &RsaPublicKey, data: &[u8]) -> Result<Vec<u8>> {
        let mut rng = rng();
        public_key.encrypt(&mut rng, Pkcs1v15Encrypt, data)
            .map_err(|e| Error::crypto(format!("Encryption failed: {}", e)))
    }

    /// Encrypt string and return base64-encoded result
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::crypto::RsaUtil;
    ///
    /// let (private_key, public_key) = RsaUtil::generate_keypair(2048).unwrap();
    /// let encrypted = RsaUtil::encrypt_str(&public_key, "Hello, World!").unwrap();
    /// let decrypted = RsaUtil::decrypt_str(&private_key, &encrypted).unwrap();
    /// assert_eq!(decrypted, "Hello, World!");
    /// ```
    pub fn encrypt_str(public_key: &RsaPublicKey, data: &str) -> Result<String> {
        let ciphertext = Self::encrypt(public_key, data.as_bytes())?;
        Ok(base64::engine::general_purpose::STANDARD.encode(ciphertext))
    }

    /// Decrypt data using RSA private key (PKCS#1 v1.5 padding)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::crypto::RsaUtil;
    ///
    /// let (private_key, public_key) = RsaUtil::generate_keypair(2048).unwrap();
    /// let message = b"Hello, World!";
    /// let encrypted = RsaUtil::encrypt(&public_key, message).unwrap();
    /// let decrypted = RsaUtil::decrypt(&private_key, &encrypted).unwrap();
    /// assert_eq!(decrypted, message);
    /// ```
    pub fn decrypt(private_key: &RsaPrivateKey, ciphertext: &[u8]) -> Result<Vec<u8>> {
        private_key.decrypt(Pkcs1v15Encrypt, ciphertext)
            .map_err(|e| Error::crypto(format!("Decryption failed: {}", e)))
    }

    /// Decrypt base64-encoded string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::crypto::RsaUtil;
    ///
    /// let (private_key, public_key) = RsaUtil::generate_keypair(2048).unwrap();
    /// let encrypted = RsaUtil::encrypt_str(&public_key, "Hello, World!").unwrap();
    /// let decrypted = RsaUtil::decrypt_str(&private_key, &encrypted).unwrap();
    /// assert_eq!(decrypted, "Hello, World!");
    /// ```
    pub fn decrypt_str(private_key: &RsaPrivateKey, encrypted_base64: &str) -> Result<String> {
        let ciphertext = base64::engine::general_purpose::STANDARD.decode(encrypted_base64)
            .map_err(|e| Error::crypto(format!("Invalid base64: {}", e)))?;
        let plaintext = Self::decrypt(private_key, &ciphertext)?;
        String::from_utf8(plaintext)
            .map_err(|e| Error::crypto(format!("Invalid UTF-8: {}", e)))
    }

    /// Sign data using RSA private key with PSS padding and SHA-256
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::crypto::RsaUtil;
    ///
    /// let (private_key, public_key) = RsaUtil::generate_keypair(2048).unwrap();
    /// let message = b"Hello, World!";
    /// let signature = RsaUtil::sign(&private_key, message).unwrap();
    /// assert!(RsaUtil::verify(&public_key, message, &signature).unwrap());
    /// ```
    pub fn sign(private_key: &RsaPrivateKey, message: &[u8]) -> Result<Vec<u8>> {
        use rsa::pss::{BlindedSigningKey};
        use rsa::signature::{RandomizedSigner};
        use sha2::Sha256;

        let mut rng = rng();
        let signing_key = BlindedSigningKey::<Sha256>::new(private_key.clone());
        let signature = signing_key.sign_with_rng(&mut rng, message);
        Ok(signature.to_vec())
    }

    /// Sign string and return base64-encoded signature
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::crypto::RsaUtil;
    ///
    /// let (private_key, public_key) = RsaUtil::generate_keypair(2048).unwrap();
    /// let signature = RsaUtil::sign_str(&private_key, "Hello, World!").unwrap();
    /// assert!(RsaUtil::verify_str(&public_key, "Hello, World!", &signature).unwrap());
    /// ```
    pub fn sign_str(private_key: &RsaPrivateKey, message: &str) -> Result<String> {
        let signature = Self::sign(private_key, message.as_bytes())?;
        Ok(base64::engine::general_purpose::STANDARD.encode(signature))
    }

    /// Verify signature using RSA public key with PSS padding and SHA-256
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::crypto::RsaUtil;
    ///
    /// let (private_key, public_key) = RsaUtil::generate_keypair(2048).unwrap();
    /// let message = b"Hello, World!";
    /// let signature = RsaUtil::sign(&private_key, message).unwrap();
    /// assert!(RsaUtil::verify(&public_key, message, &signature).unwrap());
    /// ```
    pub fn verify(public_key: &RsaPublicKey, message: &[u8], signature: &[u8]) -> Result<bool> {
        use rsa::pss::VerifyingKey;
        use rsa::signature::Verifier;
        use sha2::Sha256;

        let verifying_key = VerifyingKey::<Sha256>::new(public_key.clone());
        let signature_obj = rsa::pss::Signature::try_from(signature)
            .map_err(|e| Error::crypto(format!("Invalid signature format: {}", e)))?;
        
        match verifying_key.verify(message, &signature_obj) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Verify base64-encoded signature for string message
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::crypto::RsaUtil;
    ///
    /// let (private_key, public_key) = RsaUtil::generate_keypair(2048).unwrap();
    /// let signature = RsaUtil::sign_str(&private_key, "Hello, World!").unwrap();
    /// assert!(RsaUtil::verify_str(&public_key, "Hello, World!", &signature).unwrap());
    /// ```
    pub fn verify_str(public_key: &RsaPublicKey, message: &str, signature_base64: &str) -> Result<bool> {
        let signature = base64::engine::general_purpose::STANDARD.decode(signature_base64)
            .map_err(|e| Error::crypto(format!("Invalid base64: {}", e)))?;
        Self::verify(public_key, message.as_bytes(), &signature)
    }

    /// Get the bit size of an RSA key
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::crypto::RsaUtil;
    ///
    /// let (private_key, _) = RsaUtil::generate_keypair(2048).unwrap();
    /// assert_eq!(RsaUtil::key_size(&private_key), 2048);
    /// ```
    pub fn key_size(key: &RsaPrivateKey) -> usize {
        key.size() * 8
    }

    /// Get the maximum message size for RSA encryption (with PKCS#1 v1.5 padding)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::crypto::RsaUtil;
    ///
    /// let (_, public_key) = RsaUtil::generate_keypair(2048).unwrap();
    /// let max_size = RsaUtil::max_encrypt_size(&public_key);
    /// assert_eq!(max_size, 245); // 2048/8 - 11 = 245 bytes
    /// ```
    pub fn max_encrypt_size(public_key: &RsaPublicKey) -> usize {
        public_key.size() - 11 // PKCS#1 v1.5 padding overhead
    }

    /// Encrypt large data by splitting it into chunks
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::crypto::RsaUtil;
    ///
    /// let (private_key, public_key) = RsaUtil::generate_keypair(2048).unwrap();
    /// let large_message = "A".repeat(500); // Larger than RSA block size
    /// let encrypted = RsaUtil::encrypt_large(&public_key, large_message.as_bytes()).unwrap();
    /// let decrypted = RsaUtil::decrypt_large(&private_key, &encrypted).unwrap();
    /// assert_eq!(decrypted, large_message.as_bytes());
    /// ```
    pub fn encrypt_large(public_key: &RsaPublicKey, data: &[u8]) -> Result<Vec<u8>> {
        let max_size = Self::max_encrypt_size(public_key);
        let mut result = Vec::new();

        for chunk in data.chunks(max_size) {
            let encrypted_chunk = Self::encrypt(public_key, chunk)?;
            // Prepend chunk size (2 bytes) for later decryption
            result.extend_from_slice(&(encrypted_chunk.len() as u16).to_be_bytes());
            result.extend_from_slice(&encrypted_chunk);
        }

        Ok(result)
    }

    /// Decrypt large data that was encrypted with encrypt_large
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::crypto::RsaUtil;
    ///
    /// let (private_key, public_key) = RsaUtil::generate_keypair(2048).unwrap();
    /// let large_message = "A".repeat(500);
    /// let encrypted = RsaUtil::encrypt_large(&public_key, large_message.as_bytes()).unwrap();
    /// let decrypted = RsaUtil::decrypt_large(&private_key, &encrypted).unwrap();
    /// assert_eq!(decrypted, large_message.as_bytes());
    /// ```
    pub fn decrypt_large(private_key: &RsaPrivateKey, encrypted_data: &[u8]) -> Result<Vec<u8>> {
        let mut result = Vec::new();
        let mut offset = 0;

        while offset < encrypted_data.len() {
            if offset + 2 > encrypted_data.len() {
                return Err(Error::crypto("Invalid encrypted data format".to_string()));
            }

            // Read chunk size
            let chunk_size = u16::from_be_bytes([encrypted_data[offset], encrypted_data[offset + 1]]) as usize;
            offset += 2;

            if offset + chunk_size > encrypted_data.len() {
                return Err(Error::crypto("Invalid encrypted data format".to_string()));
            }

            // Decrypt chunk
            let chunk = &encrypted_data[offset..offset + chunk_size];
            let decrypted_chunk = Self::decrypt(private_key, chunk)?;
            result.extend_from_slice(&decrypted_chunk);
            offset += chunk_size;
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_keypair() {
        let (private_key, public_key) = RsaUtil::generate_keypair(2048).unwrap();
        assert_eq!(RsaUtil::key_size(&private_key), 2048);
        assert_eq!(public_key.size(), 256); // 2048 bits = 256 bytes
    }

    #[test]
    fn test_pem_export_import() {
        let (private_key, public_key) = RsaUtil::generate_keypair(2048).unwrap();
        
        // Test private key PEM
        let private_pem = RsaUtil::private_key_to_pem(&private_key).unwrap();
        assert!(private_pem.starts_with("-----BEGIN RSA PRIVATE KEY-----"));
        assert!(private_pem.ends_with("-----END RSA PRIVATE KEY-----\n"));
        
        let imported_private = RsaUtil::private_key_from_pem(&private_pem).unwrap();
        assert_eq!(RsaUtil::key_size(&imported_private), 2048);
        
        // Test public key PEM
        let public_pem = RsaUtil::public_key_to_pem(&public_key).unwrap();
        assert!(public_pem.starts_with("-----BEGIN RSA PUBLIC KEY-----"));
        assert!(public_pem.ends_with("-----END RSA PUBLIC KEY-----\n"));
        
        let imported_public = RsaUtil::public_key_from_pem(&public_pem).unwrap();
        assert_eq!(imported_public.size(), 256);
    }

    #[test]
    fn test_encrypt_decrypt() {
        let (private_key, public_key) = RsaUtil::generate_keypair(2048).unwrap();
        let message = b"Hello, World! This is a test message.";
        
        let encrypted = RsaUtil::encrypt(&public_key, message).unwrap();
        assert_ne!(encrypted.as_slice(), message);
        
        let decrypted = RsaUtil::decrypt(&private_key, &encrypted).unwrap();
        assert_eq!(decrypted, message);
    }

    #[test]
    fn test_encrypt_decrypt_str() {
        let (private_key, public_key) = RsaUtil::generate_keypair(2048).unwrap();
        let message = "Hello, World! 你好世界!";
        
        let encrypted = RsaUtil::encrypt_str(&public_key, message).unwrap();
        assert_ne!(encrypted, message);
        
        let decrypted = RsaUtil::decrypt_str(&private_key, &encrypted).unwrap();
        assert_eq!(decrypted, message);
    }

    #[test]
    fn test_sign_verify() {
        let (private_key, public_key) = RsaUtil::generate_keypair(2048).unwrap();
        let message = b"Hello, World! This is a test message for signing.";
        
        let signature = RsaUtil::sign(&private_key, message).unwrap();
        assert!(!signature.is_empty());
        
        let is_valid = RsaUtil::verify(&public_key, message, &signature).unwrap();
        assert!(is_valid);
        
        // Test with wrong message
        let wrong_message = b"This is a different message";
        let is_valid_wrong = RsaUtil::verify(&public_key, wrong_message, &signature).unwrap();
        assert!(!is_valid_wrong);
    }

    #[test]
    fn test_sign_verify_str() {
        let (private_key, public_key) = RsaUtil::generate_keypair(2048).unwrap();
        let message = "Hello, World! 你好世界! This is a test.";
        
        let signature = RsaUtil::sign_str(&private_key, message).unwrap();
        assert!(!signature.is_empty());
        
        let is_valid = RsaUtil::verify_str(&public_key, message, &signature).unwrap();
        assert!(is_valid);
        
        // Test with wrong message
        let is_valid_wrong = RsaUtil::verify_str(&public_key, "Wrong message", &signature).unwrap();
        assert!(!is_valid_wrong);
    }

    #[test]
    fn test_max_encrypt_size() {
        let (_, public_key) = RsaUtil::generate_keypair(2048).unwrap();
        let max_size = RsaUtil::max_encrypt_size(&public_key);
        assert_eq!(max_size, 245); // 2048/8 - 11 = 245
    }

    #[test]
    fn test_encrypt_large() {
        let (private_key, public_key) = RsaUtil::generate_keypair(2048).unwrap();
        let large_message = "A".repeat(500); // Larger than RSA block size
        
        let encrypted = RsaUtil::encrypt_large(&public_key, large_message.as_bytes()).unwrap();
        assert!(!encrypted.is_empty());
        
        let decrypted = RsaUtil::decrypt_large(&private_key, &encrypted).unwrap();
        assert_eq!(decrypted, large_message.as_bytes());
    }

    #[test]
    fn test_key_compatibility() {
        let (private_key1, public_key1) = RsaUtil::generate_keypair(2048).unwrap();
        let (private_key2, public_key2) = RsaUtil::generate_keypair(2048).unwrap();
        
        let message = b"Test message";
        
        // Encrypt with public_key1, should only decrypt with private_key1
        let encrypted1 = RsaUtil::encrypt(&public_key1, message).unwrap();
        let decrypted1 = RsaUtil::decrypt(&private_key1, &encrypted1).unwrap();
        assert_eq!(decrypted1, message);
        
        // Should fail to decrypt with wrong private key
        let result = RsaUtil::decrypt(&private_key2, &encrypted1);
        assert!(result.is_err());
        
        // Sign with private_key1, should only verify with public_key1
        let signature1 = RsaUtil::sign(&private_key1, message).unwrap();
        assert!(RsaUtil::verify(&public_key1, message, &signature1).unwrap());
        assert!(!RsaUtil::verify(&public_key2, message, &signature1).unwrap());
    }
}