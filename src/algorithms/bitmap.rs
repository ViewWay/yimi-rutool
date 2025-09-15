//! BitMap utilities for efficient bit manipulation
//!
//! This module provides efficient bit manipulation utilities used by bloom filters
//! and other algorithms that need to manage bit arrays.

use std::ops::Index;

/// A memory-efficient bitmap implementation
///
/// Provides efficient bit manipulation operations for use in bloom filters
/// and other data structures that require bit-level operations.
///
/// # Examples
///
/// ```
/// use yimi_rutool::algorithms::BitMap;
///
/// let mut bitmap = BitMap::new(100);
/// bitmap.set(42, true);
/// assert!(bitmap.get(42));
/// assert_eq!(bitmap.count_ones(), 1);
/// ```
#[derive(Debug, Clone)]
pub struct BitMap {
    data: Vec<u64>,
    size: usize,
}

impl BitMap {
    /// Create a new bitmap with the specified number of bits
    ///
    /// # Arguments
    ///
    /// * `size` - Number of bits in the bitmap
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::algorithms::BitMap;
    ///
    /// let bitmap = BitMap::new(1000);
    /// assert_eq!(bitmap.len(), 1000);
    /// ```
    pub fn new(size: usize) -> Self {
        let data_size = (size + 63) / 64; // Round up to nearest 64-bit boundary
        BitMap {
            data: vec![0u64; data_size],
            size,
        }
    }

    /// Create a bitmap filled with ones
    ///
    /// # Arguments
    ///
    /// * `size` - Number of bits in the bitmap
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::algorithms::BitMap;
    ///
    /// let bitmap = BitMap::filled(8);
    /// assert_eq!(bitmap.count_ones(), 8);
    /// ```
    pub fn filled(size: usize) -> Self {
        let mut bitmap = Self::new(size);
        bitmap.fill(true);
        bitmap
    }

    /// Get the value of a bit at the specified index
    ///
    /// # Arguments
    ///
    /// * `index` - Bit index (0-based)
    ///
    /// # Panics
    ///
    /// Panics if index is out of bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::algorithms::BitMap;
    ///
    /// let mut bitmap = BitMap::new(100);
    /// bitmap.set(42, true);
    /// assert!(bitmap.get(42));
    /// assert!(!bitmap.get(43));
    /// ```
    pub fn get(&self, index: usize) -> bool {
        assert!(index < self.size, "Index {} out of bounds for size {}", index, self.size);
        
        let word_index = index / 64;
        let bit_index = index % 64;
        (self.data[word_index] & (1u64 << bit_index)) != 0
    }

    /// Set the value of a bit at the specified index
    ///
    /// # Arguments
    ///
    /// * `index` - Bit index (0-based)
    /// * `value` - Value to set (true for 1, false for 0)
    ///
    /// # Panics
    ///
    /// Panics if index is out of bounds
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::algorithms::BitMap;
    ///
    /// let mut bitmap = BitMap::new(100);
    /// bitmap.set(42, true);
    /// assert!(bitmap.get(42));
    /// 
    /// bitmap.set(42, false);
    /// assert!(!bitmap.get(42));
    /// ```
    pub fn set(&mut self, index: usize, value: bool) {
        assert!(index < self.size, "Index {} out of bounds for size {}", index, self.size);
        
        let word_index = index / 64;
        let bit_index = index % 64;
        
        if value {
            self.data[word_index] |= 1u64 << bit_index;
        } else {
            self.data[word_index] &= !(1u64 << bit_index);
        }
    }

    /// Toggle the bit at the specified index
    ///
    /// # Arguments
    ///
    /// * `index` - Bit index (0-based)
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::algorithms::BitMap;
    ///
    /// let mut bitmap = BitMap::new(100);
    /// bitmap.toggle(42);
    /// assert!(bitmap.get(42));
    /// bitmap.toggle(42);
    /// assert!(!bitmap.get(42));
    /// ```
    pub fn toggle(&mut self, index: usize) {
        assert!(index < self.size, "Index {} out of bounds for size {}", index, self.size);
        
        let word_index = index / 64;
        let bit_index = index % 64;
        self.data[word_index] ^= 1u64 << bit_index;
    }

    /// Fill all bits with the specified value
    ///
    /// # Arguments
    ///
    /// * `value` - Value to set all bits to
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::algorithms::BitMap;
    ///
    /// let mut bitmap = BitMap::new(100);
    /// bitmap.fill(true);
    /// assert_eq!(bitmap.count_ones(), 100);
    /// 
    /// bitmap.fill(false);
    /// assert_eq!(bitmap.count_ones(), 0);
    /// ```
    pub fn fill(&mut self, value: bool) {
        let fill_value = if value { u64::MAX } else { 0 };
        self.data.fill(fill_value);
        
        // Handle partial last word
        if value && self.size % 64 != 0 {
            let last_word_bits = self.size % 64;
            let mask = (1u64 << last_word_bits) - 1;
            if let Some(last_word) = self.data.last_mut() {
                *last_word = mask;
            }
        }
    }

    /// Clear all bits (set to 0)
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::algorithms::BitMap;
    ///
    /// let mut bitmap = BitMap::filled(100);
    /// bitmap.clear();
    /// assert_eq!(bitmap.count_ones(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.fill(false);
    }

    /// Count the number of bits set to 1
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::algorithms::BitMap;
    ///
    /// let mut bitmap = BitMap::new(100);
    /// bitmap.set(10, true);
    /// bitmap.set(20, true);
    /// bitmap.set(30, true);
    /// assert_eq!(bitmap.count_ones(), 3);
    /// ```
    pub fn count_ones(&self) -> usize {
        let mut count = 0;
        
        // Count full words
        for &word in &self.data[..self.data.len().saturating_sub(1)] {
            count += word.count_ones() as usize;
        }
        
        // Handle the last word (might be partial)
        if let Some(&last_word) = self.data.last() {
            let bits_in_last_word = if self.size % 64 == 0 { 64 } else { self.size % 64 };
            let mask = (1u64 << bits_in_last_word) - 1;
            count += (last_word & mask).count_ones() as usize;
        }
        
        count
    }

    /// Count the number of bits set to 0
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::algorithms::BitMap;
    ///
    /// let mut bitmap = BitMap::new(100);
    /// bitmap.set(10, true);
    /// assert_eq!(bitmap.count_zeros(), 99);
    /// ```
    pub fn count_zeros(&self) -> usize {
        self.size - self.count_ones()
    }

    /// Get the number of bits in the bitmap
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::algorithms::BitMap;
    ///
    /// let bitmap = BitMap::new(1000);
    /// assert_eq!(bitmap.len(), 1000);
    /// ```
    pub fn len(&self) -> usize {
        self.size
    }

    /// Check if the bitmap is empty (size 0)
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::algorithms::BitMap;
    ///
    /// let empty_bitmap = BitMap::new(0);
    /// assert!(empty_bitmap.is_empty());
    /// 
    /// let bitmap = BitMap::new(100);
    /// assert!(!bitmap.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Check if all bits are set to 0
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::algorithms::BitMap;
    ///
    /// let mut bitmap = BitMap::new(100);
    /// assert!(bitmap.all_zeros());
    /// 
    /// bitmap.set(50, true);
    /// assert!(!bitmap.all_zeros());
    /// ```
    pub fn all_zeros(&self) -> bool {
        self.count_ones() == 0
    }

    /// Check if all bits are set to 1
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::algorithms::BitMap;
    ///
    /// let mut bitmap = BitMap::new(100);
    /// assert!(!bitmap.all_ones());
    /// 
    /// bitmap.fill(true);
    /// assert!(bitmap.all_ones());
    /// ```
    pub fn all_ones(&self) -> bool {
        self.count_ones() == self.size
    }

    /// Perform bitwise AND operation with another bitmap
    ///
    /// # Arguments
    ///
    /// * `other` - The other bitmap to AND with
    ///
    /// # Panics
    ///
    /// Panics if the bitmaps have different sizes
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::algorithms::BitMap;
    ///
    /// let mut bitmap1 = BitMap::new(100);
    /// let mut bitmap2 = BitMap::new(100);
    /// 
    /// bitmap1.set(10, true);
    /// bitmap1.set(20, true);
    /// bitmap2.set(10, true);
    /// bitmap2.set(30, true);
    /// 
    /// bitmap1.and(&bitmap2);
    /// assert!(bitmap1.get(10));  // Both had this bit set
    /// assert!(!bitmap1.get(20)); // Only bitmap1 had this bit set
    /// assert!(!bitmap1.get(30)); // Only bitmap2 had this bit set
    /// ```
    pub fn and(&mut self, other: &BitMap) {
        assert_eq!(self.size, other.size, "BitMap sizes must match for AND operation");
        
        for (a, &b) in self.data.iter_mut().zip(&other.data) {
            *a &= b;
        }
    }

    /// Perform bitwise OR operation with another bitmap
    ///
    /// # Arguments
    ///
    /// * `other` - The other bitmap to OR with
    ///
    /// # Panics
    ///
    /// Panics if the bitmaps have different sizes
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::algorithms::BitMap;
    ///
    /// let mut bitmap1 = BitMap::new(100);
    /// let mut bitmap2 = BitMap::new(100);
    /// 
    /// bitmap1.set(10, true);
    /// bitmap2.set(20, true);
    /// 
    /// bitmap1.or(&bitmap2);
    /// assert!(bitmap1.get(10)); // From bitmap1
    /// assert!(bitmap1.get(20)); // From bitmap2
    /// ```
    pub fn or(&mut self, other: &BitMap) {
        assert_eq!(self.size, other.size, "BitMap sizes must match for OR operation");
        
        for (a, &b) in self.data.iter_mut().zip(&other.data) {
            *a |= b;
        }
    }

    /// Perform bitwise XOR operation with another bitmap
    ///
    /// # Arguments
    ///
    /// * `other` - The other bitmap to XOR with
    ///
    /// # Panics
    ///
    /// Panics if the bitmaps have different sizes
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::algorithms::BitMap;
    ///
    /// let mut bitmap1 = BitMap::new(100);
    /// let mut bitmap2 = BitMap::new(100);
    /// 
    /// bitmap1.set(10, true);
    /// bitmap1.set(20, true);
    /// bitmap2.set(10, true);
    /// bitmap2.set(30, true);
    /// 
    /// bitmap1.xor(&bitmap2);
    /// assert!(!bitmap1.get(10)); // Both had this bit set
    /// assert!(bitmap1.get(20));  // Only bitmap1 had this bit set
    /// assert!(bitmap1.get(30));  // Only bitmap2 had this bit set
    /// ```
    pub fn xor(&mut self, other: &BitMap) {
        assert_eq!(self.size, other.size, "BitMap sizes must match for XOR operation");
        
        for (a, &b) in self.data.iter_mut().zip(&other.data) {
            *a ^= b;
        }
    }

    /// Perform bitwise NOT operation (invert all bits)
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::algorithms::BitMap;
    ///
    /// let mut bitmap = BitMap::new(100);
    /// bitmap.set(10, true);
    /// 
    /// bitmap.not();
    /// assert!(!bitmap.get(10));
    /// assert_eq!(bitmap.count_ones(), 99);
    /// ```
    pub fn not(&mut self) {
        for word in &mut self.data {
            *word = !*word;
        }
        
        // Clear any bits beyond our size in the last word
        if self.size % 64 != 0 {
            let last_word_bits = self.size % 64;
            let mask = (1u64 << last_word_bits) - 1;
            if let Some(last_word) = self.data.last_mut() {
                *last_word &= mask;
            }
        }
    }

    /// Get an iterator over all set bit indices
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::algorithms::BitMap;
    ///
    /// let mut bitmap = BitMap::new(100);
    /// bitmap.set(10, true);
    /// bitmap.set(20, true);
    /// bitmap.set(30, true);
    /// 
    /// let set_bits: Vec<usize> = bitmap.iter_ones().collect();
    /// assert_eq!(set_bits, vec![10, 20, 30]);
    /// ```
    pub fn iter_ones(&self) -> impl Iterator<Item = usize> + '_ {
        (0..self.size).filter(move |&i| self.get(i))
    }

    /// Get an iterator over all unset bit indices
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::algorithms::BitMap;
    ///
    /// let mut bitmap = BitMap::new(5);
    /// bitmap.set(1, true);
    /// bitmap.set(3, true);
    /// 
    /// let unset_bits: Vec<usize> = bitmap.iter_zeros().collect();
    /// assert_eq!(unset_bits, vec![0, 2, 4]);
    /// ```
    pub fn iter_zeros(&self) -> impl Iterator<Item = usize> + '_ {
        (0..self.size).filter(move |&i| !self.get(i))
    }

    /// Resize the bitmap to a new size
    ///
    /// If the new size is larger, new bits are set to false.
    /// If the new size is smaller, excess bits are discarded.
    ///
    /// # Arguments
    ///
    /// * `new_size` - New size in bits
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::algorithms::BitMap;
    ///
    /// let mut bitmap = BitMap::new(100);
    /// bitmap.set(50, true);
    /// 
    /// bitmap.resize(200);
    /// assert_eq!(bitmap.len(), 200);
    /// assert!(bitmap.get(50)); // Existing data preserved
    /// 
    /// bitmap.resize(25);
    /// assert_eq!(bitmap.len(), 25);
    /// // bit 50 is now out of bounds
    /// ```
    pub fn resize(&mut self, new_size: usize) {
        let new_data_size = (new_size + 63) / 64;
        
        if new_data_size > self.data.len() {
            // Growing: add new words filled with zeros
            self.data.resize(new_data_size, 0);
        } else if new_data_size < self.data.len() {
            // Shrinking: remove excess words
            self.data.truncate(new_data_size);
        }
        
        self.size = new_size;
        
        // Clear any bits beyond our new size in the last word
        if new_size % 64 != 0 {
            let last_word_bits = new_size % 64;
            let mask = (1u64 << last_word_bits) - 1;
            if let Some(last_word) = self.data.last_mut() {
                *last_word &= mask;
            }
        }
    }
}

impl Index<usize> for BitMap {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        // This is a bit tricky since we need to return a reference to a bool
        // We'll use a static reference approach
        if self.get(index) {
            &true
        } else {
            &false
        }
    }
}

// Note: IndexMut is not implemented because we can't return a mutable reference
// to a bit within a word. Use set() method instead.

impl PartialEq for BitMap {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }
        
        // Compare full words
        for (a, b) in self.data.iter().zip(&other.data) {
            if a != b {
                return false;
            }
        }
        
        true
    }
}

impl Eq for BitMap {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitmap_creation() {
        let bitmap = BitMap::new(100);
        assert_eq!(bitmap.len(), 100);
        assert!(!bitmap.is_empty());
        assert!(bitmap.all_zeros());
        assert!(!bitmap.all_ones());
        assert_eq!(bitmap.count_ones(), 0);
        assert_eq!(bitmap.count_zeros(), 100);
    }

    #[test]
    fn test_bitmap_filled() {
        let bitmap = BitMap::filled(100);
        assert_eq!(bitmap.len(), 100);
        assert!(bitmap.all_ones());
        assert!(!bitmap.all_zeros());
        assert_eq!(bitmap.count_ones(), 100);
        assert_eq!(bitmap.count_zeros(), 0);
    }

    #[test]
    fn test_bitmap_empty() {
        let bitmap = BitMap::new(0);
        assert_eq!(bitmap.len(), 0);
        assert!(bitmap.is_empty());
        assert!(bitmap.all_zeros());
        assert!(bitmap.all_ones()); // Vacuously true for empty set
    }

    #[test]
    fn test_set_and_get() {
        let mut bitmap = BitMap::new(100);
        
        bitmap.set(42, true);
        assert!(bitmap.get(42));
        assert!(!bitmap.get(41));
        assert!(!bitmap.get(43));
        
        bitmap.set(42, false);
        assert!(!bitmap.get(42));
    }

    #[test]
    fn test_toggle() {
        let mut bitmap = BitMap::new(100);
        
        bitmap.toggle(42);
        assert!(bitmap.get(42));
        
        bitmap.toggle(42);
        assert!(!bitmap.get(42));
    }

    #[test]
    fn test_fill_and_clear() {
        let mut bitmap = BitMap::new(100);
        
        bitmap.fill(true);
        assert!(bitmap.all_ones());
        assert_eq!(bitmap.count_ones(), 100);
        
        bitmap.clear();
        assert!(bitmap.all_zeros());
        assert_eq!(bitmap.count_ones(), 0);
    }

    #[test]
    fn test_count_operations() {
        let mut bitmap = BitMap::new(100);
        
        bitmap.set(10, true);
        bitmap.set(20, true);
        bitmap.set(30, true);
        
        assert_eq!(bitmap.count_ones(), 3);
        assert_eq!(bitmap.count_zeros(), 97);
    }

    #[test]
    fn test_bitwise_operations() {
        let mut bitmap1 = BitMap::new(100);
        let mut bitmap2 = BitMap::new(100);
        
        bitmap1.set(10, true);
        bitmap1.set(20, true);
        bitmap2.set(10, true);
        bitmap2.set(30, true);
        
        // Test AND
        let mut and_bitmap = bitmap1.clone();
        and_bitmap.and(&bitmap2);
        assert!(and_bitmap.get(10));  // Both had this
        assert!(!and_bitmap.get(20)); // Only bitmap1 had this
        assert!(!and_bitmap.get(30)); // Only bitmap2 had this
        
        // Test OR
        let mut or_bitmap = bitmap1.clone();
        or_bitmap.or(&bitmap2);
        assert!(or_bitmap.get(10));  // Both had this
        assert!(or_bitmap.get(20));  // bitmap1 had this
        assert!(or_bitmap.get(30));  // bitmap2 had this
        
        // Test XOR
        let mut xor_bitmap = bitmap1.clone();
        xor_bitmap.xor(&bitmap2);
        assert!(!xor_bitmap.get(10)); // Both had this (cancel out)
        assert!(xor_bitmap.get(20));  // Only bitmap1 had this
        assert!(xor_bitmap.get(30));  // Only bitmap2 had this
    }

    #[test]
    fn test_not_operation() {
        let mut bitmap = BitMap::new(5);
        bitmap.set(1, true);
        bitmap.set(3, true);
        
        bitmap.not();
        assert!(bitmap.get(0));
        assert!(!bitmap.get(1));
        assert!(bitmap.get(2));
        assert!(!bitmap.get(3));
        assert!(bitmap.get(4));
    }

    #[test]
    fn test_iterators() {
        let mut bitmap = BitMap::new(10);
        bitmap.set(1, true);
        bitmap.set(3, true);
        bitmap.set(7, true);
        
        let ones: Vec<usize> = bitmap.iter_ones().collect();
        assert_eq!(ones, vec![1, 3, 7]);
        
        let zeros: Vec<usize> = bitmap.iter_zeros().collect();
        assert_eq!(zeros, vec![0, 2, 4, 5, 6, 8, 9]);
    }

    #[test]
    fn test_resize() {
        let mut bitmap = BitMap::new(10);
        bitmap.set(5, true);
        bitmap.set(9, true);
        
        // Grow
        bitmap.resize(20);
        assert_eq!(bitmap.len(), 20);
        assert!(bitmap.get(5));  // Preserved
        assert!(bitmap.get(9));  // Preserved
        assert!(!bitmap.get(15)); // New bits are false
        
        // Shrink
        bitmap.resize(8);
        assert_eq!(bitmap.len(), 8);
        assert!(bitmap.get(5));  // Still preserved
        // bitmap.get(9) would panic now (out of bounds)
    }

    #[test]
    fn test_equality() {
        let mut bitmap1 = BitMap::new(100);
        let mut bitmap2 = BitMap::new(100);
        
        assert_eq!(bitmap1, bitmap2);
        
        bitmap1.set(42, true);
        assert_ne!(bitmap1, bitmap2);
        
        bitmap2.set(42, true);
        assert_eq!(bitmap1, bitmap2);
    }

    #[test]
    fn test_edge_cases() {
        // Test bitmap sizes that aren't multiples of 64
        let mut bitmap = BitMap::new(65);
        bitmap.set(64, true);
        assert!(bitmap.get(64));
        assert_eq!(bitmap.count_ones(), 1);
        
        // Test large indices
        let mut large_bitmap = BitMap::new(10000);
        large_bitmap.set(9999, true);
        assert!(large_bitmap.get(9999));
    }

    #[test]
    #[should_panic(expected = "Index 100 out of bounds for size 100")]
    fn test_out_of_bounds_get() {
        let bitmap = BitMap::new(100);
        bitmap.get(100);
    }

    #[test]
    #[should_panic(expected = "Index 100 out of bounds for size 100")]
    fn test_out_of_bounds_set() {
        let mut bitmap = BitMap::new(100);
        bitmap.set(100, true);
    }

    #[test]
    #[should_panic(expected = "BitMap sizes must match for AND operation")]
    fn test_mismatched_sizes_and() {
        let mut bitmap1 = BitMap::new(100);
        let bitmap2 = BitMap::new(200);
        bitmap1.and(&bitmap2);
    }
}
