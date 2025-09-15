//! Type conversion utilities
//!
//! This module provides comprehensive type conversion utilities,
//! supporting conversion between different data types.

/// Type conversion utilities
pub struct Convert;

impl Convert {
    /// Convert string to integer (i32)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::Convert;
    ///
    /// assert_eq!(Convert::to_int("123").unwrap(), 123);
    /// assert_eq!(Convert::to_int("abc"), None);
    /// ```
    pub fn to_int(s: &str) -> Option<i32> {
        s.parse::<i32>().ok()
    }

    /// Convert string to integer (i64)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::Convert;
    ///
    /// assert_eq!(Convert::to_long("123456789").unwrap(), 123456789i64);
    /// ```
    pub fn to_long(s: &str) -> Option<i64> {
        s.parse::<i64>().ok()
    }

    /// Convert string to float (f32)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::Convert;
    ///
    /// assert_eq!(Convert::to_float("123.45").unwrap(), 123.45f32);
    /// ```
    pub fn to_float(s: &str) -> Option<f32> {
        s.parse::<f32>().ok()
    }

    /// Convert string to double (f64)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::Convert;
    ///
    /// assert_eq!(Convert::to_double("123.45").unwrap(), 123.45f64);
    /// ```
    pub fn to_double(s: &str) -> Option<f64> {
        s.parse::<f64>().ok()
    }

    /// Convert string to boolean
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::Convert;
    ///
    /// assert_eq!(Convert::to_bool("true").unwrap(), true);
    /// assert_eq!(Convert::to_bool("false").unwrap(), false);
    /// assert_eq!(Convert::to_bool("1").unwrap(), true);
    /// assert_eq!(Convert::to_bool("0").unwrap(), false);
    /// ```
    pub fn to_bool(s: &str) -> Option<bool> {
        match s.to_lowercase().as_str() {
            "true" | "1" | "yes" | "on" => Some(true),
            "false" | "0" | "no" | "off" => Some(false),
            _ => None,
        }
    }

    /// Convert any value to string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::Convert;
    ///
    /// assert_eq!(Convert::to_str(&123), "123");
    /// assert_eq!(Convert::to_str(&true), "true");
    /// assert_eq!(Convert::to_str(&3.14), "3.14");
    /// ```
    pub fn to_str<T: std::fmt::Display>(value: &T) -> String {
        format!("{value}")
    }

    /// Convert string to Vec<u8>
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::Convert;
    ///
    /// let bytes = Convert::to_bytes("hello");
    /// assert_eq!(bytes, vec![104, 101, 108, 108, 111]);
    /// ```
    pub fn to_bytes(s: &str) -> Vec<u8> {
        s.as_bytes().to_vec()
    }

    /// Convert Vec<u8> to string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::Convert;
    ///
    /// let s = Convert::bytes_to_str(&[104, 101, 108, 108, 111]);
    /// assert_eq!(s, "hello");
    /// ```
    pub fn bytes_to_str(bytes: &[u8]) -> String {
        String::from_utf8_lossy(bytes).to_string()
    }

    /// Convert hexadecimal string to Vec<u8>
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::Convert;
    ///
    /// let bytes = Convert::hex_to_bytes("48656c6c6f").unwrap();
    /// assert_eq!(bytes, vec![72, 101, 108, 108, 111]);
    /// ```
    pub fn hex_to_bytes(hex: &str) -> Option<Vec<u8>> {
        if hex.len() % 2 != 0 {
            return None;
        }

        (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).ok())
            .collect()
    }

    /// Convert Vec<u8> to hexadecimal string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::Convert;
    ///
    /// let hex = Convert::bytes_to_hex(&[72, 101, 108, 108, 111]);
    /// assert_eq!(hex, "48656c6c6f");
    /// ```
    pub fn bytes_to_hex(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }

    /// Convert number to different number types
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::Convert;
    ///
    /// assert_eq!(Convert::to_u32(&42u32), 42u32);
    /// assert_eq!(Convert::to_f64(&3.0f32), 3.0f64);
    /// ```
    pub fn to_u32<T: Into<u32> + Clone>(value: &T) -> u32 {
        value.clone().into()
    }

    /// Convert to u64
    pub fn to_u64<T: Into<u64> + Clone>(value: &T) -> u64 {
        value.clone().into()
    }

    /// Convert to i32
    pub fn to_i32<T: Into<i32> + Clone>(value: &T) -> i32 {
        value.clone().into()
    }

    /// Convert to i64
    pub fn to_i64<T: Into<i64> + Clone>(value: &T) -> i64 {
        value.clone().into()
    }

    /// Convert to f32
    pub fn to_f32<T: Into<f32> + Clone>(value: &T) -> f32 {
        value.clone().into()
    }

    /// Convert to f64
    pub fn to_f64<T: Into<f64> + Clone>(value: &T) -> f64 {
        value.clone().into()
    }

    /// Convert string array to Vec<String>
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::Convert;
    ///
    /// let vec = Convert::to_string_vec(&["a", "b", "c"]);
    /// assert_eq!(vec, vec!["a", "b", "c"]);
    /// ```
    pub fn to_string_vec(strings: &[&str]) -> Vec<String> {
        strings.iter().map(|s| (*s).to_string()).collect()
    }

    /// Convert Vec<String> to string array
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::Convert;
    ///
    /// let strings = vec!["a".to_string(), "b".to_string()];
    /// let array = Convert::to_str_array(&strings);
    /// assert_eq!(array, ["a", "b"]);
    /// ```
    pub fn to_str_array(strings: &[String]) -> Vec<&str> {
        strings.iter().map(std::string::String::as_str).collect()
    }

    /// Convert string to char array
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::Convert;
    ///
    /// let chars = Convert::to_char_array("hello");
    /// assert_eq!(chars, vec!['h', 'e', 'l', 'l', 'o']);
    /// ```
    pub fn to_char_array(s: &str) -> Vec<char> {
        s.chars().collect()
    }

    /// Convert char array to string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::Convert;
    ///
    /// let s = Convert::chars_to_string(&['h', 'e', 'l', 'l', 'o']);
    /// assert_eq!(s, "hello");
    /// ```
    pub fn chars_to_string(chars: &[char]) -> String {
        chars.iter().collect()
    }

    /// Convert string to title case
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::Convert;
    ///
    /// assert_eq!(Convert::to_title_case("hello world"), "Hello World");
    /// assert_eq!(Convert::to_title_case("HELLO WORLD"), "Hello World");
    /// ```
    pub fn to_title_case(s: &str) -> String {
        s.split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => {
                        first.to_uppercase().collect::<String>()
                            + chars.as_str().to_lowercase().as_str()
                    }
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    }

    /// Convert string to kebab-case
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::Convert;
    ///
    /// assert_eq!(Convert::to_kebab_case("HelloWorld"), "hello-world");
    /// assert_eq!(Convert::to_kebab_case("userNameTest"), "user-name-test");
    /// ```
    pub fn to_kebab_case(s: &str) -> String {
        let mut result = String::new();
        for (i, ch) in s.char_indices() {
            if ch.is_uppercase() && i > 0 {
                result.push('-');
            }
            result.push(ch.to_ascii_lowercase());
        }
        result
    }

    /// Convert string to `UPPER_SNAKE_CASE` | 将字符串转换为大写蛇形命名
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::Convert;
    ///
    /// assert_eq!(Convert::to_upper_snake_case("helloWorld"), "HELLO_WORLD");
    /// assert_eq!(Convert::to_upper_snake_case("userName"), "USER_NAME");
    /// ```
    pub fn to_upper_snake_case(s: &str) -> String {
        let mut result = String::new();
        for (i, ch) in s.char_indices() {
            if ch.is_uppercase() && i > 0 {
                result.push('_');
            }
            result.push(ch.to_ascii_uppercase());
        }
        result
    }

    /// Convert string to `lower_snake_case` | 将字符串转换为小写蛇形命名
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::Convert;
    ///
    /// assert_eq!(Convert::to_lower_snake_case("HelloWorld"), "hello_world");
    /// assert_eq!(Convert::to_lower_snake_case("UserName"), "user_name");
    /// ```
    pub fn to_lower_snake_case(s: &str) -> String {
        let mut result = String::new();
        for (i, ch) in s.char_indices() {
            if ch.is_uppercase() && i > 0 {
                result.push('_');
            }
            result.push(ch.to_ascii_lowercase());
        }
        result
    }

    /// Convert number to binary string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::Convert;
    ///
    /// assert_eq!(Convert::to_binary_string(&42u8), "101010");
    /// assert_eq!(Convert::to_binary_string(&255u8), "11111111");
    /// ```
    pub fn to_binary_string<T: std::fmt::Binary>(value: &T) -> String {
        format!("{:b}", value)
    }

    /// Convert number to octal string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::Convert;
    ///
    /// assert_eq!(Convert::to_octal_string(&42u8), "52");
    /// assert_eq!(Convert::to_octal_string(&255u8), "377");
    /// ```
    pub fn to_octal_string<T: std::fmt::Octal>(value: &T) -> String {
        format!("{:o}", value)
    }

    /// Convert number to hexadecimal string (lowercase)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::Convert;
    ///
    /// assert_eq!(Convert::to_hex_string(&42u8), "2a");
    /// assert_eq!(Convert::to_hex_string(&255u8), "ff");
    /// ```
    pub fn to_hex_string<T: std::fmt::LowerHex>(value: &T) -> String {
        format!("{:x}", value)
    }

    /// Convert number to hexadecimal string (uppercase)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::Convert;
    ///
    /// assert_eq!(Convert::to_hex_string_upper(&42u8), "2A");
    /// assert_eq!(Convert::to_hex_string_upper(&255u8), "FF");
    /// ```
    pub fn to_hex_string_upper<T: std::fmt::UpperHex>(value: &T) -> String {
        format!("{:X}", value)
    }

    /// Convert string to integer with default value
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::Convert;
    ///
    /// assert_eq!(Convert::to_int_default("123", 0), 123);
    /// assert_eq!(Convert::to_int_default("abc", 42), 42);
    /// ```
    pub fn to_int_default(s: &str, default: i32) -> i32 {
        Self::to_int(s).unwrap_or(default)
    }

    /// Convert string to long with default value
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::Convert;
    ///
    /// assert_eq!(Convert::to_long_default("123456789", 0), 123456789i64);
    /// assert_eq!(Convert::to_long_default("abc", 42), 42);
    /// ```
    pub fn to_long_default(s: &str, default: i64) -> i64 {
        Self::to_long(s).unwrap_or(default)
    }

    /// Convert string to float with default value
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::Convert;
    ///
    /// assert_eq!(Convert::to_float_default("123.45", 0.0), 123.45f32);
    /// assert_eq!(Convert::to_float_default("abc", 3.14), 3.14);
    /// ```
    pub fn to_float_default(s: &str, default: f32) -> f32 {
        Self::to_float(s).unwrap_or(default)
    }

    /// Convert string to double with default value
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::Convert;
    ///
    /// assert_eq!(Convert::to_double_default("123.45", 0.0), 123.45f64);
    /// assert_eq!(Convert::to_double_default("abc", 3.14), 3.14);
    /// ```
    pub fn to_double_default(s: &str, default: f64) -> f64 {
        Self::to_double(s).unwrap_or(default)
    }

    /// Convert string to bool with default value
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::Convert;
    ///
    /// assert_eq!(Convert::to_bool_default("true", false), true);
    /// assert_eq!(Convert::to_bool_default("maybe", true), true);
    /// ```
    pub fn to_bool_default(s: &str, default: bool) -> bool {
        Self::to_bool(s).unwrap_or(default)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_int() {
        assert_eq!(Convert::to_int("123"), Some(123));
        assert_eq!(Convert::to_int("abc"), None);
        assert_eq!(Convert::to_int("-456"), Some(-456));
    }

    #[test]
    fn test_to_bool() {
        assert_eq!(Convert::to_bool("true"), Some(true));
        assert_eq!(Convert::to_bool("false"), Some(false));
        assert_eq!(Convert::to_bool("1"), Some(true));
        assert_eq!(Convert::to_bool("0"), Some(false));
        assert_eq!(Convert::to_bool("yes"), Some(true));
        assert_eq!(Convert::to_bool("no"), Some(false));
        assert_eq!(Convert::to_bool("maybe"), None);
    }

    #[test]
    fn test_hex_conversion() {
        let bytes = vec![72, 101, 108, 108, 111];
        let hex = Convert::bytes_to_hex(&bytes);
        assert_eq!(hex, "48656c6c6f");

        let decoded = Convert::hex_to_bytes(&hex).unwrap();
        assert_eq!(decoded, bytes);
    }

    #[test]
    fn test_case_conversion() {
        assert_eq!(Convert::to_title_case("hello world"), "Hello World");
        assert_eq!(Convert::to_kebab_case("HelloWorld"), "hello-world");
        assert_eq!(Convert::to_upper_snake_case("helloWorld"), "HELLO_WORLD");
        assert_eq!(Convert::to_lower_snake_case("HelloWorld"), "hello_world");
    }

    #[test]
    fn test_number_formats() {
        assert_eq!(Convert::to_binary_string(&42u8), "101010");
        assert_eq!(Convert::to_octal_string(&42u8), "52");
        assert_eq!(Convert::to_hex_string(&42u8), "2a");
        assert_eq!(Convert::to_hex_string_upper(&42u8), "2A");
    }

    #[test]
    fn test_default_conversions() {
        assert_eq!(Convert::to_int_default("123", 0), 123);
        assert_eq!(Convert::to_int_default("abc", 42), 42);
        assert_eq!(Convert::to_bool_default("true", false), true);
        assert_eq!(Convert::to_bool_default("maybe", true), true);
    }

    #[test]
    fn test_string_array_conversion() {
        let str_array = ["a", "b", "c"];
        let string_vec = Convert::to_string_vec(&str_array);
        assert_eq!(string_vec, vec!["a", "b", "c"]);

        let back_to_array = Convert::to_str_array(&string_vec);
        assert_eq!(back_to_array, vec!["a", "b", "c"]);
    }
}
