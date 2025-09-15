//! QR code generation and parsing utilities
//!
//! This module provides functionality to generate QR codes from text,
//! URLs, and other data formats, as well as parsing QR codes from images.

use crate::error::{Error, Result};
use std::fmt;
use std::path::Path;

#[cfg(feature = "qrcode")]
use qrcode::{EcLevel, QrCode as LibQrCode, Version};

#[cfg(feature = "image")]
use image::{DynamicImage, ImageFormat as ImgFormat, Luma};

/// QR code generator and parser
pub struct QrCode {
    /// The actual QR code data
    #[cfg(feature = "qrcode")]
    qr_code: LibQrCode,
    /// Raw text data
    data: String,
    /// Error correction level
    error_correction: ErrorCorrectionLevel,
    /// QR code version (size)
    version: Option<u8>,
}

/// Error correction levels for QR codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCorrectionLevel {
    /// Low error correction (~7% correction capability)
    Low,
    /// Medium error correction (~15% correction capability)
    Medium,
    /// Quartile error correction (~25% correction capability)
    Quartile,
    /// High error correction (~30% correction capability)
    High,
}

impl Default for ErrorCorrectionLevel {
    fn default() -> Self {
        Self::Medium
    }
}

/// QR code output format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QrCodeFormat {
    /// PNG image format
    Png,
    /// JPEG image format
    Jpeg,
    /// SVG vector format
    Svg,
    /// ASCII text format
    Ascii,
    /// Unicode text format
    Unicode,
}

impl QrCode {
    /// Create a new QR code from text
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::extra::QrCode;
    ///
    /// let qr = QrCode::new("Hello, World!").unwrap();
    /// ```
    pub fn new(data: &str) -> Result<Self> {
        Self::with_error_correction(data, ErrorCorrectionLevel::Medium)
    }

    /// Create a new QR code with specific error correction level
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::extra::{QrCode, ErrorCorrectionLevel};
    ///
    /// let qr = QrCode::with_error_correction("Hello, World!", ErrorCorrectionLevel::High).unwrap();
    /// ```
    pub fn with_error_correction(
        data: &str,
        error_correction: ErrorCorrectionLevel,
    ) -> Result<Self> {
        #[cfg(feature = "qrcode")]
        {
            let ec_level = match error_correction {
                ErrorCorrectionLevel::Low => EcLevel::L,
                ErrorCorrectionLevel::Medium => EcLevel::M,
                ErrorCorrectionLevel::Quartile => EcLevel::Q,
                ErrorCorrectionLevel::High => EcLevel::H,
            };

            let qr_code = LibQrCode::with_error_correction_level(data, ec_level)
                .map_err(|e| Error::validation(format!("Failed to create QR code: {}", e)))?;

            Ok(Self {
                qr_code,
                data: data.to_string(),
                error_correction,
                version: None,
            })
        }

        #[cfg(not(feature = "qrcode"))]
        {
            let _ = (data, error_correction); // Avoid unused warnings
            Err(Error::validation("QR code feature not enabled".to_string()))
        }
    }

    /// Create a QR code with specific version (size)
    pub fn with_version(data: &str, version: u8) -> Result<Self> {
        #[cfg(feature = "qrcode")]
        {
            if !(1..=40).contains(&version) {
                return Err(Error::validation(format!(
                    "Invalid QR code version: {}",
                    version
                )));
            }

            let qr_version = Version::Normal(version as i16);
            let qr_code = LibQrCode::with_version(data, qr_version, EcLevel::M)
                .map_err(|e| Error::validation(format!("Failed to create QR code: {}", e)))?;

            Ok(Self {
                qr_code,
                data: data.to_string(),
                error_correction: ErrorCorrectionLevel::Medium,
                version: Some(version),
            })
        }

        #[cfg(not(feature = "qrcode"))]
        {
            let _ = (data, version); // Avoid unused warnings
            Err(Error::validation("QR code feature not enabled".to_string()))
        }
    }

    /// Get the raw data stored in the QR code
    pub fn data(&self) -> &str {
        &self.data
    }

    /// Get the error correction level
    pub fn error_correction_level(&self) -> ErrorCorrectionLevel {
        self.error_correction
    }

    /// Get the QR code version (if explicitly set)
    pub fn version(&self) -> Option<u8> {
        self.version
    }

    /// Get the actual QR code version used
    #[cfg(feature = "qrcode")]
    pub fn actual_version(&self) -> u8 {
        match self.qr_code.version() {
            Version::Normal(v) => v as u8,
            Version::Micro(_) => 1, // Micro QR codes are much smaller
        }
    }

    /// Generate QR code as ASCII string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::extra::QrCode;
    ///
    /// let qr = QrCode::new("Test").unwrap();
    /// let ascii = qr.to_ascii();
    /// println!("{}", ascii);
    /// ```
    pub fn to_ascii(&self) -> String {
        #[cfg(feature = "qrcode")]
        {
            self.qr_code
                .render::<char>()
                .quiet_zone(false)
                .module_dimensions(1, 1)
                .build()
        }

        #[cfg(not(feature = "qrcode"))]
        {
            "QR code feature not enabled".to_string()
        }
    }

    /// Generate QR code as Unicode string (with prettier characters)
    pub fn to_unicode(&self) -> String {
        #[cfg(feature = "qrcode")]
        {
            self.qr_code
                .render::<char>()
                .quiet_zone(false)
                .module_dimensions(1, 1)
                .dark_color('█')
                .light_color(' ')
                .build()
        }

        #[cfg(not(feature = "qrcode"))]
        {
            "QR code feature not enabled".to_string()
        }
    }

    /// Generate QR code as PNG image bytes
    #[cfg(all(feature = "qrcode", feature = "image"))]
    pub fn to_png(&self, size: u32) -> Result<Vec<u8>> {
        let image = self.to_image(size)?;
        let mut bytes = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut bytes);

        image
            .write_to(&mut cursor, ImgFormat::Png)
            .map_err(|e| Error::validation(format!("Failed to encode PNG: {}", e)))?;

        Ok(bytes)
    }

    /// Generate QR code as JPEG image bytes
    #[cfg(all(feature = "qrcode", feature = "image"))]
    pub fn to_jpeg(&self, size: u32, _quality: u8) -> Result<Vec<u8>> {
        let image = self.to_image(size)?;
        let mut bytes = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut bytes);

        // Convert to RGB for JPEG
        let rgb_image = image.to_rgb8();
        let dynamic = DynamicImage::ImageRgb8(rgb_image);

        dynamic
            .write_to(&mut cursor, ImgFormat::Jpeg)
            .map_err(|e| Error::validation(format!("Failed to encode JPEG: {}", e)))?;

        Ok(bytes)
    }

    /// Generate QR code as image
    #[cfg(all(feature = "qrcode", feature = "image"))]
    pub fn to_image(&self, size: u32) -> Result<DynamicImage> {
        if size == 0 {
            return Err(Error::validation(
                "Image size must be greater than 0".to_string(),
            ));
        }

        let qr_size = self.qr_code.width();
        let scale = size / qr_size as u32;
        if scale == 0 {
            return Err(Error::validation(format!(
                "Image size {} is too small for QR code size {}",
                size, qr_size
            )));
        }

        let image = self
            .qr_code
            .render::<Luma<u8>>()
            .min_dimensions(size, size)
            .max_dimensions(size, size)
            .build();

        Ok(DynamicImage::ImageLuma8(image))
    }

    /// Save QR code as image file
    #[cfg(all(feature = "qrcode", feature = "image"))]
    pub fn save_image<P: AsRef<Path>>(&self, path: P, size: u32) -> Result<()> {
        let image = self.to_image(size)?;
        image
            .save(&path)
            .map_err(|e| Error::validation(format!("Failed to save image: {}", e)))?;
        Ok(())
    }

    /// Generate QR code as SVG string
    #[cfg(feature = "qrcode")]
    pub fn to_svg(&self, size: u32) -> String {
        // Simple SVG generation without external svg crate
        let qr_size = self.qr_code.width();
        let scale = size / qr_size as u32;
        if scale == 0 {
            return format!(
                "<svg width='{}' height='{}' xmlns='http://www.w3.org/2000/svg'></svg>",
                size, size
            );
        }

        let mut svg = format!(
            "<svg width='{}' height='{}' xmlns='http://www.w3.org/2000/svg'>",
            size, size
        );

        let colors: Vec<qrcode::Color> = self.qr_code.to_colors();
        let width: usize = self.qr_code.width();
        for y in 0..width {
            for x in 0..width {
                let index = y * width + x;
                if let Some(&cell) = colors.get(index) {
                    if cell != qrcode::Color::Light {
                        let x_pos = x as u32 * scale;
                        let y_pos = y as u32 * scale;
                        svg.push_str(&format!(
                            "<rect x='{}' y='{}' width='{}' height='{}' fill='black'/>",
                            x_pos, y_pos, scale, scale
                        ));
                    }
                }
            }
        }

        svg.push_str("</svg>");
        svg
    }

    /// Save QR code as SVG file
    #[cfg(feature = "qrcode")]
    pub fn save_svg<P: AsRef<Path>>(&self, path: P, size: u32) -> Result<()> {
        let svg_content = self.to_svg(size);
        std::fs::write(path, svg_content)
            .map_err(|e| Error::validation(format!("Failed to save SVG: {}", e)))?;
        Ok(())
    }

    /// Get QR code capacity information
    #[cfg(feature = "qrcode")]
    pub fn capacity_info(&self) -> QrCodeCapacity {
        let version = self.actual_version();
        let ec_level = self.error_correction;

        // These are approximate capacities based on QR code specifications
        let (numeric, alphanumeric, binary) = match (version, ec_level) {
            (1, ErrorCorrectionLevel::Low) => (41, 25, 17),
            (1, ErrorCorrectionLevel::Medium) => (34, 20, 14),
            (1, ErrorCorrectionLevel::Quartile) => (27, 16, 11),
            (1, ErrorCorrectionLevel::High) => (17, 10, 7),
            (10, ErrorCorrectionLevel::Low) => (652, 395, 271),
            (10, ErrorCorrectionLevel::Medium) => (513, 311, 213),
            (10, ErrorCorrectionLevel::Quartile) => (364, 221, 151),
            (10, ErrorCorrectionLevel::High) => (288, 174, 119),
            (20, ErrorCorrectionLevel::Low) => (1852, 1122, 771),
            (20, ErrorCorrectionLevel::Medium) => (1429, 866, 595),
            (20, ErrorCorrectionLevel::Quartile) => (1057, 641, 441),
            (20, ErrorCorrectionLevel::High) => (808, 493, 339),
            (40, ErrorCorrectionLevel::Low) => (7089, 4296, 2953),
            (40, ErrorCorrectionLevel::Medium) => (5596, 3391, 2331),
            (40, ErrorCorrectionLevel::Quartile) => (3993, 2420, 1663),
            (40, ErrorCorrectionLevel::High) => (3057, 1852, 1273),
            _ => {
                // Rough estimation for other versions
                let base = version as usize * version as usize;
                let factor = match ec_level {
                    ErrorCorrectionLevel::Low => 1.0,
                    ErrorCorrectionLevel::Medium => 0.8,
                    ErrorCorrectionLevel::Quartile => 0.6,
                    ErrorCorrectionLevel::High => 0.4,
                };
                let numeric = (base as f64 * factor * 0.3) as usize;
                let alphanumeric = (numeric as f64 * 0.6) as usize;
                let binary = (alphanumeric as f64 * 0.7) as usize;
                (numeric, alphanumeric, binary)
            }
        };

        QrCodeCapacity {
            version,
            error_correction: ec_level,
            numeric_capacity: numeric,
            alphanumeric_capacity: alphanumeric,
            binary_capacity: binary,
        }
    }
}

/// QR code capacity information
#[derive(Debug, Clone)]
pub struct QrCodeCapacity {
    /// QR code version
    pub version: u8,
    /// Error correction level
    pub error_correction: ErrorCorrectionLevel,
    /// Maximum numeric characters
    pub numeric_capacity: usize,
    /// Maximum alphanumeric characters
    pub alphanumeric_capacity: usize,
    /// Maximum binary bytes
    pub binary_capacity: usize,
}

impl fmt::Display for QrCodeCapacity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "QR Code Capacity (Version {}, {:?}):",
            self.version, self.error_correction
        )?;
        writeln!(f, "  Numeric: {} characters", self.numeric_capacity)?;
        writeln!(
            f,
            "  Alphanumeric: {} characters",
            self.alphanumeric_capacity
        )?;
        writeln!(f, "  Binary: {} bytes", self.binary_capacity)?;
        Ok(())
    }
}

/// Builder for creating QR codes with custom settings
#[derive(Debug)]
pub struct QrCodeBuilder {
    data: String,
    error_correction: ErrorCorrectionLevel,
    version: Option<u8>,
}

impl QrCodeBuilder {
    /// Create a new QR code builder
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::extra::{QrCodeBuilder, ErrorCorrectionLevel};
    ///
    /// let qr = QrCodeBuilder::new("https://example.com")
    ///     .error_correction(ErrorCorrectionLevel::High)
    ///     .version(5)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn new(data: &str) -> Self {
        Self {
            data: data.to_string(),
            error_correction: ErrorCorrectionLevel::Medium,
            version: None,
        }
    }

    /// Set the error correction level
    pub fn error_correction(mut self, level: ErrorCorrectionLevel) -> Self {
        self.error_correction = level;
        self
    }

    /// Set the QR code version (size)
    pub fn version(mut self, version: u8) -> Self {
        self.version = Some(version);
        self
    }

    /// Build the QR code
    pub fn build(self) -> Result<QrCode> {
        match self.version {
            Some(version) => QrCode::with_version(&self.data, version),
            None => QrCode::with_error_correction(&self.data, self.error_correction),
        }
    }
}

/// Utility functions for QR codes
pub struct QrCodeUtil;

impl QrCodeUtil {
    /// Create a QR code for a URL
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::extra::QrCodeUtil;
    ///
    /// let qr = QrCodeUtil::url("https://github.com/rust-lang/rust").unwrap();
    /// ```
    pub fn url(url: &str) -> Result<QrCode> {
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(Error::validation(
                "URL must start with http:// or https://".to_string(),
            ));
        }
        QrCode::new(url)
    }

    /// Create a QR code for WiFi credentials
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::extra::QrCodeUtil;
    ///
    /// let qr = QrCodeUtil::wifi("MyNetwork", "password123", "WPA").unwrap();
    /// ```
    pub fn wifi(ssid: &str, password: &str, security: &str) -> Result<QrCode> {
        let wifi_string = format!("WIFI:T:{};S:{};P:{};H:false;;", security, ssid, password);
        QrCode::new(&wifi_string)
    }

    /// Create a QR code for email
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::extra::QrCodeUtil;
    ///
    /// let qr = QrCodeUtil::email("test@example.com", "Subject", "Body").unwrap();
    /// ```
    pub fn email(to: &str, subject: &str, body: &str) -> Result<QrCode> {
        let email_string = format!(
            "mailto:{}?subject={}&body={}",
            to,
            urlencoding::encode(subject),
            urlencoding::encode(body)
        );
        QrCode::new(&email_string)
    }

    /// Create a QR code for SMS
    pub fn sms(number: &str, message: &str) -> Result<QrCode> {
        let sms_string = format!("sms:{}?body={}", number, urlencoding::encode(message));
        QrCode::new(&sms_string)
    }

    /// Create a QR code for phone number
    pub fn phone(number: &str) -> Result<QrCode> {
        let phone_string = format!("tel:{}", number);
        QrCode::new(&phone_string)
    }

    /// Create a QR code for vCard contact
    pub fn vcard(name: &str, phone: &str, email: &str, organization: &str) -> Result<QrCode> {
        let vcard = format!(
            "BEGIN:VCARD\nVERSION:3.0\nFN:{}\nTEL:{}\nEMAIL:{}\nORG:{}\nEND:VCARD",
            name, phone, email, organization
        );
        QrCode::new(&vcard)
    }

    /// Create a QR code for geo location
    pub fn geo_location(latitude: f64, longitude: f64) -> Result<QrCode> {
        let geo_string = format!("geo:{},{}", latitude, longitude);
        QrCode::new(&geo_string)
    }

    /// Get optimal QR code version for given data
    pub fn optimal_version(data: &str, error_correction: ErrorCorrectionLevel) -> u8 {
        let data_len = data.len();

        // Simple heuristic based on data length and error correction
        let base_version = match data_len {
            0..=25 => 1,
            26..=47 => 2,
            48..=77 => 3,
            78..=114 => 4,
            115..=154 => 5,
            155..=195 => 6,
            196..=224 => 7,
            225..=279 => 8,
            280..=335 => 9,
            336..=395 => 10,
            _ => ((data_len as f64 / 40.0).ceil() as u8).min(40),
        };

        // Adjust for error correction level
        let adjustment = match error_correction {
            ErrorCorrectionLevel::Low => 0,
            ErrorCorrectionLevel::Medium => 1,
            ErrorCorrectionLevel::Quartile => 2,
            ErrorCorrectionLevel::High => 3,
        };

        (base_version + adjustment).min(40).max(1)
    }
}

impl fmt::Display for ErrorCorrectionLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Low => write!(f, "Low"),
            Self::Medium => write!(f, "Medium"),
            Self::Quartile => write!(f, "Quartile"),
            Self::High => write!(f, "High"),
        }
    }
}

impl fmt::Display for QrCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "QR Code:")?;
        writeln!(f, "  Data: {}", self.data)?;
        writeln!(f, "  Error Correction: {}", self.error_correction)?;
        if let Some(version) = self.version {
            writeln!(f, "  Version: {}", version)?;
        }
        #[cfg(feature = "qrcode")]
        {
            writeln!(f, "  Actual Version: {}", self.actual_version())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qr_code_creation() {
        let qr = QrCode::new("Hello, World!");
        assert!(qr.is_ok());

        let qr = qr.unwrap();
        assert_eq!(qr.data(), "Hello, World!");
        assert_eq!(qr.error_correction_level(), ErrorCorrectionLevel::Medium);
    }

    #[test]
    fn test_qr_code_with_error_correction() {
        let qr = QrCode::with_error_correction("Test", ErrorCorrectionLevel::High);
        assert!(qr.is_ok());

        let qr = qr.unwrap();
        assert_eq!(qr.error_correction_level(), ErrorCorrectionLevel::High);
    }

    #[test]
    fn test_qr_code_with_version() {
        let qr = QrCode::with_version("Test", 5);
        assert!(qr.is_ok());

        let qr = qr.unwrap();
        assert_eq!(qr.version(), Some(5));
    }

    #[test]
    fn test_invalid_version() {
        let qr = QrCode::with_version("Test", 0);
        assert!(qr.is_err());

        let qr = QrCode::with_version("Test", 41);
        assert!(qr.is_err());
    }

    #[test]
    fn test_qr_code_builder() {
        let qr = QrCodeBuilder::new("Builder test")
            .error_correction(ErrorCorrectionLevel::High)
            .version(3)
            .build();

        assert!(qr.is_ok());
        let qr = qr.unwrap();
        assert_eq!(qr.data(), "Builder test");
        // When version is explicitly set, error correction level is overridden to medium
        assert_eq!(qr.error_correction_level(), ErrorCorrectionLevel::Medium);
        assert_eq!(qr.version(), Some(3));
    }

    #[test]
    fn test_ascii_output() {
        let qr = QrCode::new("Test").unwrap();
        let ascii = qr.to_ascii();
        assert!(!ascii.is_empty());
        #[cfg(feature = "qrcode")]
        {
            assert!(ascii.contains('█') || ascii.contains('#') || ascii.contains('▀'));
        }
    }

    #[test]
    fn test_unicode_output() {
        let qr = QrCode::new("Test").unwrap();
        let unicode = qr.to_unicode();
        assert!(!unicode.is_empty());
        #[cfg(feature = "qrcode")]
        {
            assert!(unicode.contains('█'));
        }
    }

    #[test]
    fn test_qr_util_url() {
        let qr = QrCodeUtil::url("https://example.com");
        assert!(qr.is_ok());

        let qr = QrCodeUtil::url("invalid-url");
        assert!(qr.is_err());
    }

    #[test]
    fn test_qr_util_wifi() {
        let qr = QrCodeUtil::wifi("MyNetwork", "password123", "WPA");
        assert!(qr.is_ok());

        let qr = qr.unwrap();
        assert!(qr.data().contains("WIFI:"));
        assert!(qr.data().contains("MyNetwork"));
        assert!(qr.data().contains("password123"));
    }

    #[test]
    fn test_qr_util_email() {
        let qr = QrCodeUtil::email("test@example.com", "Hello", "Test message");
        assert!(qr.is_ok());

        let qr = qr.unwrap();
        assert!(qr.data().contains("mailto:"));
        assert!(qr.data().contains("test@example.com"));
    }

    #[test]
    fn test_qr_util_phone() {
        let qr = QrCodeUtil::phone("+1234567890");
        assert!(qr.is_ok());

        let qr = qr.unwrap();
        assert!(qr.data().contains("tel:"));
        assert!(qr.data().contains("+1234567890"));
    }

    #[test]
    fn test_qr_util_geo() {
        let qr = QrCodeUtil::geo_location(37.7749, -122.4194);
        assert!(qr.is_ok());

        let qr = qr.unwrap();
        assert!(qr.data().contains("geo:"));
        assert!(qr.data().contains("37.7749"));
        assert!(qr.data().contains("-122.4194"));
    }

    #[test]
    fn test_optimal_version() {
        let version = QrCodeUtil::optimal_version("Short", ErrorCorrectionLevel::Low);
        assert_eq!(version, 1);

        let long_text = "A".repeat(200);
        let version = QrCodeUtil::optimal_version(&long_text, ErrorCorrectionLevel::High);
        assert!(version > 5);
    }

    #[cfg(feature = "qrcode")]
    #[test]
    fn test_capacity_info() {
        let qr = QrCode::with_version("Test", 5).unwrap();
        let capacity = qr.capacity_info();
        assert_eq!(capacity.version, 5);
        assert!(capacity.numeric_capacity > 0);
        assert!(capacity.alphanumeric_capacity > 0);
        assert!(capacity.binary_capacity > 0);
    }

    #[test]
    fn test_error_correction_display() {
        assert_eq!(ErrorCorrectionLevel::Low.to_string(), "Low");
        assert_eq!(ErrorCorrectionLevel::Medium.to_string(), "Medium");
        assert_eq!(ErrorCorrectionLevel::Quartile.to_string(), "Quartile");
        assert_eq!(ErrorCorrectionLevel::High.to_string(), "High");
    }

    #[test]
    fn test_qr_code_display() {
        let qr = QrCode::new("Display test").unwrap();
        let display = qr.to_string();
        assert!(display.contains("QR Code:"));
        assert!(display.contains("Display test"));
        assert!(display.contains("Medium"));
    }

    #[cfg(all(feature = "qrcode", feature = "image"))]
    #[test]
    fn test_image_generation() {
        let qr = QrCode::new("Image test").unwrap();
        let image = qr.to_image(200);
        assert!(image.is_ok());

        let image = image.unwrap();
        // The actual image size depends on QR code size and scaling
        // Just check that we got a reasonable size
        assert!(image.width() > 0);
        assert!(image.height() > 0);
        assert!(image.width() <= 200);
        assert!(image.height() <= 200);
    }

    #[cfg(all(feature = "qrcode", feature = "image"))]
    #[test]
    fn test_png_generation() {
        let qr = QrCode::new("PNG test").unwrap();
        let png_bytes = qr.to_png(100);
        assert!(png_bytes.is_ok());

        let bytes = png_bytes.unwrap();
        assert!(!bytes.is_empty());
        // PNG signature
        assert_eq!(&bytes[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
    }

    #[test]
    fn test_invalid_image_size() {
        let qr = QrCode::new("Size test").unwrap();

        #[cfg(all(feature = "qrcode", feature = "image"))]
        {
            let result = qr.to_image(0);
            assert!(result.is_err());
        }
    }
}
