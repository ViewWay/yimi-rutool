//! Algorithms module for yimi-rutool
//!
//! This module provides various algorithms and data structures including:
//! - Bloom filters (standard and counting)
//! - Bitmap utilities
//! - Hash functions
//! - Parameter optimization utilities
//!
//! # Features
//!
//! - **Bloom Filters**: Memory-efficient probabilistic data structures for set membership testing
//! - **Counting Bloom Filters**: Enhanced bloom filters supporting element removal
//! - **Bitmap**: Efficient bit manipulation utilities
//! - **Hash Functions**: Multiple hash algorithms for optimal distribution
//!
//! # Quick Start
//!
//! ```rust
//! use yimi_rutool::algorithms::{BloomFilter, BloomFilterBuilder};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a bloom filter with 1000 expected items and 1% false positive rate
//! let mut bloom = BloomFilterBuilder::new()
//!     .expected_items(1000)
//!     .false_positive_rate(0.01)
//!     .build();
//!
//! // Add items
//! bloom.insert("hello");
//! bloom.insert("world");
//!
//! // Test membership
//! assert!(bloom.contains("hello"));
//! assert!(!bloom.contains("not_exists")); // might be false positive
//! # Ok(())
//! # }
//! ```

pub mod bitmap;
pub mod bloom_filter;
pub mod hash_functions;

// Re-export main types for convenience
pub use bitmap::BitMap;
pub use bloom_filter::{BloomFilter, BloomFilterBuilder, CountingBloomFilter};
pub use hash_functions::{HashFunction, Hasher};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_imports() {
        // Basic test to ensure all imports work
        let _bloom = BloomFilterBuilder::new().build();
    }
}
