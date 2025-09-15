//! In-memory cache implementation with TTL support
//!
//! This module provides a thread-safe in-memory cache with time-to-live (TTL)
//! functionality, inspired by Hutool's CacheUtil.

use crate::error::{Error, Result};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

/// Cache entry with value and expiration time
#[derive(Debug, Clone)]
struct CacheEntry<V> {
    value: V,
    expires_at: Option<Instant>,
    created_at: Instant,
    access_count: u64,
    last_accessed: Instant,
}

impl<V> CacheEntry<V> {
    fn new(value: V, ttl: Option<Duration>) -> Self {
        let now = Instant::now();
        Self {
            value,
            expires_at: ttl.map(|duration| now + duration),
            created_at: now,
            access_count: 0,
            last_accessed: now,
        }
    }

    fn is_expired(&self) -> bool {
        self.expires_at
            .map_or(false, |expires_at| Instant::now() > expires_at)
    }

    fn access(&mut self) -> &V {
        self.access_count += 1;
        self.last_accessed = Instant::now();
        &self.value
    }
}

/// Thread-safe in-memory cache with TTL support
///
/// # Examples
///
/// ```rust
/// use yimi_rutool::cache::MemoryCache;
/// use std::time::Duration;
///
/// let cache = MemoryCache::new();
///
/// // Store value without TTL
/// cache.put("key1".to_string(), "value1".to_string()).unwrap();
///
/// // Store value with TTL
/// cache.put_with_ttl("key2".to_string(), "value2".to_string(), Duration::from_secs(60)).unwrap();
///
/// // Retrieve values
/// assert_eq!(cache.get(&"key1".to_string()).unwrap(), Some("value1".to_string()));
/// assert_eq!(cache.get(&"key2".to_string()).unwrap(), Some("value2".to_string()));
/// ```
pub struct MemoryCache<K, V>
where
    K: Clone + Eq + std::hash::Hash,
    V: Clone,
{
    data: Arc<RwLock<HashMap<K, CacheEntry<V>>>>,
    default_ttl: Option<Duration>,
    max_size: Option<usize>,
}

impl<K, V> MemoryCache<K, V>
where
    K: Clone + Eq + std::hash::Hash,
    V: Clone,
{
    /// Create a new cache with no size limit and no default TTL
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::cache::MemoryCache;
    ///
    /// let cache: MemoryCache<String, i32> = MemoryCache::new();
    /// ```
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
            default_ttl: None,
            max_size: None,
        }
    }

    /// Create a new cache with default TTL
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::cache::MemoryCache;
    /// use std::time::Duration;
    ///
    /// let cache: MemoryCache<String, i32> = MemoryCache::with_ttl(Duration::from_secs(300));
    /// ```
    pub fn with_ttl(default_ttl: Duration) -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
            default_ttl: Some(default_ttl),
            max_size: None,
        }
    }

    /// Create a new cache with maximum size limit
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::cache::MemoryCache;
    ///
    /// let cache: MemoryCache<String, i32> = MemoryCache::with_max_size(1000);
    /// ```
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
            default_ttl: None,
            max_size: Some(max_size),
        }
    }

    /// Create a new cache with both default TTL and maximum size
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::cache::MemoryCache;
    /// use std::time::Duration;
    ///
    /// let cache: MemoryCache<String, i32> = MemoryCache::with_ttl_and_size(
    ///     Duration::from_secs(300),
    ///     1000
    /// );
    /// ```
    pub fn with_ttl_and_size(default_ttl: Duration, max_size: usize) -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
            default_ttl: Some(default_ttl),
            max_size: Some(max_size),
        }
    }

    /// Store a value in the cache with default TTL
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::cache::MemoryCache;
    ///
    /// let cache = MemoryCache::new();
    /// cache.put("key".to_string(), "value".to_string()).unwrap();
    /// ```
    pub fn put(&self, key: K, value: V) -> Result<()> {
        let entry = CacheEntry::new(value, self.default_ttl);
        self.put_entry(key, entry)
    }

    /// Store a value in the cache with specific TTL
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::cache::MemoryCache;
    /// use std::time::Duration;
    ///
    /// let cache = MemoryCache::new();
    /// cache.put_with_ttl("key", "value", Duration::from_secs(60)).unwrap();
    /// ```
    pub fn put_with_ttl(&self, key: K, value: V, ttl: Duration) -> Result<()> {
        let entry = CacheEntry::new(value, Some(ttl));
        self.put_entry(key, entry)
    }

    /// Store a value in the cache without TTL (never expires)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::cache::MemoryCache;
    ///
    /// let cache = MemoryCache::new();
    /// cache.put_permanent("key".to_string(), "value".to_string()).unwrap();
    /// ```
    pub fn put_permanent(&self, key: K, value: V) -> Result<()> {
        let entry = CacheEntry::new(value, None);
        self.put_entry(key, entry)
    }

    fn put_entry(&self, key: K, entry: CacheEntry<V>) -> Result<()> {
        let mut data = self
            .data
            .write()
            .map_err(|_| Error::concurrency("Failed to acquire write lock".to_string()))?;

        // Check size limit and evict if necessary
        if let Some(max_size) = self.max_size {
            while data.len() >= max_size {
                // Remove oldest entry
                if let Some(oldest_key) = self.find_oldest_key(&data) {
                    data.remove(&oldest_key);
                } else {
                    break;
                }
            }
        }

        data.insert(key, entry);
        Ok(())
    }

    fn find_oldest_key(&self, data: &HashMap<K, CacheEntry<V>>) -> Option<K> {
        data.iter()
            .min_by_key(|(_, entry)| entry.created_at)
            .map(|(key, _)| key.clone())
    }

    /// Retrieve a value from the cache
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::cache::MemoryCache;
    ///
    /// let cache = MemoryCache::new();
    /// cache.put("key".to_string(), "value".to_string()).unwrap();
    ///
    /// let result = cache.get(&"key".to_string()).unwrap();
    /// assert_eq!(result, Some("value".to_string()));
    /// ```
    pub fn get(&self, key: &K) -> Result<Option<V>> {
        let mut data = self
            .data
            .write()
            .map_err(|_| Error::concurrency("Failed to acquire write lock".to_string()))?;

        if let Some(entry) = data.get_mut(key) {
            if entry.is_expired() {
                data.remove(key);
                Ok(None)
            } else {
                Ok(Some(entry.access().clone()))
            }
        } else {
            Ok(None)
        }
    }

    /// Check if a key exists in the cache (without updating access time)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::cache::MemoryCache;
    ///
    /// let cache = MemoryCache::new();
    /// cache.put("key".to_string(), "value".to_string()).unwrap();
    ///
    /// assert!(cache.contains_key(&"key".to_string()).unwrap());
    /// assert!(!cache.contains_key(&"nonexistent".to_string()).unwrap());
    /// ```
    pub fn contains_key(&self, key: &K) -> Result<bool> {
        let data = self
            .data
            .read()
            .map_err(|_| Error::concurrency("Failed to acquire read lock".to_string()))?;

        if let Some(entry) = data.get(key) {
            Ok(!entry.is_expired())
        } else {
            Ok(false)
        }
    }

    /// Remove a value from the cache
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::cache::MemoryCache;
    ///
    /// let cache = MemoryCache::new();
    /// cache.put("key".to_string(), "value".to_string()).unwrap();
    ///
    /// let removed = cache.remove(&"key".to_string()).unwrap();
    /// assert_eq!(removed, Some("value".to_string()));
    /// ```
    pub fn remove(&self, key: &K) -> Result<Option<V>> {
        let mut data = self
            .data
            .write()
            .map_err(|_| Error::concurrency("Failed to acquire write lock".to_string()))?;

        Ok(data.remove(key).map(|entry| entry.value))
    }

    /// Clear all entries from the cache
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::cache::MemoryCache;
    ///
    /// let cache = MemoryCache::new();
    /// cache.put("key1".to_string(), "value1".to_string()).unwrap();
    /// cache.put("key2".to_string(), "value2".to_string()).unwrap();
    ///
    /// cache.clear().unwrap();
    /// assert_eq!(cache.size().unwrap(), 0);
    /// ```
    pub fn clear(&self) -> Result<()> {
        let mut data = self
            .data
            .write()
            .map_err(|_| Error::concurrency("Failed to acquire write lock".to_string()))?;

        data.clear();
        Ok(())
    }

    /// Get the number of entries in the cache
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::cache::MemoryCache;
    ///
    /// let cache = MemoryCache::new();
    /// cache.put("key1".to_string(), "value1".to_string()).unwrap();
    /// cache.put("key2".to_string(), "value2".to_string()).unwrap();
    ///
    /// assert_eq!(cache.size().unwrap(), 2);
    /// ```
    pub fn size(&self) -> Result<usize> {
        let data = self
            .data
            .read()
            .map_err(|_| Error::concurrency("Failed to acquire read lock".to_string()))?;

        Ok(data.len())
    }

    /// Check if the cache is empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::cache::MemoryCache;
    ///
    /// let cache = MemoryCache::new();
    /// assert!(cache.is_empty().unwrap());
    ///
    /// cache.put("key".to_string(), "value".to_string()).unwrap();
    /// assert!(!cache.is_empty().unwrap());
    /// ```
    pub fn is_empty(&self) -> Result<bool> {
        Ok(self.size()? == 0)
    }

    /// Remove all expired entries from the cache
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::cache::MemoryCache;
    /// use std::time::Duration;
    ///
    /// let cache = MemoryCache::new();
    /// cache.put_with_ttl("key", "value", Duration::from_millis(1)).unwrap();
    ///
    /// std::thread::sleep(Duration::from_millis(10));
    /// let removed = cache.cleanup_expired().unwrap();
    /// assert_eq!(removed, 1);
    /// ```
    pub fn cleanup_expired(&self) -> Result<usize> {
        let mut data = self
            .data
            .write()
            .map_err(|_| Error::concurrency("Failed to acquire write lock".to_string()))?;

        let expired_keys: Vec<K> = data
            .iter()
            .filter(|(_, entry)| entry.is_expired())
            .map(|(key, _)| key.clone())
            .collect();

        let count = expired_keys.len();
        for key in expired_keys {
            data.remove(&key);
        }

        Ok(count)
    }

    /// Get all keys in the cache (excluding expired ones)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::cache::MemoryCache;
    ///
    /// let cache = MemoryCache::new();
    /// cache.put("key1".to_string(), "value1".to_string()).unwrap();
    /// cache.put("key2".to_string(), "value2".to_string()).unwrap();
    ///
    /// let keys = cache.keys().unwrap();
    /// assert_eq!(keys.len(), 2);
    /// ```
    pub fn keys(&self) -> Result<Vec<K>> {
        let data = self
            .data
            .read()
            .map_err(|_| Error::concurrency("Failed to acquire read lock".to_string()))?;

        let keys: Vec<K> = data
            .iter()
            .filter(|(_, entry)| !entry.is_expired())
            .map(|(key, _)| key.clone())
            .collect();

        Ok(keys)
    }

    /// Get cache statistics
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::cache::MemoryCache;
    ///
    /// let cache = MemoryCache::new();
    /// cache.put("key1".to_string(), "value1".to_string()).unwrap();
    /// cache.put("key2".to_string(), "value2".to_string()).unwrap();
    ///
    /// let stats = cache.stats().unwrap();
    /// assert_eq!(stats.total_entries, 2);
    /// ```
    pub fn stats(&self) -> Result<CacheStats> {
        let data = self
            .data
            .read()
            .map_err(|_| Error::concurrency("Failed to acquire read lock".to_string()))?;

        let total_entries = data.len();
        let expired_entries = data.values().filter(|entry| entry.is_expired()).count();
        let active_entries = total_entries - expired_entries;

        let total_access_count: u64 = data.values().map(|entry| entry.access_count).sum();
        let avg_access_count = if total_entries > 0 {
            // Use safe conversion for better precision
            if total_entries == 0 {
                0.0
            } else {
                (total_access_count as f64) / (total_entries as f64)
            }
        } else {
            0.0
        };

        Ok(CacheStats {
            total_entries,
            active_entries,
            expired_entries,
            total_access_count,
            avg_access_count,
        })
    }

    /// Get or compute a value for the given key
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::cache::MemoryCache;
    ///
    /// let cache = MemoryCache::new();
    ///
    /// let value = cache.get_or_compute("key".to_string(), || "computed_value".to_string()).unwrap();
    /// assert_eq!(value, "computed_value");
    ///
    /// // Second call should return cached value
    /// let cached_value = cache.get_or_compute("key".to_string(), || "new_value".to_string()).unwrap();
    /// assert_eq!(cached_value, "computed_value");
    /// ```
    pub fn get_or_compute<F>(&self, key: K, compute_fn: F) -> Result<V>
    where
        F: FnOnce() -> V,
    {
        // Try to get existing value first
        if let Some(value) = self.get(&key)? {
            return Ok(value);
        }

        // Compute and store new value
        let value = compute_fn();
        self.put(key.clone(), value.clone())?;
        Ok(value)
    }

    /// Get or compute a value with TTL for the given key
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::cache::MemoryCache;
    /// use std::time::Duration;
    ///
    /// let cache = MemoryCache::new();
    ///
    /// let value = cache.get_or_compute_with_ttl(
    ///     "key",
    ///     || "computed_value".to_string(),
    ///     Duration::from_secs(60)
    /// ).unwrap();
    /// assert_eq!(value, "computed_value");
    /// ```
    pub fn get_or_compute_with_ttl<F>(&self, key: K, compute_fn: F, ttl: Duration) -> Result<V>
    where
        F: FnOnce() -> V,
    {
        // Try to get existing value first
        if let Some(value) = self.get(&key)? {
            return Ok(value);
        }

        // Compute and store new value with TTL
        let value = compute_fn();
        self.put_with_ttl(key.clone(), value.clone(), ttl)?;
        Ok(value)
    }
}

impl<K, V> Default for MemoryCache<K, V>
where
    K: Clone + Eq + std::hash::Hash,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> Clone for MemoryCache<K, V>
where
    K: Clone + Eq + std::hash::Hash,
    V: Clone,
{
    fn clone(&self) -> Self {
        Self {
            data: Arc::clone(&self.data),
            default_ttl: self.default_ttl,
            max_size: self.max_size,
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Total number of entries (including expired)
    pub total_entries: usize,
    /// Number of active (non-expired) entries
    pub active_entries: usize,
    /// Number of expired entries
    pub expired_entries: usize,
    /// Total access count across all entries
    pub total_access_count: u64,
    /// Average access count per entry
    pub avg_access_count: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_basic_operations() {
        let cache: MemoryCache<String, String> = MemoryCache::new();

        // Test put and get
        cache.put("key1".to_string(), "value1".to_string()).unwrap();
        assert_eq!(
            cache.get(&"key1".to_string()).unwrap(),
            Some("value1".to_string())
        );

        // Test contains_key
        assert!(cache.contains_key(&"key1".to_string()).unwrap());
        assert!(!cache.contains_key(&"nonexistent".to_string()).unwrap());

        // Test size
        assert_eq!(cache.size().unwrap(), 1);
        assert!(!cache.is_empty().unwrap());

        // Test remove
        let removed = cache.remove(&"key1".to_string()).unwrap();
        assert_eq!(removed, Some("value1".to_string()));
        assert_eq!(cache.get(&"key1".to_string()).unwrap(), None);
        assert!(cache.is_empty().unwrap());
    }

    #[test]
    fn test_ttl() {
        let cache: MemoryCache<String, String> = MemoryCache::new();

        // Store with short TTL
        cache
            .put_with_ttl(
                "key".to_string(),
                "value".to_string(),
                Duration::from_millis(10),
            )
            .unwrap();
        assert_eq!(
            cache.get(&"key".to_string()).unwrap(),
            Some("value".to_string())
        );

        // Wait for expiration
        thread::sleep(Duration::from_millis(15));
        assert_eq!(cache.get(&"key".to_string()).unwrap(), None);
    }

    #[test]
    fn test_default_ttl() {
        let cache = MemoryCache::with_ttl(Duration::from_millis(10));

        cache.put("key".to_string(), "value".to_string()).unwrap();
        assert_eq!(
            cache.get(&"key".to_string()).unwrap(),
            Some("value".to_string())
        );

        thread::sleep(Duration::from_millis(15));
        assert_eq!(cache.get(&"key".to_string()).unwrap(), None);
    }

    #[test]
    fn test_permanent_storage() {
        let cache = MemoryCache::with_ttl(Duration::from_millis(10));

        cache
            .put_permanent("key".to_string(), "value".to_string())
            .unwrap();
        thread::sleep(Duration::from_millis(15));

        // Should still be available
        assert_eq!(
            cache.get(&"key".to_string()).unwrap(),
            Some("value".to_string())
        );
    }

    #[test]
    fn test_max_size() {
        let cache: MemoryCache<String, String> = MemoryCache::with_max_size(2);

        cache.put("key1".to_string(), "value1".to_string()).unwrap();
        cache.put("key2".to_string(), "value2".to_string()).unwrap();
        cache.put("key3".to_string(), "value3".to_string()).unwrap(); // Should evict key1

        assert_eq!(cache.size().unwrap(), 2);
        assert_eq!(cache.get(&"key1".to_string()).unwrap(), None); // Evicted
        assert_eq!(
            cache.get(&"key2".to_string()).unwrap(),
            Some("value2".to_string())
        );
        assert_eq!(
            cache.get(&"key3".to_string()).unwrap(),
            Some("value3".to_string())
        );
    }

    #[test]
    fn test_cleanup_expired() {
        let cache: MemoryCache<String, String> = MemoryCache::new();

        cache
            .put_with_ttl(
                "key1".to_string(),
                "value1".to_string(),
                Duration::from_millis(10),
            )
            .unwrap();
        cache
            .put_permanent("key2".to_string(), "value2".to_string())
            .unwrap();

        thread::sleep(Duration::from_millis(15));

        let removed = cache.cleanup_expired().unwrap();
        assert_eq!(removed, 1);
        assert_eq!(cache.size().unwrap(), 1);
        assert_eq!(
            cache.get(&"key2".to_string()).unwrap(),
            Some("value2".to_string())
        );
    }

    #[test]
    fn test_get_or_compute() {
        let cache: MemoryCache<String, String> = MemoryCache::new();

        // First call should compute
        let value = cache
            .get_or_compute("key".to_string(), || "computed".to_string())
            .unwrap();
        assert_eq!(value, "computed".to_string());

        // Second call should return cached value
        let cached = cache
            .get_or_compute("key".to_string(), || "new_computed".to_string())
            .unwrap();
        assert_eq!(cached, "computed".to_string());
    }

    #[test]
    fn test_stats() {
        let cache: MemoryCache<String, String> = MemoryCache::new();

        cache.put("key1".to_string(), "value1".to_string()).unwrap();
        cache.put("key2".to_string(), "value2".to_string()).unwrap();
        cache.get(&"key1".to_string()).unwrap(); // Access once
        cache.get(&"key1".to_string()).unwrap(); // Access twice

        let stats = cache.stats().unwrap();
        assert_eq!(stats.total_entries, 2);
        assert_eq!(stats.active_entries, 2);
        assert_eq!(stats.expired_entries, 0);
        assert_eq!(stats.total_access_count, 2);
        assert_eq!(stats.avg_access_count, 1.0);
    }

    #[test]
    fn test_keys() {
        let cache: MemoryCache<String, String> = MemoryCache::new();

        cache.put("key1".to_string(), "value1".to_string()).unwrap();
        cache.put("key2".to_string(), "value2".to_string()).unwrap();
        cache
            .put_with_ttl(
                "key3".to_string(),
                "value3".to_string(),
                Duration::from_millis(1),
            )
            .unwrap();

        thread::sleep(Duration::from_millis(10));

        let keys = cache.keys().unwrap();
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&"key1".to_string()));
        assert!(keys.contains(&"key2".to_string()));
        assert!(!keys.contains(&"key3".to_string())); // Expired
    }

    #[test]
    fn test_clear() {
        let cache: MemoryCache<String, String> = MemoryCache::new();

        cache.put("key1".to_string(), "value1".to_string()).unwrap();
        cache.put("key2".to_string(), "value2".to_string()).unwrap();

        cache.clear().unwrap();
        assert!(cache.is_empty().unwrap());
    }

    #[test]
    fn test_clone() {
        let cache1 = MemoryCache::new();
        cache1.put("key".to_string(), "value".to_string()).unwrap();

        let cache2 = cache1.clone();
        assert_eq!(
            cache2.get(&"key".to_string()).unwrap(),
            Some("value".to_string())
        );

        // They should share the same underlying data
        cache2
            .put("key2".to_string(), "value2".to_string())
            .unwrap();
        assert_eq!(
            cache1.get(&"key2".to_string()).unwrap(),
            Some("value2".to_string())
        );
    }
}
