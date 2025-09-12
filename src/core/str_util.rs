//! String utility functions
//!
//! This module provides comprehensive string manipulation utilities,
//! inspired by Hutool's CharSequenceUtil.

use regex::Regex;

/// String utility functions
pub struct StrUtil;

impl StrUtil {
    /// The null string constant: "null"
    pub const NULL: &str = "null";

    /// The empty string constant: ""
    pub const EMPTY: &str = "";

    /// The space string constant: " "
    pub const SPACE: &str = " ";

    /// Check if a string is empty (null or zero length)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// assert_eq!(StrUtil::is_empty(""), true);
    /// assert_eq!(StrUtil::is_empty("   "), false); // contains spaces
    /// assert_eq!(StrUtil::is_empty("abc"), false);
    /// ```
    pub fn is_empty(s: &str) -> bool {
        s.is_empty()
    }

    /// Check if a string is empty or contains only whitespace characters
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// assert_eq!(StrUtil::is_blank(""), true);
    /// assert_eq!(StrUtil::is_blank("   "), true); // only whitespace
    /// assert_eq!(StrUtil::is_blank("  \t\n  "), true); // mixed whitespace
    /// assert_eq!(StrUtil::is_blank("abc"), false);
    /// ```
    pub fn is_blank(s: &str) -> bool {
        s.trim().is_empty()
    }

    /// Check if a string is not empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// assert_eq!(StrUtil::is_not_empty("abc"), true);
    /// assert_eq!(StrUtil::is_not_empty(""), false);
    /// ```
    pub fn is_not_empty(s: &str) -> bool {
        !s.is_empty()
    }

    /// Check if a string is not blank (not empty and not only whitespace)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// assert_eq!(StrUtil::is_not_blank("abc"), true);
    /// assert_eq!(StrUtil::is_not_blank("   "), false);
    /// assert_eq!(StrUtil::is_not_blank(""), false);
    /// ```
    pub fn is_not_blank(s: &str) -> bool {
        !Self::is_blank(s)
    }

    /// Trim whitespace from both ends of a string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// assert_eq!(StrUtil::trim("  hello  "), "hello");
    /// assert_eq!(StrUtil::trim(""), "");
    /// ```
    pub fn trim(s: &str) -> &str {
        s.trim()
    }

    /// Trim whitespace from the start of a string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// assert_eq!(StrUtil::trim_start("  hello  "), "hello  ");
    /// ```
    pub fn trim_start(s: &str) -> &str {
        s.trim_start()
    }

    /// Trim whitespace from the end of a string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// assert_eq!(StrUtil::trim_end("  hello  "), "  hello");
    /// ```
    pub fn trim_end(s: &str) -> &str {
        s.trim_end()
    }

    /// Remove all whitespace characters from a string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// assert_eq!(StrUtil::remove_all_whitespace("a b c"), "abc");
    /// assert_eq!(StrUtil::remove_all_whitespace("a\tb\nc"), "abc");
    /// ```
    pub fn remove_all_whitespace(s: &str) -> String {
        s.chars()
            .filter(|c| !c.is_whitespace())
            .collect()
    }

    /// Convert string to lowercase
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// assert_eq!(StrUtil::to_lower_case("HELLO"), "hello");
    /// ```
    pub fn to_lower_case(s: &str) -> String {
        s.to_lowercase()
    }

    /// Convert string to uppercase
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// assert_eq!(StrUtil::to_upper_case("hello"), "HELLO");
    /// ```
    pub fn to_upper_case(s: &str) -> String {
        s.to_uppercase()
    }

    /// Capitalize the first character of a string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// assert_eq!(StrUtil::capitalize("hello"), "Hello");
    /// assert_eq!(StrUtil::capitalize("HELLO"), "HELLO");
    /// ```
    pub fn capitalize(s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }

    /// Convert string to camelCase
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// assert_eq!(StrUtil::to_camel_case("hello_world"), "helloWorld");
    /// assert_eq!(StrUtil::to_camel_case("user_name_test"), "userNameTest");
    /// ```
    pub fn to_camel_case(s: &str) -> String {
        let parts: Vec<&str> = s.split('_').collect();
        if parts.is_empty() {
            return String::new();
        }

        let mut result = parts[0].to_lowercase();
        for part in parts.iter().skip(1) {
            result.push_str(&Self::capitalize(part));
        }
        result
    }

    /// Convert string to PascalCase
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// assert_eq!(StrUtil::to_pascal_case("hello_world"), "HelloWorld");
    /// assert_eq!(StrUtil::to_pascal_case("user_name"), "UserName");
    /// ```
    pub fn to_pascal_case(s: &str) -> String {
        let parts: Vec<&str> = s.split('_').collect();
        let mut result = String::new();
        for part in parts {
            result.push_str(&Self::capitalize(part));
        }
        result
    }

    /// Convert string to snake_case
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// assert_eq!(StrUtil::to_snake_case("HelloWorld"), "hello_world");
    /// assert_eq!(StrUtil::to_snake_case("UserName"), "user_name");
    /// ```
    pub fn to_snake_case(s: &str) -> String {
        let mut result = String::new();
        for (i, ch) in s.char_indices() {
            if ch.is_uppercase() && i > 0 {
                result.push('_');
            }
            result.push(ch.to_ascii_lowercase());
        }
        result
    }

    /// Check if string starts with the specified prefix
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// assert_eq!(StrUtil::starts_with("hello world", "hello"), true);
    /// assert_eq!(StrUtil::starts_with("hello world", "world"), false);
    /// ```
    pub fn starts_with(s: &str, prefix: &str) -> bool {
        s.starts_with(prefix)
    }

    /// Check if string ends with the specified suffix
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// assert_eq!(StrUtil::ends_with("hello world", "world"), true);
    /// assert_eq!(StrUtil::ends_with("hello world", "hello"), false);
    /// ```
    pub fn ends_with(s: &str, suffix: &str) -> bool {
        s.ends_with(suffix)
    }

    /// Check if string contains the specified substring
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// assert_eq!(StrUtil::contains("hello world", "world"), true);
    /// assert_eq!(StrUtil::contains("hello world", "test"), false);
    /// ```
    pub fn contains(s: &str, substr: &str) -> bool {
        s.contains(substr)
    }

    /// Get substring from start index to end index
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// assert_eq!(StrUtil::sub_string("hello world", 6, 11), "world");
    /// ```
    pub fn sub_string(s: &str, start: usize, end: usize) -> &str {
        if start >= s.len() || start >= end {
            return "";
        }
        let end = end.min(s.len());
        &s[start..end]
    }

    /// Get substring from start index to end of string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// assert_eq!(StrUtil::sub_string_from("hello world", 6), "world");
    /// ```
    pub fn sub_string_from(s: &str, start: usize) -> &str {
        if start >= s.len() {
            return "";
        }
        &s[start..]
    }

    /// Replace all occurrences of a substring
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// assert_eq!(StrUtil::replace("hello world", "world", "rust"), "hello rust");
    /// ```
    pub fn replace(s: &str, from: &str, to: &str) -> String {
        s.replace(from, to)
    }

    /// Replace first occurrence of a substring
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// assert_eq!(StrUtil::replace_first("hello world world", "world", "rust"), "hello rust world");
    /// ```
    pub fn replace_first(s: &str, from: &str, to: &str) -> String {
        if let Some(pos) = s.find(from) {
            let mut result = String::with_capacity(s.len() + to.len() - from.len());
            result.push_str(&s[..pos]);
            result.push_str(to);
            result.push_str(&s[pos + from.len()..]);
            result
        } else {
            s.to_string()
        }
    }

    /// Replace last occurrence of a substring
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// assert_eq!(StrUtil::replace_last("hello world world", "world", "rust"), "hello world rust");
    /// ```
    pub fn replace_last(s: &str, from: &str, to: &str) -> String {
        if let Some(pos) = s.rfind(from) {
            let mut result = String::with_capacity(s.len() + to.len() - from.len());
            result.push_str(&s[..pos]);
            result.push_str(to);
            result.push_str(&s[pos + from.len()..]);
            result
        } else {
            s.to_string()
        }
    }

    /// Split string by delimiter
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// let result = StrUtil::split("a,b,c", ",");
    /// assert_eq!(result, vec!["a", "b", "c"]);
    /// ```
    pub fn split(s: &str, delimiter: &str) -> Vec<String> {
        s.split(delimiter)
            .map(|s| s.to_string())
            .collect()
    }

    /// Join strings with delimiter
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// let result = StrUtil::join(&["a", "b", "c"], ",");
    /// assert_eq!(result, "a,b,c");
    /// ```
    pub fn join(strings: &[&str], delimiter: &str) -> String {
        strings.join(delimiter)
    }

    /// Format string with arguments
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// let result = StrUtil::format("Hello, {0}!", &["World"]);
    /// assert_eq!(result, "Hello, World!");
    ///
    /// let result = StrUtil::format("{0} + {1} = {2}", &["1", "2", "3"]);
    /// assert_eq!(result, "1 + 2 = 3");
    /// ```
    pub fn format(template: &str, args: &[&str]) -> String {
        let mut result = template.to_string();
        for (i, arg) in args.iter().enumerate() {
            let placeholder = format!("{{{}}}", i);
            result = result.replace(&placeholder, arg);
        }
        result
    }

    /// Check if string matches regex pattern
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// assert_eq!(StrUtil::matches("hello123", r"^[a-z]+\d+$").unwrap(), true);
    /// assert_eq!(StrUtil::matches("hello", r"^[a-z]+\d+$").unwrap(), false);
    /// ```
    pub fn matches(s: &str, pattern: &str) -> Result<bool, regex::Error> {
        let regex = Regex::new(pattern)?;
        Ok(regex.is_match(s))
    }

    /// Extract first match from regex pattern
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// let result = StrUtil::extract_first("hello123world", r"\d+").unwrap();
    /// assert_eq!(result, Some("123".to_string()));
    /// ```
    pub fn extract_first(s: &str, pattern: &str) -> Result<Option<String>, regex::Error> {
        let regex = Regex::new(pattern)?;
        Ok(regex.find(s).map(|m| m.as_str().to_string()))
    }

    /// Extract all matches from regex pattern
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// let result = StrUtil::extract_all("a1b2c3", r"\d+").unwrap();
    /// assert_eq!(result, vec!["1", "2", "3"]);
    /// ```
    pub fn extract_all(s: &str, pattern: &str) -> Result<Vec<String>, regex::Error> {
        let regex = Regex::new(pattern)?;
        Ok(regex.find_iter(s)
            .map(|m| m.as_str().to_string())
            .collect())
    }

    /// Reverse a string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// assert_eq!(StrUtil::reverse("hello"), "olleh");
    /// assert_eq!(StrUtil::reverse("123"), "321");
    /// ```
    pub fn reverse(s: &str) -> String {
        s.chars().rev().collect()
    }

    /// Pad string to the left with specified character to reach target length
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// assert_eq!(StrUtil::pad_left("5", 3, '0'), "005");
    /// assert_eq!(StrUtil::pad_left("hello", 3, ' '), "hello");
    /// ```
    pub fn pad_left(s: &str, length: usize, pad_char: char) -> String {
        if s.len() >= length {
            s.to_string()
        } else {
            let pad_len = length - s.len();
            let padding = std::iter::repeat(pad_char).take(pad_len).collect::<String>();
            padding + s
        }
    }

    /// Pad string to the right with specified character to reach target length
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// assert_eq!(StrUtil::pad_right("5", 3, '0'), "500");
    /// assert_eq!(StrUtil::pad_right("hello", 3, ' '), "hello");
    /// ```
    pub fn pad_right(s: &str, length: usize, pad_char: char) -> String {
        if s.len() >= length {
            s.to_string()
        } else {
            let pad_len = length - s.len();
            let padding = std::iter::repeat(pad_char).take(pad_len).collect::<String>();
            s.to_string() + &padding
        }
    }

    /// Center string with padding to reach target length
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// assert_eq!(StrUtil::center("abc", 7, ' '), "  abc  ");
    /// assert_eq!(StrUtil::center("hello", 3, ' '), "hello");
    /// ```
    pub fn center(s: &str, length: usize, pad_char: char) -> String {
        if s.len() >= length {
            s.to_string()
        } else {
            let total_pad = length - s.len();
            let left_pad = total_pad / 2;
            let right_pad = total_pad - left_pad;
            let left_padding = std::iter::repeat(pad_char).take(left_pad).collect::<String>();
            let right_padding = std::iter::repeat(pad_char).take(right_pad).collect::<String>();
            left_padding + s + &right_padding
        }
    }

    /// Check if all strings in the slice are blank
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// assert_eq!(StrUtil::is_all_blank(&["", "  ", "\t"]), true);
    /// assert_eq!(StrUtil::is_all_blank(&["", "hello"]), false);
    /// ```
    pub fn is_all_blank(strings: &[&str]) -> bool {
        strings.iter().all(|s| Self::is_blank(s))
    }

    /// Check if any string in the slice is blank
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// assert_eq!(StrUtil::has_blank(&["hello", "", "world"]), true);
    /// assert_eq!(StrUtil::has_blank(&["hello", "world"]), false);
    /// ```
    pub fn has_blank(strings: &[&str]) -> bool {
        strings.iter().any(|s| Self::is_blank(s))
    }

    /// Check if all strings in the slice are not blank
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// assert_eq!(StrUtil::is_all_not_blank(&["hello", "world"]), true);
    /// assert_eq!(StrUtil::is_all_not_blank(&["hello", ""]), false);
    /// ```
    pub fn is_all_not_blank(strings: &[&str]) -> bool {
        strings.iter().all(|s| Self::is_not_blank(s))
    }

    /// Generate a random string of specified length
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// let random_str = StrUtil::random_string(10);
    /// assert_eq!(random_str.len(), 10);
    /// ```
    pub fn random_string(length: usize) -> String {
        use rand::{Rng, rng};
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

        let mut rng = rng();
        (0..length)
            .map(|_| {
                let idx = rng.random_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    /// Generate a random alphanumeric string of specified length
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// let random_str = StrUtil::random_alphanumeric(8);
    /// assert_eq!(random_str.len(), 8);
    /// // Should only contain letters and numbers
    /// assert!(random_str.chars().all(|c| c.is_alphanumeric()));
    /// ```
    pub fn random_alphanumeric(length: usize) -> String {
        Self::random_string(length)
    }

    /// Generate a random numeric string of specified length
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::core::StrUtil;
    ///
    /// let random_num = StrUtil::random_numeric(5);
    /// assert_eq!(random_num.len(), 5);
    /// // Should only contain digits
    /// assert!(random_num.chars().all(|c| c.is_numeric()));
    /// ```
    pub fn random_numeric(length: usize) -> String {
        use rand::{Rng, rng};
        const DIGITS: &[u8] = b"0123456789";

        let mut rng = rng();
        (0..length)
            .map(|_| {
                let idx = rng.random_range(0..DIGITS.len());
                DIGITS[idx] as char
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_empty() {
        assert!(StrUtil::is_empty(""));
        assert!(!StrUtil::is_empty("hello"));
    }

    #[test]
    fn test_is_blank() {
        assert!(StrUtil::is_blank(""));
        assert!(StrUtil::is_blank("   "));
        assert!(StrUtil::is_blank("  \t\n  "));
        assert!(!StrUtil::is_blank("hello"));
    }

    #[test]
    fn test_trim() {
        assert_eq!(StrUtil::trim("  hello  "), "hello");
        assert_eq!(StrUtil::trim(""), "");
        assert_eq!(StrUtil::trim("no spaces"), "no spaces");
    }

    #[test]
    fn test_to_camel_case() {
        assert_eq!(StrUtil::to_camel_case("hello_world"), "helloWorld");
        assert_eq!(StrUtil::to_camel_case("user_name_test"), "userNameTest");
        assert_eq!(StrUtil::to_camel_case("single"), "single");
    }

    #[test]
    fn test_to_snake_case() {
        assert_eq!(StrUtil::to_snake_case("HelloWorld"), "hello_world");
        assert_eq!(StrUtil::to_snake_case("UserName"), "user_name");
        assert_eq!(StrUtil::to_snake_case("single"), "single");
    }

    #[test]
    fn test_replace() {
        assert_eq!(StrUtil::replace("hello world", "world", "rust"), "hello rust");
        assert_eq!(StrUtil::replace("aaa", "a", "b"), "bbb");
    }

    #[test]
    fn test_format() {
        assert_eq!(StrUtil::format("Hello, {0}!", &["World"]), "Hello, World!");
        assert_eq!(StrUtil::format("{0} + {1} = {2}", &["1", "2", "3"]), "1 + 2 = 3");
    }

    #[test]
    fn test_pad_left() {
        assert_eq!(StrUtil::pad_left("5", 3, '0'), "005");
        assert_eq!(StrUtil::pad_left("hello", 3, ' '), "hello");
    }

    #[test]
    fn test_pad_right() {
        assert_eq!(StrUtil::pad_right("5", 3, '0'), "500");
        assert_eq!(StrUtil::pad_right("hello", 3, ' '), "hello");
    }

    #[test]
    fn test_center() {
        assert_eq!(StrUtil::center("abc", 7, ' '), "  abc  ");
        assert_eq!(StrUtil::center("hello", 3, ' '), "hello");
    }

    #[test]
    fn test_random_string() {
        let s1 = StrUtil::random_string(10);
        let s2 = StrUtil::random_string(10);
        assert_eq!(s1.len(), 10);
        assert_eq!(s2.len(), 10);
        assert_ne!(s1, s2); // Should be different (with very high probability)
    }

    #[test]
    fn test_random_numeric() {
        let s = StrUtil::random_numeric(5);
        assert_eq!(s.len(), 5);
        assert!(s.chars().all(|c| c.is_numeric()));
    }
}
