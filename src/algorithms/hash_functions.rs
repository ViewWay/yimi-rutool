//! Hash function implementations for bloom filters
//!
//! This module provides various hash function implementations optimized
//! for use in bloom filters and other probabilistic data structures.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher as StdHasher};

/// Trait for hash functions used in bloom filters
pub trait Hasher {
    /// Compute hash value for the given item
    fn hash<T: Hash + ?Sized>(&self, item: &T) -> usize;
}

/// Different hash function algorithms available
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashAlgorithm {
    /// Default Rust hasher (SipHash)
    Default,
    /// Murmur3 hash (32-bit)
    Murmur3,
    /// FNV-1a hash
    Fnv1a,
    /// Custom seeded hash
    Seeded(u64),
}

/// A hash function with configurable algorithm and seed
#[derive(Debug, Clone)]
pub struct HashFunction {
    algorithm: HashAlgorithm,
    seed: u64,
}

impl HashFunction {
    /// Create a new hash function with the default algorithm
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::algorithms::hash_functions::{HashFunction, Hasher};
    ///
    /// let hash_fn = HashFunction::new();
    /// let hash_value = hash_fn.hash("hello");
    /// assert!(hash_value > 0);
    /// ```
    pub fn new() -> Self {
        HashFunction {
            algorithm: HashAlgorithm::Default,
            seed: 0,
        }
    }

    /// Create a new hash function with a specific algorithm
    ///
    /// # Arguments
    ///
    /// * `algorithm` - The hash algorithm to use
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::algorithms::hash_functions::{HashFunction, HashAlgorithm, Hasher};
    ///
    /// let hash_fn = HashFunction::with_algorithm(HashAlgorithm::Murmur3);
    /// let hash_value = hash_fn.hash("hello");
    /// assert!(hash_value > 0);
    /// ```
    pub fn with_algorithm(algorithm: HashAlgorithm) -> Self {
        HashFunction { algorithm, seed: 0 }
    }

    /// Create a new hash function with a specific algorithm and seed
    ///
    /// # Arguments
    ///
    /// * `algorithm` - The hash algorithm to use
    /// * `seed` - Seed value for the hash function
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::algorithms::hash_functions::{HashFunction, HashAlgorithm, Hasher};
    ///
    /// let hash_fn = HashFunction::with_seed(HashAlgorithm::Fnv1a, 12345);
    /// let hash_value = hash_fn.hash("hello");
    /// assert!(hash_value > 0);
    /// ```
    pub fn with_seed(algorithm: HashAlgorithm, seed: u64) -> Self {
        HashFunction { algorithm, seed }
    }

    /// Generate multiple hash functions with different seeds
    ///
    /// This is useful for bloom filters that need multiple independent hash functions.
    ///
    /// # Arguments
    ///
    /// * `count` - Number of hash functions to generate
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::algorithms::hash_functions::HashFunction;
    ///
    /// let hash_functions = HashFunction::generate_functions(5);
    /// assert_eq!(hash_functions.len(), 5);
    /// ```
    pub fn generate_functions(count: usize) -> Vec<HashFunction> {
        let algorithms = [
            HashAlgorithm::Default,
            HashAlgorithm::Murmur3,
            HashAlgorithm::Fnv1a,
        ];
        
        let mut functions = Vec::with_capacity(count);
        
        for i in 0..count {
            let algorithm = algorithms[i % algorithms.len()];
            let seed = (i as u64).wrapping_mul(0x9e3779b97f4a7c15_u64); // Golden ratio multiplier
            functions.push(HashFunction::with_seed(algorithm, seed));
        }
        
        functions
    }

    /// Get the algorithm used by this hash function
    pub fn algorithm(&self) -> HashAlgorithm {
        self.algorithm
    }

    /// Get the seed used by this hash function
    pub fn seed(&self) -> u64 {
        self.seed
    }
}

impl Default for HashFunction {
    fn default() -> Self {
        Self::new()
    }
}

impl Hasher for HashFunction {
    fn hash<T: Hash + ?Sized>(&self, item: &T) -> usize {
        match self.algorithm {
            HashAlgorithm::Default => {
                let mut hasher = DefaultHasher::new();
                self.seed.hash(&mut hasher);
                item.hash(&mut hasher);
                hasher.finish() as usize
            }
            HashAlgorithm::Murmur3 => {
                murmur3_hash(item, self.seed as u32)
            }
            HashAlgorithm::Fnv1a => {
                fnv1a_hash(item, self.seed)
            }
            HashAlgorithm::Seeded(base_seed) => {
                let mut hasher = DefaultHasher::new();
                (base_seed ^ self.seed).hash(&mut hasher);
                item.hash(&mut hasher);
                hasher.finish() as usize
            }
        }
    }
}

/// Murmur3 32-bit hash implementation
///
/// This is a simplified version of MurmurHash3_x86_32 optimized for speed.
fn murmur3_hash<T: Hash + ?Sized>(item: &T, seed: u32) -> usize {
    // Convert the item to bytes using a hasher
    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    item.hash(&mut hasher);
    let hash64 = hasher.finish();
    
    // Apply Murmur3 finalizer to improve distribution
    let mut h = hash64 as u32;
    h ^= h >> 16;
    h = h.wrapping_mul(0x85ebca6b);
    h ^= h >> 13;
    h = h.wrapping_mul(0xc2b2ae35);
    h ^= h >> 16;
    
    h as usize
}

/// FNV-1a hash implementation
///
/// Fast hash function with good distribution properties.
fn fnv1a_hash<T: Hash + ?Sized>(item: &T, seed: u64) -> usize {
    const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;
    
    // Start with offset basis XORed with seed
    let mut hash = FNV_OFFSET_BASIS ^ seed;
    
    // Hash the item to get bytes
    let mut hasher = DefaultHasher::new();
    item.hash(&mut hasher);
    let item_hash = hasher.finish();
    
    // Apply FNV-1a algorithm to the item hash bytes
    let bytes = item_hash.to_le_bytes();
    for &byte in &bytes {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    
    hash as usize
}

/// Double hashing strategy for generating multiple hash values
///
/// Uses the formula: h_i(x) = (h1(x) + i * h2(x)) mod m
/// where h1 and h2 are two independent hash functions.
pub struct DoubleHasher {
    hasher1: HashFunction,
    hasher2: HashFunction,
}

impl DoubleHasher {
    /// Create a new double hasher
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::algorithms::hash_functions::DoubleHasher;
    ///
    /// let double_hasher = DoubleHasher::new();
    /// let hashes = double_hasher.hash_multiple("hello", 5, 1000);
    /// assert_eq!(hashes.len(), 5);
    /// ```
    pub fn new() -> Self {
        DoubleHasher {
            hasher1: HashFunction::with_algorithm(HashAlgorithm::Default),
            hasher2: HashFunction::with_algorithm(HashAlgorithm::Murmur3),
        }
    }

    /// Create a double hasher with specific algorithms
    ///
    /// # Arguments
    ///
    /// * `algo1` - First hash algorithm
    /// * `algo2` - Second hash algorithm
    pub fn with_algorithms(algo1: HashAlgorithm, algo2: HashAlgorithm) -> Self {
        DoubleHasher {
            hasher1: HashFunction::with_algorithm(algo1),
            hasher2: HashFunction::with_algorithm(algo2),
        }
    }

    /// Generate multiple hash values using double hashing
    ///
    /// # Arguments
    ///
    /// * `item` - Item to hash
    /// * `count` - Number of hash values to generate
    /// * `max_value` - Maximum value for hash results (modulo operation)
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::algorithms::hash_functions::DoubleHasher;
    ///
    /// let double_hasher = DoubleHasher::new();
    /// let hashes = double_hasher.hash_multiple("test", 3, 100);
    /// assert_eq!(hashes.len(), 3);
    /// for &hash in &hashes {
    ///     assert!(hash < 100);
    /// }
    /// ```
    pub fn hash_multiple<T: Hash + ?Sized>(&self, item: &T, count: usize, max_value: usize) -> Vec<usize> {
        if max_value == 0 {
            return vec![0; count];
        }
        
        let h1 = self.hasher1.hash(item) % max_value;
        let h2 = self.hasher2.hash(item) % max_value;
        
        // Ensure h2 is odd to avoid cycles
        let h2 = if h2 % 2 == 0 { (h2 + 1) % max_value } else { h2 };
        
        let mut hashes = Vec::with_capacity(count);
        for i in 0..count {
            let hash = (h1 + i * h2) % max_value;
            hashes.push(hash);
        }
        
        hashes
    }
}

impl Default for DoubleHasher {
    fn default() -> Self {
        Self::new()
    }
}

/// Hash function quality metrics
pub struct HashQuality {
    /// Distribution uniformity (0.0 to 1.0, where 1.0 is perfectly uniform)
    pub uniformity: f64,
    /// Collision rate for a sample set
    pub collision_rate: f64,
    /// Avalanche effect score (0.0 to 1.0, where 1.0 is ideal)
    pub avalanche_score: f64,
}

/// Evaluate hash function quality with a test dataset
///
/// # Arguments
///
/// * `hasher` - Hash function to evaluate
/// * `test_data` - Test dataset
/// * `bucket_count` - Number of buckets for distribution testing
///
/// # Examples
///
/// ```
/// use yimi_rutool::algorithms::hash_functions::{HashFunction, evaluate_hash_quality};
///
/// let hash_fn = HashFunction::new();
/// let test_data: Vec<String> = (0..1000).map(|i| format!("item_{}", i)).collect();
/// let quality = evaluate_hash_quality(&hash_fn, &test_data, 100);
/// println!("Uniformity: {:.3}", quality.uniformity);
/// ```
pub fn evaluate_hash_quality<T: Hash + ?Sized + Clone>(
    hasher: &HashFunction,
    test_data: &[T],
    bucket_count: usize,
) -> HashQuality {
    if test_data.is_empty() || bucket_count == 0 {
        return HashQuality {
            uniformity: 0.0,
            collision_rate: 0.0,
            avalanche_score: 0.0,
        };
    }

    // Test distribution uniformity
    let mut buckets = vec![0; bucket_count];
    let mut hash_values = Vec::new();
    
    for item in test_data {
        let hash = hasher.hash(item);
        hash_values.push(hash);
        buckets[hash % bucket_count] += 1;
    }

    // Calculate uniformity using chi-square test
    let expected = test_data.len() as f64 / bucket_count as f64;
    let chi_square: f64 = buckets
        .iter()
        .map(|&observed| {
            let diff = observed as f64 - expected;
            diff * diff / expected
        })
        .sum();
    
    // Normalize chi-square to get uniformity score (inverse relationship)
    let uniformity = 1.0 / (1.0 + chi_square / bucket_count as f64);

    // Calculate collision rate
    let unique_hashes = {
        let mut sorted_hashes = hash_values.clone();
        sorted_hashes.sort_unstable();
        sorted_hashes.dedup();
        sorted_hashes.len()
    };
    let collision_rate = 1.0 - (unique_hashes as f64 / test_data.len() as f64);

    // Simple avalanche test (this is a simplified version)
    let avalanche_score = calculate_avalanche_score(hasher, test_data);

    HashQuality {
        uniformity,
        collision_rate,
        avalanche_score,
    }
}

/// Calculate avalanche effect score
fn calculate_avalanche_score<T: Hash + ?Sized + Clone>(
    hasher: &HashFunction,
    test_data: &[T],
) -> f64 {
    if test_data.is_empty() {
        return 0.0;
    }

    let sample_size = test_data.len().min(100); // Limit sample size for performance
    let mut bit_flip_counts = 0;
    let mut total_comparisons = 0;

    for i in 0..sample_size {
        let hash1 = hasher.hash(&test_data[i]);
        
        // For avalanche test, we'd need to flip bits in the input,
        // but since we can't easily do that with generic Hash types,
        // we'll use a simplified approach by comparing consecutive items
        if i + 1 < sample_size {
            let hash2 = hasher.hash(&test_data[i + 1]);
            let xor_result = hash1 ^ hash2;
            bit_flip_counts += xor_result.count_ones();
            total_comparisons += std::mem::size_of::<usize>() * 8; // bits in usize
        }
    }

    if total_comparisons == 0 {
        return 0.0;
    }

    // Ideally, about 50% of bits should flip
    let flip_ratio = bit_flip_counts as f64 / total_comparisons as f64;
    1.0 - (flip_ratio - 0.5).abs() * 2.0 // Score closer to 1.0 when flip_ratio is closer to 0.5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_function_creation() {
        let hash_fn = HashFunction::new();
        assert_eq!(hash_fn.algorithm(), HashAlgorithm::Default);
        assert_eq!(hash_fn.seed(), 0);
    }

    #[test]
    fn test_hash_function_with_algorithm() {
        let hash_fn = HashFunction::with_algorithm(HashAlgorithm::Murmur3);
        assert_eq!(hash_fn.algorithm(), HashAlgorithm::Murmur3);
        assert_eq!(hash_fn.seed(), 0);
    }

    #[test]
    fn test_hash_function_with_seed() {
        let hash_fn = HashFunction::with_seed(HashAlgorithm::Fnv1a, 12345);
        assert_eq!(hash_fn.algorithm(), HashAlgorithm::Fnv1a);
        assert_eq!(hash_fn.seed(), 12345);
    }

    #[test]
    fn test_generate_functions() {
        let functions = HashFunction::generate_functions(5);
        assert_eq!(functions.len(), 5);
        
        // Each function should have a different seed
        for i in 0..functions.len() {
            for j in (i + 1)..functions.len() {
                assert_ne!(functions[i].seed(), functions[j].seed());
            }
        }
    }

    #[test]
    fn test_hash_consistency() {
        let hash_fn = HashFunction::new();
        let item = "test_string";
        
        let hash1 = hash_fn.hash(item);
        let hash2 = hash_fn.hash(item);
        
        assert_eq!(hash1, hash2); // Same input should produce same hash
    }

    #[test]
    fn test_different_algorithms_produce_different_hashes() {
        let item = "test_string";
        
        let default_hash = HashFunction::with_algorithm(HashAlgorithm::Default).hash(item);
        let murmur3_hash = HashFunction::with_algorithm(HashAlgorithm::Murmur3).hash(item);
        let fnv1a_hash = HashFunction::with_algorithm(HashAlgorithm::Fnv1a).hash(item);
        
        // Different algorithms should generally produce different hashes
        assert_ne!(default_hash, murmur3_hash);
        assert_ne!(default_hash, fnv1a_hash);
        assert_ne!(murmur3_hash, fnv1a_hash);
    }

    #[test]
    fn test_different_seeds_produce_different_hashes() {
        let item = "test_string";
        let algorithm = HashAlgorithm::Default;
        
        let hash1 = HashFunction::with_seed(algorithm, 123).hash(item);
        let hash2 = HashFunction::with_seed(algorithm, 456).hash(item);
        
        assert_ne!(hash1, hash2); // Different seeds should produce different hashes
    }

    #[test]
    fn test_hash_distribution() {
        let hash_fn = HashFunction::new();
        let bucket_count = 100;
        let item_count = 1000;
        
        let mut buckets = vec![0; bucket_count];
        
        for i in 0..item_count {
            let item = format!("item_{}", i);
            let hash = hash_fn.hash(&item);
            buckets[hash % bucket_count] += 1;
        }
        
        // Check that no bucket is empty and none is overly full
        let min_count = buckets.iter().min().unwrap();
        let max_count = buckets.iter().max().unwrap();
        
        assert!(*min_count > 0); // No bucket should be empty
        assert!(*max_count < item_count / 5); // No bucket should have more than 20% of items
    }

    #[test]
    fn test_double_hasher() {
        let double_hasher = DoubleHasher::new();
        let hashes = double_hasher.hash_multiple("test", 5, 100);
        
        assert_eq!(hashes.len(), 5);
        
        // All hashes should be within bounds
        for &hash in &hashes {
            assert!(hash < 100);
        }
        
        // Hashes should be different (with high probability)
        let unique_hashes: std::collections::HashSet<_> = hashes.iter().collect();
        assert!(unique_hashes.len() > 1); // Should have at least some different values
    }

    #[test]
    fn test_double_hasher_zero_max_value() {
        let double_hasher = DoubleHasher::new();
        let hashes = double_hasher.hash_multiple("test", 5, 0);
        
        assert_eq!(hashes.len(), 5);
        assert!(hashes.iter().all(|&h| h == 0));
    }

    #[test]
    fn test_hash_quality_evaluation() {
        let hash_fn = HashFunction::new();
        let test_data: Vec<String> = (0..1000).map(|i| format!("item_{}", i)).collect();
        
        let quality = evaluate_hash_quality(&hash_fn, &test_data, 100);
        
        assert!(quality.uniformity >= 0.0 && quality.uniformity <= 1.0);
        assert!(quality.collision_rate >= 0.0 && quality.collision_rate <= 1.0);
        assert!(quality.avalanche_score >= 0.0 && quality.avalanche_score <= 1.0);
    }

    #[test]
    fn test_hash_quality_empty_data() {
        let hash_fn = HashFunction::new();
        let test_data: Vec<String> = vec![];
        
        let quality = evaluate_hash_quality(&hash_fn, &test_data, 100);
        
        assert_eq!(quality.uniformity, 0.0);
        assert_eq!(quality.collision_rate, 0.0);
        assert_eq!(quality.avalanche_score, 0.0);
    }

    #[test]
    fn test_murmur3_hash() {
        let item = "test";
        let hash1 = murmur3_hash(&item, 123);
        let hash2 = murmur3_hash(&item, 123);
        let hash3 = murmur3_hash(&item, 456);
        
        assert_eq!(hash1, hash2); // Same input and seed should produce same hash
        assert_ne!(hash1, hash3); // Different seed should produce different hash
    }

    #[test]
    fn test_fnv1a_hash() {
        let item = "test";
        let hash1 = fnv1a_hash(&item, 123);
        let hash2 = fnv1a_hash(&item, 123);
        let hash3 = fnv1a_hash(&item, 456);
        
        assert_eq!(hash1, hash2); // Same input and seed should produce same hash
        assert_ne!(hash1, hash3); // Different seed should produce different hash
    }

    #[test]
    fn test_seeded_algorithm() {
        let item = "test";
        let hash_fn1 = HashFunction::with_seed(HashAlgorithm::Seeded(999), 123);
        let hash_fn2 = HashFunction::with_seed(HashAlgorithm::Seeded(999), 456);
        
        let hash1 = hash_fn1.hash(&item);
        let hash2 = hash_fn2.hash(&item);
        
        assert_ne!(hash1, hash2); // Different seeds should produce different hashes
    }
}
