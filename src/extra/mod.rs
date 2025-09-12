//! Extra utilities for rutool
//!
//! This module provides additional utilities including:
//! - QR code generation and parsing
//! - Image processing and manipulation
//! - File compression and decompression
//! - Barcode generation
//! - PDF utilities

#[cfg(feature = "qrcode")]
pub mod qr_code;

#[cfg(feature = "image")]
pub mod image_util;

#[cfg(feature = "zip")]
pub mod compression;

/// Re-export commonly used types for convenience
#[cfg(feature = "qrcode")]
pub use qr_code::{QrCode, QrCodeBuilder};

#[cfg(feature = "image")]
pub use image_util::{ImageUtil, ImageFormat};

#[cfg(feature = "zip")]
pub use compression::{CompressionUtil, CompressionFormat};
