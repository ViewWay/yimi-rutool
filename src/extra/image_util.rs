//! Image processing and manipulation utilities
//!
//! This module provides functionality for loading, processing, and saving images
//! with support for various formats and transformations.

use crate::error::{Error, Result};
use std::path::Path;

#[cfg(feature = "image")]
use image::{
    ColorType, DynamicImage, GenericImageView, ImageBuffer, ImageFormat as ImgFormat, Rgb, Rgba,
    imageops::FilterType,
};

/// Supported image formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    /// PNG format
    Png,
    /// JPEG format
    Jpeg,
    /// GIF format
    Gif,
    /// WebP format
    WebP,
    /// BMP format
    Bmp,
    /// TIFF format
    Tiff,
    /// ICO format
    Ico,
}

impl ImageFormat {
    /// Convert to image crate format
    #[cfg(feature = "image")]
    #[allow(dead_code)]
    fn to_image_format(self) -> ImgFormat {
        match self {
            Self::Png => ImgFormat::Png,
            Self::Jpeg => ImgFormat::Jpeg,
            Self::Gif => ImgFormat::Gif,
            Self::WebP => ImgFormat::WebP,
            Self::Bmp => ImgFormat::Bmp,
            Self::Tiff => ImgFormat::Tiff,
            Self::Ico => ImgFormat::Ico,
        }
    }

    /// Get file extension for this format
    pub fn extension(self) -> &'static str {
        match self {
            Self::Png => "png",
            Self::Jpeg => "jpg",
            Self::Gif => "gif",
            Self::WebP => "webp",
            Self::Bmp => "bmp",
            Self::Tiff => "tiff",
            Self::Ico => "ico",
        }
    }

    /// Detect format from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "png" => Some(Self::Png),
            "jpg" | "jpeg" => Some(Self::Jpeg),
            "gif" => Some(Self::Gif),
            "webp" => Some(Self::WebP),
            "bmp" => Some(Self::Bmp),
            "tiff" | "tif" => Some(Self::Tiff),
            "ico" => Some(Self::Ico),
            _ => None,
        }
    }
}

/// Image resize filter types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizeFilter {
    /// Nearest neighbor (fastest, lowest quality)
    Nearest,
    /// Linear interpolation
    Triangle,
    /// Catmull-Rom spline
    CatmullRom,
    /// Gaussian filter
    Gaussian,
    /// Lanczos filter (slowest, highest quality)
    Lanczos3,
}

#[cfg(feature = "image")]
impl ResizeFilter {
    fn to_filter_type(self) -> FilterType {
        match self {
            Self::Nearest => FilterType::Nearest,
            Self::Triangle => FilterType::Triangle,
            Self::CatmullRom => FilterType::CatmullRom,
            Self::Gaussian => FilterType::Gaussian,
            Self::Lanczos3 => FilterType::Lanczos3,
        }
    }
}

/// Image rotation angle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotationAngle {
    /// Rotate 90 degrees clockwise
    Rotate90,
    /// Rotate 180 degrees
    Rotate180,
    /// Rotate 270 degrees clockwise (90 degrees counter-clockwise)
    Rotate270,
}

/// Image information
#[derive(Debug, Clone)]
pub struct ImageInfo {
    /// Image width in pixels
    pub width: u32,
    /// Image height in pixels
    pub height: u32,
    /// Color type
    pub color_type: String,
    /// Image format
    pub format: Option<ImageFormat>,
    /// File size in bytes
    pub file_size: Option<u64>,
}

/// Image processing utility
pub struct ImageUtil;

impl ImageUtil {
    /// Load an image from file
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::extra::ImageUtil;
    ///
    /// // let image = ImageUtil::load("path/to/image.jpg").unwrap();
    /// ```
    #[cfg(feature = "image")]
    pub fn load<P: AsRef<Path>>(path: P) -> Result<DynamicImage> {
        let image = image::open(&path)
            .map_err(|e| Error::validation(format!("Failed to load image: {}", e)))?;
        Ok(image)
    }

    /// Load an image from bytes
    #[cfg(feature = "image")]
    pub fn load_from_bytes(bytes: &[u8]) -> Result<DynamicImage> {
        let image = image::load_from_memory(bytes)
            .map_err(|e| Error::validation(format!("Failed to load image from bytes: {}", e)))?;
        Ok(image)
    }

    /// Save an image to file
    #[cfg(feature = "image")]
    pub fn save<P: AsRef<Path>>(image: &DynamicImage, path: P) -> Result<()> {
        image
            .save(&path)
            .map_err(|e| Error::validation(format!("Failed to save image: {}", e)))?;
        Ok(())
    }

    /// Save an image to bytes with specified format
    #[cfg(feature = "image")]
    pub fn save_to_bytes(image: &DynamicImage, format: ImageFormat) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut bytes);

        match format {
            ImageFormat::Png => image.write_to(&mut cursor, ImgFormat::Png),
            ImageFormat::Jpeg => image.write_to(&mut cursor, ImgFormat::Jpeg),
            ImageFormat::Gif => image.write_to(&mut cursor, ImgFormat::Gif),
            ImageFormat::Bmp => image.write_to(&mut cursor, ImgFormat::Bmp),
            ImageFormat::Tiff => image.write_to(&mut cursor, ImgFormat::Tiff),
            _ => {
                return Err(Error::validation(format!(
                    "Unsupported output format: {:?}",
                    format
                )));
            }
        }
        .map_err(|e| Error::validation(format!("Failed to encode image: {}", e)))?;

        Ok(bytes)
    }

    /// Get image information
    #[cfg(feature = "image")]
    pub fn get_info<P: AsRef<Path>>(path: P) -> Result<ImageInfo> {
        let image = Self::load(&path)?;
        let (width, height) = image.dimensions();

        let color_type = match image.color() {
            ColorType::L8 => "Grayscale",
            ColorType::La8 => "Grayscale + Alpha",
            ColorType::Rgb8 => "RGB",
            ColorType::Rgba8 => "RGBA",
            ColorType::L16 => "Grayscale 16-bit",
            ColorType::La16 => "Grayscale + Alpha 16-bit",
            ColorType::Rgb16 => "RGB 16-bit",
            ColorType::Rgba16 => "RGBA 16-bit",
            ColorType::Rgb32F => "RGB 32-bit Float",
            ColorType::Rgba32F => "RGBA 32-bit Float",
            _ => "Unknown",
        }
        .to_string();

        let format = path
            .as_ref()
            .extension()
            .and_then(|ext| ext.to_str())
            .and_then(ImageFormat::from_extension);

        let file_size = std::fs::metadata(&path).map(|metadata| metadata.len()).ok();

        Ok(ImageInfo {
            width,
            height,
            color_type,
            format,
            file_size,
        })
    }

    /// Resize an image
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::extra::{ImageUtil, ResizeFilter};
    ///
    /// // let image = ImageUtil::load("input.jpg").unwrap();
    /// // let resized = ImageUtil::resize(&image, 800, 600, ResizeFilter::Lanczos3);
    /// ```
    #[cfg(feature = "image")]
    pub fn resize(
        image: &DynamicImage,
        width: u32,
        height: u32,
        filter: ResizeFilter,
    ) -> DynamicImage {
        image.resize_exact(width, height, filter.to_filter_type())
    }

    /// Resize an image maintaining aspect ratio
    #[cfg(feature = "image")]
    pub fn resize_to_fit(
        image: &DynamicImage,
        max_width: u32,
        max_height: u32,
        filter: ResizeFilter,
    ) -> DynamicImage {
        image.resize(max_width, max_height, filter.to_filter_type())
    }

    /// Create a thumbnail
    #[cfg(feature = "image")]
    pub fn thumbnail(image: &DynamicImage, size: u32) -> DynamicImage {
        Self::resize_to_fit(image, size, size, ResizeFilter::Lanczos3)
    }

    /// Crop an image
    #[cfg(feature = "image")]
    pub fn crop(
        image: &DynamicImage,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> Result<DynamicImage> {
        let (img_width, img_height) = image.dimensions();

        if x + width > img_width || y + height > img_height {
            return Err(Error::validation(format!(
                "Crop area ({}, {}, {}, {}) exceeds image dimensions ({}, {})",
                x, y, width, height, img_width, img_height
            )));
        }

        Ok(image.crop_imm(x, y, width, height))
    }

    /// Rotate an image
    #[cfg(feature = "image")]
    pub fn rotate(image: &DynamicImage, angle: RotationAngle) -> DynamicImage {
        match angle {
            RotationAngle::Rotate90 => image.rotate90(),
            RotationAngle::Rotate180 => image.rotate180(),
            RotationAngle::Rotate270 => image.rotate270(),
        }
    }

    /// Flip an image horizontally
    #[cfg(feature = "image")]
    pub fn flip_horizontal(image: &DynamicImage) -> DynamicImage {
        image.fliph()
    }

    /// Flip an image vertically
    #[cfg(feature = "image")]
    pub fn flip_vertical(image: &DynamicImage) -> DynamicImage {
        image.flipv()
    }

    /// Convert image to grayscale
    #[cfg(feature = "image")]
    pub fn to_grayscale(image: &DynamicImage) -> DynamicImage {
        image.grayscale()
    }

    /// Adjust image brightness
    #[cfg(feature = "image")]
    pub fn adjust_brightness(image: &DynamicImage, value: i32) -> DynamicImage {
        image.brighten(value)
    }

    /// Adjust image contrast
    #[cfg(feature = "image")]
    pub fn adjust_contrast(image: &DynamicImage, contrast: f32) -> DynamicImage {
        image.adjust_contrast(contrast)
    }

    /// Apply blur filter
    #[cfg(feature = "image")]
    pub fn blur(image: &DynamicImage, sigma: f32) -> DynamicImage {
        image.blur(sigma)
    }

    /// Apply unsharpen filter
    #[cfg(feature = "image")]
    pub fn unsharpen(image: &DynamicImage, sigma: f32, threshold: i32) -> DynamicImage {
        image.unsharpen(sigma, threshold)
    }

    /// Apply invert filter
    #[cfg(feature = "image")]
    pub fn invert(image: &DynamicImage) -> DynamicImage {
        let mut inverted = image.clone();
        inverted.invert();
        inverted
    }

    /// Convert image format
    #[cfg(feature = "image")]
    pub fn convert_format<P: AsRef<Path>>(
        input_path: P,
        output_path: P,
        format: ImageFormat,
    ) -> Result<()> {
        let image = Self::load(input_path)?;

        match format {
            ImageFormat::Png => image.save_with_format(&output_path, ImgFormat::Png),
            ImageFormat::Jpeg => image.save_with_format(&output_path, ImgFormat::Jpeg),
            ImageFormat::Gif => image.save_with_format(&output_path, ImgFormat::Gif),
            ImageFormat::Bmp => image.save_with_format(&output_path, ImgFormat::Bmp),
            ImageFormat::Tiff => image.save_with_format(&output_path, ImgFormat::Tiff),
            ImageFormat::WebP => image.save_with_format(&output_path, ImgFormat::WebP),
            ImageFormat::Ico => image.save_with_format(&output_path, ImgFormat::Ico),
        }
        .map_err(|e| Error::validation(format!("Failed to convert image format: {}", e)))?;

        Ok(())
    }

    /// Create a solid color image
    #[cfg(feature = "image")]
    pub fn create_solid_color(width: u32, height: u32, r: u8, g: u8, b: u8) -> DynamicImage {
        let color = Rgb([r, g, b]);
        let buffer = ImageBuffer::from_pixel(width, height, color);
        DynamicImage::ImageRgb8(buffer)
    }

    /// Create a solid color image with alpha
    #[cfg(feature = "image")]
    pub fn create_solid_color_rgba(
        width: u32,
        height: u32,
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    ) -> DynamicImage {
        let color = Rgba([r, g, b, a]);
        let buffer = ImageBuffer::from_pixel(width, height, color);
        DynamicImage::ImageRgba8(buffer)
    }

    /// Combine two images side by side
    #[cfg(feature = "image")]
    pub fn combine_horizontal(left: &DynamicImage, right: &DynamicImage) -> Result<DynamicImage> {
        let (left_width, left_height) = left.dimensions();
        let (right_width, right_height) = right.dimensions();

        let max_height = left_height.max(right_height);
        let total_width = left_width + right_width;

        let mut combined = Self::create_solid_color(total_width, max_height, 255, 255, 255);

        // Overlay left image
        image::imageops::overlay(&mut combined, left, 0i64, 0i64);

        // Overlay right image
        image::imageops::overlay(&mut combined, right, left_width as i64, 0i64);

        Ok(combined)
    }

    /// Combine two images vertically
    #[cfg(feature = "image")]
    pub fn combine_vertical(top: &DynamicImage, bottom: &DynamicImage) -> Result<DynamicImage> {
        let (top_width, top_height) = top.dimensions();
        let (bottom_width, bottom_height) = bottom.dimensions();

        let max_width = top_width.max(bottom_width);
        let total_height = top_height + bottom_height;

        let mut combined = Self::create_solid_color(max_width, total_height, 255, 255, 255);

        // Overlay top image
        image::imageops::overlay(&mut combined, top, 0i64, 0i64);

        // Overlay bottom image
        image::imageops::overlay(&mut combined, bottom, 0i64, top_height as i64);

        Ok(combined)
    }

    /// Add watermark to image
    #[cfg(feature = "image")]
    pub fn add_watermark(
        base: &DynamicImage,
        watermark: &DynamicImage,
        x: u32,
        y: u32,
    ) -> Result<DynamicImage> {
        let (base_width, base_height) = base.dimensions();
        let (watermark_width, watermark_height) = watermark.dimensions();

        if x + watermark_width > base_width || y + watermark_height > base_height {
            return Err(Error::validation(
                "Watermark position exceeds base image dimensions".to_string(),
            ));
        }

        let mut result = base.clone();
        image::imageops::overlay(&mut result, watermark, x as i64, y as i64);

        Ok(result)
    }

    /// Get image histogram
    #[cfg(feature = "image")]
    pub fn histogram(image: &DynamicImage) -> ImageHistogram {
        let rgb_image = image.to_rgb8();
        let mut red = [0u32; 256];
        let mut green = [0u32; 256];
        let mut blue = [0u32; 256];

        for pixel in rgb_image.pixels() {
            red[pixel[0] as usize] += 1;
            green[pixel[1] as usize] += 1;
            blue[pixel[2] as usize] += 1;
        }

        ImageHistogram { red, green, blue }
    }

    /// Detect if an image is mostly dark or light
    #[cfg(feature = "image")]
    pub fn analyze_brightness(image: &DynamicImage) -> BrightnessAnalysis {
        let gray_image = image.to_luma8();
        let pixels = gray_image.pixels();
        let total_pixels = pixels.len();

        let brightness_sum: u64 = pixels.clone().map(|p| p[0] as u64).sum();
        let average_brightness = brightness_sum / total_pixels as u64;

        let dark_pixels = pixels.clone().filter(|p| p[0] < 85).count();
        let bright_pixels = pixels.filter(|p| p[0] > 170).count();

        BrightnessAnalysis {
            average_brightness: average_brightness as u8,
            dark_pixel_percentage: (dark_pixels as f32 / total_pixels as f32) * 100.0,
            bright_pixel_percentage: (bright_pixels as f32 / total_pixels as f32) * 100.0,
            is_mostly_dark: average_brightness < 85,
            is_mostly_bright: average_brightness > 170,
        }
    }
}

/// Image histogram data
#[derive(Debug, Clone)]
pub struct ImageHistogram {
    /// Red channel histogram
    pub red: [u32; 256],
    /// Green channel histogram
    pub green: [u32; 256],
    /// Blue channel histogram
    pub blue: [u32; 256],
}

impl ImageHistogram {
    /// Get the peak value for red channel
    pub fn red_peak(&self) -> (u8, u32) {
        let (index, value) = self
            .red
            .iter()
            .enumerate()
            .max_by_key(|&(_, &v)| v)
            .unwrap();
        (index as u8, *value)
    }

    /// Get the peak value for green channel
    pub fn green_peak(&self) -> (u8, u32) {
        let (index, &value) = self
            .green
            .iter()
            .enumerate()
            .max_by_key(|&(_, &v)| v)
            .unwrap();
        (index as u8, value)
    }

    /// Get the peak value for blue channel
    pub fn blue_peak(&self) -> (u8, u32) {
        let (index, &value) = self
            .blue
            .iter()
            .enumerate()
            .max_by_key(|&(_, &v)| v)
            .unwrap();
        (index as u8, value)
    }
}

/// Brightness analysis result
#[derive(Debug, Clone)]
pub struct BrightnessAnalysis {
    /// Average brightness (0-255)
    pub average_brightness: u8,
    /// Percentage of dark pixels
    pub dark_pixel_percentage: f32,
    /// Percentage of bright pixels
    pub bright_pixel_percentage: f32,
    /// Whether the image is mostly dark
    pub is_mostly_dark: bool,
    /// Whether the image is mostly bright
    pub is_mostly_bright: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_format_extension() {
        assert_eq!(ImageFormat::Png.extension(), "png");
        assert_eq!(ImageFormat::Jpeg.extension(), "jpg");
        assert_eq!(ImageFormat::Gif.extension(), "gif");
    }

    #[test]
    fn test_image_format_from_extension() {
        assert_eq!(ImageFormat::from_extension("png"), Some(ImageFormat::Png));
        assert_eq!(ImageFormat::from_extension("jpg"), Some(ImageFormat::Jpeg));
        assert_eq!(ImageFormat::from_extension("jpeg"), Some(ImageFormat::Jpeg));
        assert_eq!(ImageFormat::from_extension("gif"), Some(ImageFormat::Gif));
        assert_eq!(ImageFormat::from_extension("unknown"), None);
    }

    #[cfg(feature = "image")]
    #[test]
    fn test_create_solid_color() {
        let image = ImageUtil::create_solid_color(100, 100, 255, 0, 0);
        let (width, height) = image.dimensions();
        assert_eq!(width, 100);
        assert_eq!(height, 100);
    }

    #[cfg(feature = "image")]
    #[test]
    fn test_create_solid_color_rgba() {
        let image = ImageUtil::create_solid_color_rgba(50, 50, 0, 255, 0, 128);
        let (width, height) = image.dimensions();
        assert_eq!(width, 50);
        assert_eq!(height, 50);
    }

    #[cfg(feature = "image")]
    #[test]
    fn test_image_transformations() {
        let image = ImageUtil::create_solid_color(100, 100, 128, 128, 128);

        // Test resize
        let resized = ImageUtil::resize(&image, 200, 200, ResizeFilter::Nearest);
        assert_eq!(resized.dimensions(), (200, 200));

        // Test thumbnail
        let thumb = ImageUtil::thumbnail(&image, 50);
        let (thumb_width, thumb_height) = thumb.dimensions();
        assert!(thumb_width <= 50 && thumb_height <= 50);

        // Test grayscale
        let gray = ImageUtil::to_grayscale(&image);
        assert_eq!(gray.dimensions(), image.dimensions());

        // Test rotate
        let rotated = ImageUtil::rotate(&image, RotationAngle::Rotate90);
        assert_eq!(rotated.dimensions(), (100, 100)); // Square so dimensions stay same

        // Test flip
        let flipped_h = ImageUtil::flip_horizontal(&image);
        let flipped_v = ImageUtil::flip_vertical(&image);
        assert_eq!(flipped_h.dimensions(), image.dimensions());
        assert_eq!(flipped_v.dimensions(), image.dimensions());
    }

    #[cfg(feature = "image")]
    #[test]
    fn test_crop() {
        let image = ImageUtil::create_solid_color(100, 100, 255, 255, 255);

        let cropped = ImageUtil::crop(&image, 10, 10, 50, 50);
        assert!(cropped.is_ok());
        let cropped = cropped.unwrap();
        assert_eq!(cropped.dimensions(), (50, 50));

        // Test invalid crop
        let invalid_crop = ImageUtil::crop(&image, 90, 90, 50, 50);
        assert!(invalid_crop.is_err());
    }

    #[cfg(feature = "image")]
    #[test]
    fn test_combine_images() {
        let image1 = ImageUtil::create_solid_color(50, 100, 255, 0, 0);
        let image2 = ImageUtil::create_solid_color(50, 100, 0, 255, 0);

        let horizontal = ImageUtil::combine_horizontal(&image1, &image2);
        assert!(horizontal.is_ok());
        let horizontal = horizontal.unwrap();
        assert_eq!(horizontal.dimensions(), (100, 100));

        let vertical = ImageUtil::combine_vertical(&image1, &image2);
        assert!(vertical.is_ok());
        let vertical = vertical.unwrap();
        assert_eq!(vertical.dimensions(), (50, 200));
    }

    #[cfg(feature = "image")]
    #[test]
    fn test_watermark() {
        let base = ImageUtil::create_solid_color(200, 200, 255, 255, 255);
        let watermark = ImageUtil::create_solid_color(50, 50, 255, 0, 0);

        let result = ImageUtil::add_watermark(&base, &watermark, 10, 10);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.dimensions(), (200, 200));

        // Test invalid position
        let invalid = ImageUtil::add_watermark(&base, &watermark, 180, 180);
        assert!(invalid.is_err());
    }

    #[cfg(feature = "image")]
    #[test]
    fn test_histogram() {
        let image = ImageUtil::create_solid_color(10, 10, 128, 64, 192);
        let histogram = ImageUtil::histogram(&image);

        // For a solid color image, the histogram should have all pixels at the specific color values
        assert_eq!(histogram.red[128], 100); // 10x10 = 100 pixels
        assert_eq!(histogram.green[64], 100);
        assert_eq!(histogram.blue[192], 100);

        let (red_peak_value, red_peak_count) = histogram.red_peak();
        assert_eq!(red_peak_value, 128);
        assert_eq!(red_peak_count, 100);
    }

    #[cfg(feature = "image")]
    #[test]
    fn test_brightness_analysis() {
        // Test dark image
        let dark_image = ImageUtil::create_solid_color(10, 10, 50, 50, 50);
        let dark_analysis = ImageUtil::analyze_brightness(&dark_image);
        assert!(dark_analysis.is_mostly_dark);
        assert!(!dark_analysis.is_mostly_bright);

        // Test bright image
        let bright_image = ImageUtil::create_solid_color(10, 10, 200, 200, 200);
        let bright_analysis = ImageUtil::analyze_brightness(&bright_image);
        assert!(!bright_analysis.is_mostly_dark);
        assert!(bright_analysis.is_mostly_bright);
    }

    #[cfg(feature = "image")]
    #[test]
    fn test_image_effects() {
        let image = ImageUtil::create_solid_color(100, 100, 128, 128, 128);

        // Test brightness adjustment
        let brighter = ImageUtil::adjust_brightness(&image, 50);
        assert_eq!(brighter.dimensions(), image.dimensions());

        // Test contrast adjustment
        let contrasted = ImageUtil::adjust_contrast(&image, 1.5);
        assert_eq!(contrasted.dimensions(), image.dimensions());

        // Test blur
        let blurred = ImageUtil::blur(&image, 1.0);
        assert_eq!(blurred.dimensions(), image.dimensions());

        // Test invert
        let inverted = ImageUtil::invert(&image);
        assert_eq!(inverted.dimensions(), image.dimensions());
    }

    #[cfg(feature = "image")]
    #[test]
    fn test_save_to_bytes() {
        let image = ImageUtil::create_solid_color(10, 10, 255, 0, 0);

        let png_bytes = ImageUtil::save_to_bytes(&image, ImageFormat::Png);
        assert!(png_bytes.is_ok());
        let png_bytes = png_bytes.unwrap();
        assert!(!png_bytes.is_empty());

        let jpeg_bytes = ImageUtil::save_to_bytes(&image, ImageFormat::Jpeg);
        assert!(jpeg_bytes.is_ok());
        let jpeg_bytes = jpeg_bytes.unwrap();
        assert!(!jpeg_bytes.is_empty());
    }
}
