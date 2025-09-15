//! Text processing utilities
//!
//! This module provides advanced text processing capabilities including:
//! - Sensitive word filtering using DFA (Deterministic Finite Automaton)
//! - Word replacement strategies
//! - Batch text processing
//! - Performance optimized text analysis
//!
//! # Features
//!
//! - **DFA Sensitive Word Filter**: High-performance sensitive word detection and filtering
//! - **Flexible Replacement**: Multiple replacement strategies (mask, replace, highlight)
//! - **Batch Processing**: Efficient processing of large text datasets
//! - **Custom Word Lists**: Support for custom sensitive word dictionaries
//!
//! # Quick Start
//!
//! ```rust
//! use yimi_rutool::text::{SensitiveWordFilter, FilterBuilder};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a filter with some sensitive words
//! let mut filter = FilterBuilder::new()
//!     .add_word("badword")
//!     .add_word("sensitive")
//!     .build();
//!
//! // Filter text
//! let text = "This contains a badword in it";
//! let filtered = filter.filter(text);
//! println!("Filtered: {}", filtered); // "This contains a *** in it"
//! # Ok(())
//! # }
//! ```

pub mod sensitive;

// Re-export main types for convenience
pub use sensitive::{
    SensitiveWordFilter, FilterBuilder, FilterStrategy, FilterResult,
    WordMatch, ProcessingStats
};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_module_imports() {
        // Basic test to ensure all imports work
        let _filter = FilterBuilder::new().build();
    }
}
