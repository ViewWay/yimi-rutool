//! Bloom Filter implementations
//!
//! This module provides both standard and counting bloom filter implementations
//! with automatic parameter optimization and multiple hash function support.

use super::bitmap::BitMap;
use super::hash_functions::{HashFunction, Hasher};
use crate::error::{Error, Result};
use std::hash::Hash;

/// A memory-efficient probabilistic data structure for set membership testing
///
/// # Examples
///
/// ```
/// use yimi_rutool::algorithms::{BloomFilter, BloomFilterBuilder};
///
/// let mut bloom = BloomFilterBuilder::new()
///     .expected_items(1000)
///     .false_positive_rate(0.01)
///     .build();
///
/// bloom.insert("hello");
/// assert!(bloom.contains("hello"));
/// ```
#[derive(Debug, Clone)]
pub struct BloomFilter {
    bitmap: BitMap,
    hash_functions: Vec<HashFunction>,
    num_items: usize,
    capacity: usize,
}

impl BloomFilter {
    /// Create a new bloom filter with specified parameters
    ///
    /// # Arguments
    ///
    /// * `capacity` - Expected number of items
    /// * `false_positive_rate` - Desired false positive rate (0.0 to 1.0)
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::algorithms::BloomFilter;
    ///
    /// let bloom = BloomFilter::new(1000, 0.01).unwrap();
    /// ```
    pub fn new(capacity: usize, false_positive_rate: f64) -> Result<Self> {
        if capacity == 0 {
            return Err(Error::custom("Capacity must be greater than 0"));
        }

        if false_positive_rate <= 0.0 || false_positive_rate >= 1.0 {
            return Err(Error::custom("False positive rate must be between 0 and 1"));
        }

        let (bitmap_size, num_hashes) = Self::optimal_parameters(capacity, false_positive_rate);
        let bitmap = BitMap::new(bitmap_size);
        let hash_functions = HashFunction::generate_functions(num_hashes);

        Ok(BloomFilter {
            bitmap,
            hash_functions,
            num_items: 0,
            capacity,
        })
    }

    /// Calculate optimal parameters for the bloom filter
    ///
    /// Returns (bitmap_size, num_hash_functions)
    fn optimal_parameters(capacity: usize, false_positive_rate: f64) -> (usize, usize) {
        // m = -(n * ln(p)) / (ln(2)^2)
        let bitmap_size = (-(capacity as f64 * false_positive_rate.ln()) / (2.0_f64.ln().powi(2)))
            .ceil() as usize;

        // k = (m / n) * ln(2)
        let num_hashes = ((bitmap_size as f64 / capacity as f64) * 2.0_f64.ln()).round() as usize;

        (bitmap_size.max(1), num_hashes.max(1).min(10)) // Limit hash functions to reasonable range
    }

    /// Insert an item into the bloom filter
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::algorithms::BloomFilter;
    ///
    /// let mut bloom = BloomFilter::new(100, 0.01).unwrap();
    /// bloom.insert("hello");
    /// bloom.insert(&42);
    /// ```
    pub fn insert<T: Hash + ?Sized>(&mut self, item: &T) {
        let hashes = self.compute_hashes(item);
        for &hash_value in &hashes {
            let index = hash_value % self.bitmap.len();
            self.bitmap.set(index, true);
        }
        self.num_items += 1;
    }

    /// Test if an item might be in the set
    ///
    /// Returns `true` if the item might be in the set (with possible false positives)
    /// Returns `false` if the item is definitely not in the set
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::algorithms::BloomFilter;
    ///
    /// let mut bloom = BloomFilter::new(100, 0.01).unwrap();
    /// bloom.insert("hello");
    ///
    /// assert!(bloom.contains("hello"));
    /// // This might be true (false positive) or false
    /// let _might_contain = bloom.contains("world");
    /// ```
    pub fn contains<T: Hash + ?Sized>(&self, item: &T) -> bool {
        let hashes = self.compute_hashes(item);
        for &hash_value in &hashes {
            let index = hash_value % self.bitmap.len();
            if !self.bitmap.get(index) {
                return false;
            }
        }
        true
    }

    /// Compute hash values for an item using all hash functions
    fn compute_hashes<T: Hash + ?Sized>(&self, item: &T) -> Vec<usize> {
        self.hash_functions
            .iter()
            .map(|hash_fn| hash_fn.hash(item))
            .collect()
    }

    /// Get the current number of items inserted
    pub fn len(&self) -> usize {
        self.num_items
    }

    /// Check if the bloom filter is empty
    pub fn is_empty(&self) -> bool {
        self.num_items == 0
    }

    /// Get the capacity (expected number of items)
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get the current false positive rate estimate
    pub fn false_positive_rate(&self) -> f64 {
        if self.is_empty() {
            return 0.0;
        }

        let bits_set = self.bitmap.count_ones();
        let total_bits = self.bitmap.len();

        // p = (1 - e^(-kn/m))^k
        let ratio = bits_set as f64 / total_bits as f64;
        ratio.powf(self.hash_functions.len() as f64)
    }

    /// Clear all items from the bloom filter
    pub fn clear(&mut self) {
        self.bitmap.clear();
        self.num_items = 0;
    }

    /// Get the size of the underlying bitmap in bits
    pub fn bitmap_size(&self) -> usize {
        self.bitmap.len()
    }

    /// Get the number of hash functions used
    pub fn num_hash_functions(&self) -> usize {
        self.hash_functions.len()
    }
}

/// Builder for creating bloom filters with custom parameters
///
/// # Examples
///
/// ```
/// use yimi_rutool::algorithms::BloomFilterBuilder;
///
/// let bloom = BloomFilterBuilder::new()
///     .expected_items(1000)
///     .false_positive_rate(0.01)
///     .build();
/// ```
#[derive(Debug)]
pub struct BloomFilterBuilder {
    capacity: Option<usize>,
    false_positive_rate: Option<f64>,
    bitmap_size: Option<usize>,
    num_hash_functions: Option<usize>,
}

impl BloomFilterBuilder {
    /// Create a new bloom filter builder
    pub fn new() -> Self {
        BloomFilterBuilder {
            capacity: None,
            false_positive_rate: None,
            bitmap_size: None,
            num_hash_functions: None,
        }
    }

    /// Set the expected number of items
    pub fn expected_items(mut self, capacity: usize) -> Self {
        self.capacity = Some(capacity);
        self
    }

    /// Set the desired false positive rate
    pub fn false_positive_rate(mut self, rate: f64) -> Self {
        self.false_positive_rate = Some(rate);
        self
    }

    /// Set the bitmap size directly (advanced usage)
    pub fn bitmap_size(mut self, size: usize) -> Self {
        self.bitmap_size = Some(size);
        self
    }

    /// Set the number of hash functions directly (advanced usage)
    pub fn num_hash_functions(mut self, num: usize) -> Self {
        self.num_hash_functions = Some(num);
        self
    }

    /// Build the bloom filter
    pub fn build(self) -> BloomFilter {
        let capacity = self.capacity.unwrap_or(1000);
        let false_positive_rate = self.false_positive_rate.unwrap_or(0.01);

        if let (Some(bitmap_size), Some(num_hashes)) = (self.bitmap_size, self.num_hash_functions) {
            // Manual parameters
            let bitmap = BitMap::new(bitmap_size);
            let hash_functions = HashFunction::generate_functions(num_hashes);

            BloomFilter {
                bitmap,
                hash_functions,
                num_items: 0,
                capacity,
            }
        } else {
            // Auto-calculated parameters
            BloomFilter::new(capacity, false_positive_rate).unwrap_or_else(|_| {
                // Fallback to safe defaults
                let bitmap = BitMap::new(1024);
                let hash_functions = HashFunction::generate_functions(3);
                BloomFilter {
                    bitmap,
                    hash_functions,
                    num_items: 0,
                    capacity: 1000,
                }
            })
        }
    }
}

impl Default for BloomFilterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// A counting bloom filter that supports element removal
///
/// Unlike standard bloom filters, counting bloom filters use counters
/// instead of bits, allowing for element removal operations.
///
/// # Examples
///
/// ```
/// use yimi_rutool::algorithms::CountingBloomFilter;
///
/// let mut filter = CountingBloomFilter::new(1000, 0.01).unwrap();
/// filter.insert("hello");
/// filter.insert("hello"); // Increment counter
///
/// assert!(filter.contains("hello"));
/// filter.remove("hello"); // Decrement counter
/// assert!(filter.contains("hello")); // Still contains (counter > 0)
/// filter.remove("hello"); // Decrement to 0
/// assert!(!filter.contains("hello")); // Now removed
/// ```
#[derive(Debug, Clone)]
pub struct CountingBloomFilter {
    counters: Vec<u32>,
    hash_functions: Vec<HashFunction>,
    num_items: usize,
    capacity: usize,
}

impl CountingBloomFilter {
    /// Create a new counting bloom filter
    pub fn new(capacity: usize, false_positive_rate: f64) -> Result<Self> {
        if capacity == 0 {
            return Err(Error::custom("Capacity must be greater than 0"));
        }

        if false_positive_rate <= 0.0 || false_positive_rate >= 1.0 {
            return Err(Error::custom("False positive rate must be between 0 and 1"));
        }

        let (counter_size, num_hashes) =
            BloomFilter::optimal_parameters(capacity, false_positive_rate);
        let counters = vec![0u32; counter_size];
        let hash_functions = HashFunction::generate_functions(num_hashes);

        Ok(CountingBloomFilter {
            counters,
            hash_functions,
            num_items: 0,
            capacity,
        })
    }

    /// Insert an item into the counting bloom filter
    pub fn insert<T: Hash + ?Sized>(&mut self, item: &T) {
        let hashes = self.compute_hashes(item);
        for &hash_value in &hashes {
            let index = hash_value % self.counters.len();
            self.counters[index] = self.counters[index].saturating_add(1);
        }
        self.num_items += 1;
    }

    /// Remove an item from the counting bloom filter
    ///
    /// Returns `true` if the item was potentially in the filter, `false` otherwise
    pub fn remove<T: Hash + ?Sized>(&mut self, item: &T) -> bool {
        let hashes = self.compute_hashes(item);

        // First check if item might be present
        for &hash_value in &hashes {
            let index = hash_value % self.counters.len();
            if self.counters[index] == 0 {
                return false; // Item definitely not present
            }
        }

        // Decrement counters
        for &hash_value in &hashes {
            let index = hash_value % self.counters.len();
            self.counters[index] = self.counters[index].saturating_sub(1);
        }

        if self.num_items > 0 {
            self.num_items -= 1;
        }

        true
    }

    /// Test if an item might be in the set
    pub fn contains<T: Hash + ?Sized>(&self, item: &T) -> bool {
        let hashes = self.compute_hashes(item);
        for &hash_value in &hashes {
            let index = hash_value % self.counters.len();
            if self.counters[index] == 0 {
                return false;
            }
        }
        true
    }

    /// Compute hash values for an item
    fn compute_hashes<T: Hash + ?Sized>(&self, item: &T) -> Vec<usize> {
        self.hash_functions
            .iter()
            .map(|hash_fn| hash_fn.hash(item))
            .collect()
    }

    /// Get the current number of items inserted
    pub fn len(&self) -> usize {
        self.num_items
    }

    /// Check if the filter is empty
    pub fn is_empty(&self) -> bool {
        self.num_items == 0
    }

    /// Get the capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Clear all items
    pub fn clear(&mut self) {
        self.counters.fill(0);
        self.num_items = 0;
    }

    /// Get the size of the counter array
    pub fn counter_size(&self) -> usize {
        self.counters.len()
    }

    /// Get the number of hash functions used
    pub fn num_hash_functions(&self) -> usize {
        self.hash_functions.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bloom_filter_creation() {
        let bloom = BloomFilter::new(1000, 0.01).unwrap();
        assert_eq!(bloom.len(), 0);
        assert!(bloom.is_empty());
        assert_eq!(bloom.capacity(), 1000);
    }

    #[test]
    fn test_bloom_filter_invalid_params() {
        assert!(BloomFilter::new(0, 0.01).is_err());
        assert!(BloomFilter::new(1000, 0.0).is_err());
        assert!(BloomFilter::new(1000, 1.0).is_err());
    }

    #[test]
    fn test_bloom_filter_insert_and_contains() {
        let mut bloom = BloomFilter::new(100, 0.01).unwrap();

        bloom.insert("hello");
        bloom.insert("world");
        bloom.insert(&42);

        assert!(bloom.contains("hello"));
        assert!(bloom.contains("world"));
        assert!(bloom.contains(&42));
        assert_eq!(bloom.len(), 3);
    }

    #[test]
    fn test_bloom_filter_false_negatives_impossible() {
        let mut bloom = BloomFilter::new(10, 0.1).unwrap();
        let items = vec!["a", "b", "c", "d", "e"];

        for item in &items {
            bloom.insert(item);
        }

        // All inserted items must be found (no false negatives)
        for item in &items {
            assert!(bloom.contains(item));
        }
    }

    #[test]
    fn test_bloom_filter_builder() {
        let bloom = BloomFilterBuilder::new()
            .expected_items(500)
            .false_positive_rate(0.05)
            .build();

        assert_eq!(bloom.capacity(), 500);
        assert!(bloom.is_empty());
    }

    #[test]
    fn test_bloom_filter_builder_manual_params() {
        let bloom = BloomFilterBuilder::new()
            .bitmap_size(1024)
            .num_hash_functions(5)
            .build();

        assert_eq!(bloom.bitmap_size(), 1024);
        assert_eq!(bloom.num_hash_functions(), 5);
    }

    #[test]
    fn test_bloom_filter_clear() {
        let mut bloom = BloomFilter::new(100, 0.01).unwrap();
        bloom.insert("test");
        assert!(!bloom.is_empty());

        bloom.clear();
        assert!(bloom.is_empty());
        assert_eq!(bloom.len(), 0);
    }

    #[test]
    fn test_counting_bloom_filter() {
        let mut filter = CountingBloomFilter::new(100, 0.01).unwrap();

        filter.insert("hello");
        assert!(filter.contains("hello"));
        assert_eq!(filter.len(), 1);

        // Insert same item again
        filter.insert("hello");
        assert!(filter.contains("hello"));
        assert_eq!(filter.len(), 2);

        // Remove once
        assert!(filter.remove("hello"));
        assert!(filter.contains("hello")); // Still there
        assert_eq!(filter.len(), 1);

        // Remove again
        assert!(filter.remove("hello"));
        assert!(!filter.contains("hello")); // Now gone
        assert_eq!(filter.len(), 0);
    }

    #[test]
    fn test_counting_bloom_filter_remove_nonexistent() {
        let mut filter = CountingBloomFilter::new(100, 0.01).unwrap();

        // Try to remove item that was never added
        assert!(!filter.remove("nonexistent"));
        assert_eq!(filter.len(), 0);
    }

    #[test]
    fn test_false_positive_rate_estimation() {
        let mut bloom = BloomFilter::new(100, 0.01).unwrap();

        // Empty filter should have 0 false positive rate
        assert_eq!(bloom.false_positive_rate(), 0.0);

        // Add some items
        for i in 0..50 {
            bloom.insert(&i);
        }

        let fp_rate = bloom.false_positive_rate();
        assert!(fp_rate > 0.0 && fp_rate < 1.0);
    }

    #[test]
    fn test_optimal_parameters() {
        let (m, k) = BloomFilter::optimal_parameters(1000, 0.01);
        assert!(m > 0);
        assert!(k > 0 && k <= 10);

        // Larger capacity should need larger bitmap
        let (m2, _) = BloomFilter::optimal_parameters(10000, 0.01);
        assert!(m2 > m);

        // Lower false positive rate should need larger bitmap
        let (m3, _) = BloomFilter::optimal_parameters(1000, 0.001);
        assert!(m3 > m);
    }
}
