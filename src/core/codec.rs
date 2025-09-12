//! Encoding and decoding utilities
//!
//! This module provides common encoding/decoding utilities,
//! including Base64, Base58, Base62, and Hex encoding.

use base64::{Engine as _, engine::general_purpose};
use urlencoding;

/// Base64 utility functions
pub struct Base64Util;

impl Base64Util {
    /// Encode bytes to Base64 string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::Base64Util;
    ///
    /// let encoded = Base64Util::encode("Hello, World!".as_bytes());
    /// assert_eq!(encoded, "SGVsbG8sIFdvcmxkIQ==");
    /// ```
    pub fn encode(data: &[u8]) -> String {
        general_purpose::STANDARD.encode(data)
    }

    /// Decode Base64 string to bytes
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::Base64Util;
    ///
    /// let decoded = Base64Util::decode("SGVsbG8sIFdvcmxkIQ==").unwrap();
    /// assert_eq!(String::from_utf8(decoded).unwrap(), "Hello, World!");
    /// ```
    pub fn decode(data: &str) -> Result<Vec<u8>, base64::DecodeError> {
        general_purpose::STANDARD.decode(data)
    }

    /// Encode string to Base64 string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::Base64Util;
    ///
    /// let encoded = Base64Util::encode_str("Hello, World!");
    /// assert_eq!(encoded, "SGVsbG8sIFdvcmxkIQ==");
    /// ```
    pub fn encode_str(data: &str) -> String {
        Self::encode(data.as_bytes())
    }

    /// Decode Base64 string to string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::Base64Util;
    ///
    /// let decoded = Base64Util::decode_str("SGVsbG8sIFdvcmxkIQ==").unwrap();
    /// assert_eq!(decoded, "Hello, World!");
    /// ```
    pub fn decode_str(data: &str) -> Result<String, Box<dyn std::error::Error>> {
        let bytes = Self::decode(data)?;
        Ok(String::from_utf8(bytes)?)
    }
}

/// Base58 utility functions
pub struct Base58Util;

impl Base58Util {
    const ALPHABET: &'static [u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

    /// Encode bytes to Base58 string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::Base58Util;
    ///
    /// let encoded = Base58Util::encode("Hello".as_bytes());
    /// assert_eq!(encoded, "9Ajdvzr");
    /// ```
    pub fn encode(data: &[u8]) -> String {
        if data.is_empty() {
            return String::new();
        }

        let mut result = Vec::new();
        let mut num = data.iter().fold(0u128, |acc, &b| (acc << 8) | b as u128);

        while num > 0 {
            let remainder = (num % 58) as usize;
            result.push(Self::ALPHABET[remainder]);
            num /= 58;
        }

        // Add leading zeros
        for &byte in data {
            if byte == 0 {
                result.push(b'1');
            } else {
                break;
            }
        }

        result.reverse();
        String::from_utf8(result).unwrap()
    }

    /// Decode Base58 string to bytes
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::Base58Util;
    ///
    /// let decoded = Base58Util::decode("9Ajdvzr").unwrap();
    /// assert_eq!(String::from_utf8(decoded).unwrap(), "Hello");
    /// ```
    pub fn decode(data: &str) -> Result<Vec<u8>, &'static str> {
        if data.is_empty() {
            return Ok(Vec::new());
        }

        let mut result = 0u128;
        let mut leading_zeros = 0;

        // Count leading zeros
        for &byte in data.as_bytes() {
            if byte == b'1' {
                leading_zeros += 1;
            } else {
                break;
            }
        }

        // Decode
        for &byte in data.as_bytes() {
            let digit = Self::ALPHABET.iter().position(|&c| c == byte)
                .ok_or("Invalid Base58 character")?;
            result = result.checked_mul(58).ok_or("Number too large")?;
            result = result.checked_add(digit as u128).ok_or("Number too large")?;
        }

        // Convert to bytes
        let mut bytes = Vec::new();
        let mut temp = result;
        while temp > 0 {
            bytes.push((temp % 256) as u8);
            temp /= 256;
        }
        bytes.reverse();

        // Add leading zeros
        let mut final_result = vec![0u8; leading_zeros];
        final_result.extend(bytes);

        Ok(final_result)
    }
}

/// Hex utility functions
pub struct HexUtil;

impl HexUtil {
    /// Encode bytes to hexadecimal string (lowercase)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::HexUtil;
    ///
    /// let encoded = HexUtil::encode("Hello".as_bytes());
    /// assert_eq!(encoded, "48656c6c6f");
    /// ```
    pub fn encode(data: &[u8]) -> String {
        data.iter()
            .map(|b| format!("{:02x}", b))
            .collect()
    }

    /// Encode bytes to hexadecimal string (uppercase)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::HexUtil;
    ///
    /// let encoded = HexUtil::encode_upper("Hello".as_bytes());
    /// assert_eq!(encoded, "48656C6C6F");
    /// ```
    pub fn encode_upper(data: &[u8]) -> String {
        data.iter()
            .map(|b| format!("{:02X}", b))
            .collect()
    }

    /// Decode hexadecimal string to bytes
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::HexUtil;
    ///
    /// let decoded = HexUtil::decode("48656c6c6f").unwrap();
    /// assert_eq!(String::from_utf8(decoded).unwrap(), "Hello");
    /// ```
    pub fn decode(data: &str) -> Result<Vec<u8>, &'static str> {
        if data.len() % 2 != 0 {
            return Err("Hex string length must be even");
        }

        (0..data.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&data[i..i + 2], 16).map_err(|_| "Invalid hex character"))
            .collect()
    }

    /// Check if string is valid hexadecimal
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::HexUtil;
    ///
    /// assert!(HexUtil::is_valid_hex("48656c6c6f"));
    /// assert!(!HexUtil::is_valid_hex("48656c6c6g")); // 'g' is not valid hex
    /// assert!(!HexUtil::is_valid_hex("48656c6c6")); // odd length
    /// ```
    pub fn is_valid_hex(data: &str) -> bool {
        data.len() % 2 == 0 && data.chars().all(|c| c.is_ascii_hexdigit())
    }
}

/// URL encoding/decoding utilities
pub struct UrlUtil;

impl UrlUtil {
    /// URL encode a string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::codec::UrlUtil;
    ///
    /// let encoded = UrlUtil::encode("Hello World!");
    /// assert_eq!(encoded, "Hello%20World%21");
    /// ```
    pub fn encode(data: &str) -> String {
        urlencoding::encode(data).to_string()
    }

    /// URL decode a string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::codec::UrlUtil;
    ///
    /// let decoded = UrlUtil::decode("Hello%20World%21").unwrap();
    /// assert_eq!(decoded, "Hello World!");
    /// ```
    pub fn decode(data: &str) -> Result<String, std::string::FromUtf8Error> {
        urlencoding::decode(data).map(|cow| cow.to_string()).map_err(|e| e)
    }
}

/// Percent encoding/decoding utilities
pub struct PercentUtil;

impl PercentUtil {
    /// Percent encode a string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::codec::PercentUtil;
    ///
    /// let encoded = PercentUtil::encode("Hello World!");
    /// assert_eq!(encoded, "Hello%20World%21");
    /// ```
    pub fn encode(data: &str) -> String {
        let mut result = String::new();
        for byte in data.as_bytes() {
            match byte {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    result.push(*byte as char);
                }
                _ => {
                    result.push_str(&format!("%{:02X}", byte));
                }
            }
        }
        result
    }

    /// Percent decode a string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::codec::PercentUtil;
    ///
    /// let decoded = PercentUtil::decode("Hello%20World%21").unwrap();
    /// assert_eq!(decoded, "Hello World!");
    /// ```
    pub fn decode(data: &str) -> Result<String, &'static str> {
        let mut result = Vec::new();
        let bytes = data.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            match bytes[i] {
                b'%' => {
                    if i + 2 >= bytes.len() {
                        return Err("Invalid percent encoding");
                    }
                    let hex = &data[i + 1..i + 3];
                    let byte = u8::from_str_radix(hex, 16).map_err(|_| "Invalid hex in percent encoding")?;
                    result.push(byte);
                    i += 3;
                }
                b'+' => {
                    result.push(b' ');
                    i += 1;
                }
                _ => {
                    result.push(bytes[i]);
                    i += 1;
                }
            }
        }

        String::from_utf8(result).map_err(|_| "Invalid UTF-8 in percent encoding")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encode_decode() {
        let original = "Hello, World!";
        let encoded = Base64Util::encode_str(original);
        let decoded = Base64Util::decode_str(&encoded).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_base58_encode_decode() {
        let original = "Hello";
        let encoded = Base58Util::encode(original.as_bytes());
        let decoded = Base58Util::decode(&encoded).unwrap();
        assert_eq!(String::from_utf8(decoded).unwrap(), original);
    }

    #[test]
    fn test_hex_encode_decode() {
        let original = "Hello";
        let encoded = HexUtil::encode(original.as_bytes());
        let decoded = HexUtil::decode(&encoded).unwrap();
        assert_eq!(String::from_utf8(decoded).unwrap(), original);
    }

    #[test]
    fn test_hex_upper_encode() {
        let original = "Hello";
        let encoded = HexUtil::encode_upper(original.as_bytes());
        assert_eq!(encoded, "48656C6C6F");
    }

    #[test]
    fn test_hex_validity() {
        assert!(HexUtil::is_valid_hex("48656c6c6f"));
        assert!(!HexUtil::is_valid_hex("48656c6c6g"));
        assert!(!HexUtil::is_valid_hex("48656c6c6"));
    }

    #[test]
    fn test_url_encode_decode() {
        let original = "Hello World!";
        let encoded = UrlUtil::encode(original);
        let decoded = UrlUtil::decode(&encoded).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_percent_encode_decode() {
        let original = "Hello World!";
        let encoded = PercentUtil::encode(original);
        let decoded = PercentUtil::decode(&encoded).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_percent_encode_special_chars() {
        let original = "你好世界";
        let encoded = PercentUtil::encode(original);
        // Should contain percent encoding for Chinese characters
        assert!(encoded.contains('%'));
        let decoded = PercentUtil::decode(&encoded).unwrap();
        assert_eq!(decoded, original);
    }
}
