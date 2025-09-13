//! File compression and decompression utilities
//!
//! This module provides functionality for compressing and decompressing files
//! and directories using various compression formats.

use crate::error::{Error, Result};
use std::fs::{File, create_dir_all};
use std::io::{Read, Write};
use std::path::Path;

#[cfg(feature = "zip")]
use zip::{ZipWriter, ZipArchive, write::FileOptions, CompressionMethod};

/// Supported compression formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionFormat {
    /// ZIP format
    Zip,
    /// GZIP format (single file)
    Gzip,
    /// TAR format (uncompressed archive)
    Tar,
    /// TAR.GZ format (compressed archive)
    TarGz,
}

impl CompressionFormat {
    /// Get file extension for this format
    pub fn extension(self) -> &'static str {
        match self {
            Self::Zip => "zip",
            Self::Gzip => "gz",
            Self::Tar => "tar",
            Self::TarGz => "tar.gz",
        }
    }

    /// Detect format from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "zip" => Some(Self::Zip),
            "gz" | "gzip" => Some(Self::Gzip),
            "tar" => Some(Self::Tar),
            "tar.gz" | "tgz" => Some(Self::TarGz),
            _ => None,
        }
    }

    /// Detect format from file path
    pub fn from_path<P: AsRef<Path>>(path: P) -> Option<Self> {
        let path_str = path.as_ref().to_string_lossy().to_lowercase();
        
        if path_str.ends_with(".tar.gz") || path_str.ends_with(".tgz") {
            Some(Self::TarGz)
        } else if let Some(ext) = path.as_ref().extension() {
            Self::from_extension(&ext.to_string_lossy())
        } else {
            None
        }
    }
}

/// Compression level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionLevel {
    /// No compression (fastest)
    None,
    /// Fastest compression
    Fastest,
    /// Balanced compression and speed
    Balanced,
    /// Best compression (slowest)
    Best,
}

impl CompressionLevel {
    /// Convert to ZIP compression level
    #[cfg(feature = "zip")]
    fn to_zip_level(self) -> i32 {
        match self {
            Self::None => 0,
            Self::Fastest => 1,
            Self::Balanced => 6,
            Self::Best => 9,
        }
    }
}

/// Compression statistics
#[derive(Debug, Clone)]
pub struct CompressionStats {
    /// Original size in bytes
    pub original_size: u64,
    /// Compressed size in bytes
    pub compressed_size: u64,
    /// Compression ratio (compressed_size / original_size)
    pub compression_ratio: f64,
    /// Space saved in bytes
    pub space_saved: u64,
    /// Space saved percentage
    pub space_saved_percentage: f64,
    /// Number of files processed
    pub file_count: usize,
}

impl CompressionStats {
    /// Create new compression statistics
    pub fn new(original_size: u64, compressed_size: u64, file_count: usize) -> Self {
        let compression_ratio = if original_size > 0 {
            compressed_size as f64 / original_size as f64
        } else {
            0.0
        };
        
        let space_saved = original_size.saturating_sub(compressed_size);
        let space_saved_percentage = if original_size > 0 {
            (space_saved as f64 / original_size as f64) * 100.0
        } else {
            0.0
        };

        Self {
            original_size,
            compressed_size,
            compression_ratio,
            space_saved,
            space_saved_percentage,
            file_count,
        }
    }
}

impl std::fmt::Display for CompressionStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Compression Statistics:")?;
        writeln!(f, "  Files processed: {}", self.file_count)?;
        writeln!(f, "  Original size: {} bytes", self.original_size)?;
        writeln!(f, "  Compressed size: {} bytes", self.compressed_size)?;
        writeln!(f, "  Compression ratio: {:.2}", self.compression_ratio)?;
        writeln!(f, "  Space saved: {} bytes ({:.1}%)", self.space_saved, self.space_saved_percentage)?;
        Ok(())
    }
}

/// Compression utility
pub struct CompressionUtil;

impl CompressionUtil {
    /// Compress a file or directory to ZIP format
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::extra::{CompressionUtil, CompressionLevel};
    ///
    /// // let stats = CompressionUtil::compress_zip(
    /// //     "source_directory",
    /// //     "output.zip",
    /// //     CompressionLevel::Balanced
    /// // ).unwrap();
    /// ```
    #[cfg(feature = "zip")]
    pub fn compress_zip<P: AsRef<Path>, Q: AsRef<Path>>(
        source: P,
        destination: Q,
        level: CompressionLevel,
    ) -> Result<CompressionStats> {
        let source_path = source.as_ref();
        let destination_path = destination.as_ref();

        if !source_path.exists() {
            return Err(Error::not_found(format!("Source path does not exist: {:?}", source_path)));
        }

        let file = File::create(destination_path)
            .map_err(|e| Error::validation(format!("Failed to create ZIP file: {}", e)))?;
        
        let mut zip = ZipWriter::new(file);
        let options = FileOptions::<()>::default()
            .compression_method(CompressionMethod::Deflated)
            .compression_level(Some(level.to_zip_level() as i64));

        let mut original_size = 0u64;
        let mut file_count = 0usize;

        if source_path.is_file() {
            // Compress single file
            let file_name = source_path.file_name()
                .ok_or_else(|| Error::validation("Invalid file name".to_string()))?
                .to_string_lossy();
            
            zip.start_file(&file_name, options)
                .map_err(|e| Error::validation(format!("Failed to start ZIP file entry: {e}")))?;

            let mut source_file = File::open(source_path)
                .map_err(|e| Error::validation(format!("Failed to open source file: {}", e)))?;
            
            let mut buffer = Vec::new();
            source_file.read_to_end(&mut buffer)
                .map_err(|e| Error::validation(format!("Failed to read source file: {}", e)))?;
            
            original_size += buffer.len() as u64;
            file_count += 1;

            zip.write_all(&buffer)
                .map_err(|e| Error::validation(format!("Failed to write to ZIP: {e}")))?;
        } else if source_path.is_dir() {
            // Compress directory
            Self::compress_directory_to_zip(&mut zip, source_path, source_path, options, &mut original_size, &mut file_count)?;
        }

        let zip_file = zip.finish()
            .map_err(|e| Error::validation(format!("Failed to finish ZIP file: {}", e)))?;
        
        let compressed_size = zip_file.metadata()
            .map_err(|e| Error::validation(format!("Failed to get ZIP file metadata: {}", e)))?
            .len();

        Ok(CompressionStats::new(original_size, compressed_size, file_count))
    }

    /// Decompress a ZIP file
    #[cfg(feature = "zip")]
    pub fn decompress_zip<P: AsRef<Path>, Q: AsRef<Path>>(
        source: P,
        destination: Q,
    ) -> Result<CompressionStats> {
        let source_path = source.as_ref();
        let destination_path = destination.as_ref();

        if !source_path.exists() {
            return Err(Error::not_found(format!("ZIP file does not exist: {:?}", source_path)));
        }

        let file = File::open(source_path)
            .map_err(|e| Error::validation(format!("Failed to open ZIP file: {e}")))?;
        
        let mut archive = ZipArchive::new(file)
            .map_err(|e| Error::validation(format!("Failed to read ZIP archive: {e}")))?;

        let mut original_size = 0u64;
        let mut compressed_size = 0u64;
        let file_count = archive.len();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)
                .map_err(|e| Error::validation(format!("Failed to read ZIP entry {i}: {e}")))?;
            
            let outpath = destination_path.join(file.name());
            
            compressed_size += file.compressed_size();
            original_size += file.size();

            if file.name().ends_with('/') {
                // Directory
                create_dir_all(&outpath)
                    .map_err(|e| Error::validation(format!("Failed to create directory: {}", e)))?;
            } else {
                // File
                if let Some(parent) = outpath.parent() {
                    create_dir_all(parent)
                        .map_err(|e| Error::validation(format!("Failed to create parent directory: {}", e)))?;
                }
                
                let mut outfile = File::create(&outpath)
                    .map_err(|e| Error::validation(format!("Failed to create output file: {e}")))?;
                
                std::io::copy(&mut file, &mut outfile)
                    .map_err(|e| Error::validation(format!("Failed to extract file: {e}")))?;
            }
        }

        Ok(CompressionStats::new(original_size, compressed_size, file_count))
    }

    /// Compress data to GZIP format
    #[cfg(feature = "flate2")]
    pub fn compress_gzip(data: &[u8]) -> Result<Vec<u8>> {
        use flate2::{Compression, write::GzEncoder};
        
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data)
            .map_err(|e| Error::validation(format!("Failed to compress with GZIP: {}", e)))?;
        
        encoder.finish()
            .map_err(|e| Error::validation(format!("Failed to finish GZIP compression: {}", e)))
    }

    /// Decompress GZIP data
    #[cfg(feature = "flate2")]
    pub fn decompress_gzip(data: &[u8]) -> Result<Vec<u8>> {
        use flate2::read::GzDecoder;
        
        let mut decoder = GzDecoder::new(data);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)
            .map_err(|e| Error::validation(format!("Failed to decompress GZIP: {}", e)))?;
        
        Ok(decompressed)
    }

    /// Compress a file with GZIP
    #[cfg(feature = "flate2")]
    pub fn compress_file_gzip<P: AsRef<Path>, Q: AsRef<Path>>(
        source: P,
        destination: Q,
    ) -> Result<CompressionStats> {
        use flate2::{Compression, write::GzEncoder};
        
        let source_path = source.as_ref();
        let destination_path = destination.as_ref();

        if !source_path.exists() {
            return Err(Error::not_found(format!("Source file does not exist: {:?}", source_path)));
        }

        let mut source_file = File::open(source_path)
            .map_err(|e| Error::validation(format!("Failed to open source file: {}", e)))?;
        
        let destination_file = File::create(destination_path)
            .map_err(|e| Error::validation(format!("Failed to create destination file: {e}")))?;
        
        let mut encoder = GzEncoder::new(destination_file, Compression::default());
        
        let original_size = std::io::copy(&mut source_file, &mut encoder)
            .map_err(|e| Error::validation(format!("Failed to compress file: {}", e)))?;
        
        encoder.finish()
            .map_err(|e| Error::validation(format!("Failed to finish compression: {}", e)))?;

        let compressed_size = std::fs::metadata(destination_path)
            .map_err(|e| Error::validation(format!("Failed to get compressed file size: {}", e)))?
            .len();

        Ok(CompressionStats::new(original_size, compressed_size, 1))
    }

    /// Decompress a GZIP file
    #[cfg(feature = "flate2")]
    pub fn decompress_file_gzip<P: AsRef<Path>, Q: AsRef<Path>>(
        source: P,
        destination: Q,
    ) -> Result<CompressionStats> {
        use flate2::read::GzDecoder;
        
        let source_path = source.as_ref();
        let destination_path = destination.as_ref();

        if !source_path.exists() {
            return Err(Error::not_found(format!("GZIP file does not exist: {source_path:?}")));
        }

        let source_file = File::open(source_path)
            .map_err(|e| Error::validation(format!("Failed to open GZIP file: {e}")))?;
        
        let mut decoder = GzDecoder::new(source_file);
        
        let mut destination_file = File::create(destination_path)
            .map_err(|e| Error::validation(format!("Failed to create destination file: {e}")))?;
        
        let original_size = std::io::copy(&mut decoder, &mut destination_file)
            .map_err(|e| Error::validation(format!("Failed to decompress file: {e}")))?;

        let compressed_size = std::fs::metadata(source_path)
            .map_err(|e| Error::validation(format!("Failed to get source file size: {e}")))?
            .len();

        Ok(CompressionStats::new(original_size, compressed_size, 1))
    }

    /// Recursively compress a directory to ZIP
    #[cfg(feature = "zip")]
    fn compress_directory_to_zip(
        zip: &mut ZipWriter<File>,
        base_path: &Path,
        current_path: &Path,
        options: FileOptions<()>,
        total_size: &mut u64,
        file_count: &mut usize,
    ) -> Result<()> {
        let entries = std::fs::read_dir(current_path)
            .map_err(|e| Error::validation(format!("Failed to read directory: {e}")))?;

        for entry in entries {
            let entry = entry
                .map_err(|e| Error::validation(format!("Failed to read directory entry: {e}")))?;
            let path = entry.path();
            
            let relative_path = path.strip_prefix(base_path)
                .map_err(|e| Error::validation(format!("Failed to create relative path: {e}")))?;
            
            let name = relative_path.to_string_lossy().replace('\\', "/");

            if path.is_file() {
                zip.start_file(&name, options)
                    .map_err(|e| Error::validation(format!("Failed to start ZIP file entry: {e}")))?;

                let mut file = File::open(&path)
                    .map_err(|e| Error::validation(format!("Failed to open file: {e}")))?;
                
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)
                    .map_err(|e| Error::validation(format!("Failed to read file: {e}")))?;
                
                *total_size += buffer.len() as u64;
                *file_count += 1;

                zip.write_all(&buffer)
                    .map_err(|e| Error::validation(format!("Failed to write to ZIP: {e}")))?;
            } else if path.is_dir() {
                // Add directory entry
                zip.add_directory(format!("{name}/"), options)
                    .map_err(|e| Error::validation(format!("Failed to add directory to ZIP: {e}")))?;
                
                // Recursively process subdirectory
                Self::compress_directory_to_zip(zip, base_path, &path, options, total_size, file_count)?;
            }
        }

        Ok(())
    }

    /// Get information about a ZIP file without extracting it
    /// 
    /// # Errors
    /// 
    /// Returns `Error` if:
    /// - ZIP file cannot be opened
    /// - ZIP archive cannot be read
    /// - Entry information cannot be accessed
    #[cfg(feature = "zip")]
    pub fn zip_info<P: AsRef<Path>>(path: P) -> Result<ZipInfo> {
        let file = File::open(&path)
            .map_err(|e| Error::validation(format!("Failed to open ZIP file: {e}")))?;
        
        let mut archive = ZipArchive::new(file)
            .map_err(|e| Error::validation(format!("Failed to read ZIP archive: {e}")))?;

        let mut files = Vec::new();
        let mut total_compressed_size = 0u64;
        let mut total_uncompressed_size = 0u64;

        for i in 0..archive.len() {
            let file = archive.by_index(i)
                .map_err(|e| Error::validation(format!("Failed to read ZIP entry {i}: {e}")))?;
            
            let info = ZipFileInfo {
                name: file.name().to_string(),
                compressed_size: file.compressed_size(),
                uncompressed_size: file.size(),
                is_directory: file.name().ends_with('/'),
                compression_method: format!("{:?}", file.compression()),
            };

            total_compressed_size += file.compressed_size();
            total_uncompressed_size += file.size();
            files.push(info);
        }

        Ok(ZipInfo {
            file_count: archive.len(),
            total_compressed_size,
            total_uncompressed_size,
            compression_ratio: if total_uncompressed_size > 0 {
                total_compressed_size as f64 / total_uncompressed_size as f64
            } else {
                0.0
            },
            files,
        })
    }

    /// List files in a ZIP archive
    #[cfg(feature = "zip")]
    pub fn list_zip<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
        let file = File::open(&path)
            .map_err(|e| Error::validation(format!("Failed to open ZIP file: {e}")))?;
        
        let mut archive = ZipArchive::new(file)
            .map_err(|e| Error::validation(format!("Failed to read ZIP archive: {e}")))?;

        let mut files = Vec::new();
        for i in 0..archive.len() {
            let file = archive.by_index(i)
                .map_err(|e| Error::validation(format!("Failed to read ZIP entry {i}: {e}")))?;
            files.push(file.name().to_string());
        }

        Ok(files)
    }

    /// Extract a specific file from a ZIP archive
    #[cfg(feature = "zip")]
    pub fn extract_file_from_zip<P: AsRef<Path>, Q: AsRef<Path>>(
        zip_path: P,
        file_name: &str,
        destination: Q,
    ) -> Result<()> {
        let file = File::open(&zip_path)
            .map_err(|e| Error::validation(format!("Failed to open ZIP file: {e}")))?;
        
        let mut archive = ZipArchive::new(file)
            .map_err(|e| Error::validation(format!("Failed to read ZIP archive: {e}")))?;

        let mut zip_file = archive.by_name(file_name)
            .map_err(|e| Error::not_found(format!("File '{file_name}' not found in ZIP: {e}")))?;

        let mut output_file = File::create(destination)
            .map_err(|e| Error::validation(format!("Failed to create output file: {e}")))?;

        std::io::copy(&mut zip_file, &mut output_file)
            .map_err(|e| Error::validation(format!("Failed to extract file: {e}")))?;

        Ok(())
    }

    /// Calculate compression ratio for a file
    pub fn calculate_compression_ratio(original_size: u64, compressed_size: u64) -> f64 {
        if original_size > 0 {
            compressed_size as f64 / original_size as f64
        } else {
            0.0
        }
    }

    /// Estimate compression ratio for different formats
    pub fn estimate_compression_ratio(file_type: &str) -> f64 {
        match file_type.to_lowercase().as_str() {
            "txt" | "csv" | "json" | "xml" | "html" | "css" | "js" => 0.2, // Text files compress very well
            "log" => 0.1, // Log files often have repetitive content
            "sql" => 0.3,
            "pdf" => 0.9, // PDFs are already compressed
            "jpg" | "jpeg" | "png" | "gif" | "webp" => 0.95, // Images are already compressed
            "mp3" | "mp4" | "avi" | "mkv" => 0.98, // Media files are already compressed
            "zip" | "rar" | "7z" | "gz" | "bz2" => 0.99, // Already compressed archives
            "exe" | "dll" | "so" => 0.7, // Binary files
            "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" => 0.6, // Office documents
            _ => 0.5, // Default estimate
        }
    }
}

/// Information about a ZIP archive
#[derive(Debug, Clone)]
pub struct ZipInfo {
    /// Number of files in the archive
    pub file_count: usize,
    /// Total compressed size
    pub total_compressed_size: u64,
    /// Total uncompressed size
    pub total_uncompressed_size: u64,
    /// Overall compression ratio
    pub compression_ratio: f64,
    /// List of files in the archive
    pub files: Vec<ZipFileInfo>,
}

/// Information about a file in a ZIP archive
#[derive(Debug, Clone)]
pub struct ZipFileInfo {
    /// File name
    pub name: String,
    /// Compressed size
    pub compressed_size: u64,
    /// Uncompressed size
    pub uncompressed_size: u64,
    /// Whether this is a directory
    pub is_directory: bool,
    /// Compression method used
    pub compression_method: String,
}

impl std::fmt::Display for ZipInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ZIP Archive Information:")?;
        writeln!(f, "  Files: {}", self.file_count)?;
        writeln!(f, "  Compressed size: {} bytes", self.total_compressed_size)?;
        writeln!(f, "  Uncompressed size: {} bytes", self.total_uncompressed_size)?;
        writeln!(f, "  Compression ratio: {:.2}", self.compression_ratio)?;
        writeln!(f, "  Space saved: {:.1}%", (1.0 - self.compression_ratio) * 100.0)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_format_extension() {
        assert_eq!(CompressionFormat::Zip.extension(), "zip");
        assert_eq!(CompressionFormat::Gzip.extension(), "gz");
        assert_eq!(CompressionFormat::Tar.extension(), "tar");
        assert_eq!(CompressionFormat::TarGz.extension(), "tar.gz");
    }

    #[test]
    fn test_compression_format_from_extension() {
        assert_eq!(CompressionFormat::from_extension("zip"), Some(CompressionFormat::Zip));
        assert_eq!(CompressionFormat::from_extension("gz"), Some(CompressionFormat::Gzip));
        assert_eq!(CompressionFormat::from_extension("tar"), Some(CompressionFormat::Tar));
        assert_eq!(CompressionFormat::from_extension("unknown"), None);
    }

    #[test]
    fn test_compression_format_from_path() {
        assert_eq!(CompressionFormat::from_path("test.zip"), Some(CompressionFormat::Zip));
        assert_eq!(CompressionFormat::from_path("test.tar.gz"), Some(CompressionFormat::TarGz));
        assert_eq!(CompressionFormat::from_path("test.tgz"), Some(CompressionFormat::TarGz));
        assert_eq!(CompressionFormat::from_path("test.txt"), None);
    }

    #[test]
    fn test_compression_stats() {
        let stats = CompressionStats::new(1000, 600, 5);
        assert_eq!(stats.original_size, 1000);
        assert_eq!(stats.compressed_size, 600);
        assert_eq!(stats.compression_ratio, 0.6);
        assert_eq!(stats.space_saved, 400);
        assert_eq!(stats.space_saved_percentage, 40.0);
        assert_eq!(stats.file_count, 5);
    }

    #[test]
    fn test_compression_stats_display() {
        let stats = CompressionStats::new(1000, 600, 3);
        let display = stats.to_string();
        assert!(display.contains("Files processed: 3"));
        assert!(display.contains("Original size: 1000 bytes"));
        assert!(display.contains("Compressed size: 600 bytes"));
        assert!(display.contains("40.0%"));
    }

    #[test]
    fn test_calculate_compression_ratio() {
        assert_eq!(CompressionUtil::calculate_compression_ratio(1000, 600), 0.6);
        assert_eq!(CompressionUtil::calculate_compression_ratio(0, 100), 0.0);
        assert_eq!(CompressionUtil::calculate_compression_ratio(100, 0), 0.0);
    }

    #[test]
    fn test_estimate_compression_ratio() {
        assert!(CompressionUtil::estimate_compression_ratio("txt") < 0.5);
        assert!(CompressionUtil::estimate_compression_ratio("json") < 0.5);
        assert!(CompressionUtil::estimate_compression_ratio("jpg") > 0.9);
        assert!(CompressionUtil::estimate_compression_ratio("mp3") > 0.9);
        assert!(CompressionUtil::estimate_compression_ratio("zip") > 0.9);
        assert_eq!(CompressionUtil::estimate_compression_ratio("unknown"), 0.5);
    }

    #[cfg(feature = "flate2")]
    #[test]
    fn test_gzip_compression() {
        let data = b"Hello, World! This is a test string for compression.".repeat(100);
        
        let compressed = CompressionUtil::compress_gzip(&data).unwrap();
        assert!(compressed.len() < data.len());
        
        let decompressed = CompressionUtil::decompress_gzip(&compressed).unwrap();
        assert_eq!(decompressed, data);
    }

    // Note: ZIP tests would require creating temporary files and directories
    // These are more complex integration tests that would be better suited
    // for a separate test module with proper setup and teardown
}
