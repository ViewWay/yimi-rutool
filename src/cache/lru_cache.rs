//! LRU (Least Recently Used) cache implementation
//!
//! This module provides a thread-safe LRU cache that automatically evicts
//! the least recently used items when the cache reaches its capacity limit.

use crate::error::{Error, Result};
use std::collections::HashMap;
use std::ptr::NonNull;
use std::sync::{Arc, Mutex};

/// A node in the doubly-linked list
struct Node<K, V> {
    key: K,
    value: V,
    prev: Option<NonNull<Node<K, V>>>,
    next: Option<NonNull<Node<K, V>>>,
}

impl<K, V> Node<K, V> {
    fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            prev: None,
            next: None,
        }
    }
}

/// Thread-safe LRU cache implementation
///
/// The LRU cache maintains a fixed capacity and automatically evicts the least
/// recently used items when new items are inserted and the cache is at capacity.
///
/// # Examples
///
/// ```rust
/// use rutool::cache::LruCache;
///
/// let mut cache = LruCache::new(2);
/// 
/// cache.put("key1".to_string(), "value1".to_string()).unwrap();
/// cache.put("key2".to_string(), "value2".to_string()).unwrap();
/// cache.put("key3".to_string(), "value3".to_string()).unwrap(); // This will evict "key1"
/// 
/// assert_eq!(cache.get(&"key1".to_string()).unwrap(), None); // Evicted
/// assert_eq!(cache.get(&"key2".to_string()).unwrap(), Some("value2".to_string()));
/// assert_eq!(cache.get(&"key3".to_string()).unwrap(), Some("value3".to_string()));
/// ```
pub struct LruCache<K, V>
where
    K: Clone + Eq + std::hash::Hash,
    V: Clone,
{
    inner: Arc<Mutex<LruCacheInner<K, V>>>,
}

struct LruCacheInner<K, V>
where
    K: Clone + Eq + std::hash::Hash,
    V: Clone,
{
    capacity: usize,
    map: HashMap<K, NonNull<Node<K, V>>>,
    head: Option<NonNull<Node<K, V>>>,
    tail: Option<NonNull<Node<K, V>>>,
    len: usize,
}

impl<K, V> LruCache<K, V>
where
    K: Clone + Eq + std::hash::Hash,
    V: Clone,
{
    /// Create a new LRU cache with the specified capacity
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::cache::LruCache;
    ///
    /// let cache: LruCache<String, i32> = LruCache::new(100);
    /// ```
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "Capacity must be greater than 0");
        
        Self {
            inner: Arc::new(Mutex::new(LruCacheInner {
                capacity,
                map: HashMap::new(),
                head: None,
                tail: None,
                len: 0,
            })),
        }
    }

    /// Get a value from the cache
    ///
    /// This operation moves the accessed item to the front of the LRU list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::cache::LruCache;
    ///
    /// let mut cache = LruCache::new(2);
    /// cache.put("key", "value").unwrap();
    /// 
    /// assert_eq!(cache.get("key").unwrap(), Some("value"));
    /// assert_eq!(cache.get("nonexistent").unwrap(), None);
    /// ```
    pub fn get(&self, key: &K) -> Result<Option<V>> {
        let mut inner = self.inner.lock()
            .map_err(|_| Error::concurrency("Failed to acquire lock".to_string()))?;
        
        if let Some(&node_ptr) = inner.map.get(key) {
            unsafe {
                let node_ref = node_ptr.as_ref();
                let value = node_ref.value.clone();
                
                // Move to front
                inner.move_to_front(node_ptr);
                
                Ok(Some(value))
            }
        } else {
            Ok(None)
        }
    }

    /// Insert a key-value pair into the cache
    ///
    /// If the cache is at capacity, the least recently used item will be evicted.
    /// If the key already exists, its value will be updated and it will be moved
    /// to the front of the LRU list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::cache::LruCache;
    ///
    /// let mut cache = LruCache::new(2);
    /// cache.put("key", "value").unwrap();
    /// ```
    pub fn put(&self, key: K, value: V) -> Result<()> {
        let mut inner = self.inner.lock()
            .map_err(|_| Error::concurrency("Failed to acquire lock".to_string()))?;
        
        if let Some(&existing_node) = inner.map.get(&key) {
            // Update existing node
            unsafe {
                let mut existing_node_mut = existing_node;
                let existing_ref = existing_node_mut.as_mut();
                existing_ref.value = value;
                inner.move_to_front(existing_node_mut);
            }
        } else {
            // Create new node
            let new_node = Box::new(Node::new(key.clone(), value));
            let new_node_ptr = NonNull::from(Box::leak(new_node));
            
            inner.map.insert(key, new_node_ptr);
            unsafe {
                inner.add_to_front(new_node_ptr);
            }
            inner.len += 1;
            
            // Check capacity and evict if necessary
            if inner.len > inner.capacity {
                unsafe {
                    inner.remove_tail();
                }
            }
        }
        
        Ok(())
    }

    /// Remove a key-value pair from the cache
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::cache::LruCache;
    ///
    /// let mut cache = LruCache::new(2);
    /// cache.put("key", "value").unwrap();
    /// 
    /// let removed = cache.remove("key").unwrap();
    /// assert_eq!(removed, Some("value"));
    /// assert_eq!(cache.get("key").unwrap(), None);
    /// ```
    pub fn remove(&self, key: &K) -> Result<Option<V>> {
        let mut inner = self.inner.lock()
            .map_err(|_| Error::concurrency("Failed to acquire lock".to_string()))?;
        
        if let Some(node_ptr) = inner.map.remove(key) {
            unsafe {
                let value = node_ptr.as_ref().value.clone();
                inner.remove_node(node_ptr);
                inner.len -= 1;
                
                // Deallocate the node
                let _ = Box::from_raw(node_ptr.as_ptr());
                
                Ok(Some(value))
            }
        } else {
            Ok(None)
        }
    }

    /// Check if the cache contains a key
    ///
    /// This operation does not affect the LRU order.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::cache::LruCache;
    ///
    /// let mut cache = LruCache::new(2);
    /// cache.put("key", "value").unwrap();
    /// 
    /// assert!(cache.contains_key("key").unwrap());
    /// assert!(!cache.contains_key("nonexistent").unwrap());
    /// ```
    pub fn contains_key(&self, key: &K) -> Result<bool> {
        let inner = self.inner.lock()
            .map_err(|_| Error::concurrency("Failed to acquire lock".to_string()))?;
        
        Ok(inner.map.contains_key(key))
    }

    /// Get the current number of items in the cache
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::cache::LruCache;
    ///
    /// let mut cache = LruCache::new(10);
    /// assert_eq!(cache.len().unwrap(), 0);
    /// 
    /// cache.put("key", "value").unwrap();
    /// assert_eq!(cache.len().unwrap(), 1);
    /// ```
    pub fn len(&self) -> Result<usize> {
        let inner = self.inner.lock()
            .map_err(|_| Error::concurrency("Failed to acquire lock".to_string()))?;
        
        Ok(inner.len)
    }

    /// Check if the cache is empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::cache::LruCache;
    ///
    /// let cache = LruCache::new(10);
    /// assert!(cache.is_empty().unwrap());
    /// ```
    pub fn is_empty(&self) -> Result<bool> {
        Ok(self.len()? == 0)
    }

    /// Get the capacity of the cache
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::cache::LruCache;
    ///
    /// let cache = LruCache::new(100);
    /// assert_eq!(cache.capacity().unwrap(), 100);
    /// ```
    pub fn capacity(&self) -> Result<usize> {
        let inner = self.inner.lock()
            .map_err(|_| Error::concurrency("Failed to acquire lock".to_string()))?;
        
        Ok(inner.capacity)
    }

    /// Clear all items from the cache
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::cache::LruCache;
    ///
    /// let mut cache = LruCache::new(10);
    /// cache.put("key1".to_string(), "value1".to_string()).unwrap();
    /// cache.put("key2".to_string(), "value2".to_string()).unwrap();
    /// 
    /// cache.clear().unwrap();
    /// assert!(cache.is_empty().unwrap());
    /// ```
    pub fn clear(&self) -> Result<()> {
        let mut inner = self.inner.lock()
            .map_err(|_| Error::concurrency("Failed to acquire lock".to_string()))?;
        
        // Deallocate all nodes
        unsafe {
            let mut current = inner.head;
            while let Some(node_ptr) = current {
                let node_ref = node_ptr.as_ref();
                current = node_ref.next;
                let _ = Box::from_raw(node_ptr.as_ptr());
            }
        }
        
        inner.map.clear();
        inner.head = None;
        inner.tail = None;
        inner.len = 0;
        
        Ok(())
    }

    /// Get all keys in the cache in LRU order (most recent first)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::cache::LruCache;
    ///
    /// let mut cache = LruCache::new(3);
    /// cache.put("key1".to_string(), "value1".to_string()).unwrap();
    /// cache.put("key2".to_string(), "value2".to_string()).unwrap();
    /// cache.put("key3".to_string(), "value3".to_string()).unwrap();
    /// 
    /// let keys = cache.keys().unwrap();
    /// assert_eq!(keys, vec!["key3".to_string(), "key2".to_string(), "key1".to_string()]);
    /// ```
    pub fn keys(&self) -> Result<Vec<K>> {
        let inner = self.inner.lock()
            .map_err(|_| Error::concurrency("Failed to acquire lock".to_string()))?;
        
        let mut keys = Vec::new();
        let mut current = inner.head;
        
        unsafe {
            while let Some(node_ptr) = current {
                let node_ref = node_ptr.as_ref();
                keys.push(node_ref.key.clone());
                current = node_ref.next;
            }
        }
        
        Ok(keys)
    }

    /// Peek at the least recently used item without removing it
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::cache::LruCache;
    ///
    /// let mut cache = LruCache::new(3);
    /// cache.put("key1".to_string(), "value1".to_string()).unwrap();
    /// cache.put("key2".to_string(), "value2".to_string()).unwrap();
    /// 
    /// let (key, value) = cache.peek_lru().unwrap().unwrap();
    /// assert_eq!(key, "key1");
    /// assert_eq!(value, "value1");
    /// ```
    pub fn peek_lru(&self) -> Result<Option<(K, V)>> {
        let inner = self.inner.lock()
            .map_err(|_| Error::concurrency("Failed to acquire lock".to_string()))?;
        
        if let Some(tail_ptr) = inner.tail {
            unsafe {
                let tail_ref = tail_ptr.as_ref();
                Ok(Some((tail_ref.key.clone(), tail_ref.value.clone())))
            }
        } else {
            Ok(None)
        }
    }

    /// Peek at the most recently used item without removing it
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::cache::LruCache;
    ///
    /// let mut cache = LruCache::new(3);
    /// cache.put("key1".to_string(), "value1".to_string()).unwrap();
    /// cache.put("key2".to_string(), "value2".to_string()).unwrap();
    /// 
    /// let (key, value) = cache.peek_mru().unwrap().unwrap();
    /// assert_eq!(key, "key2");
    /// assert_eq!(value, "value2");
    /// ```
    pub fn peek_mru(&self) -> Result<Option<(K, V)>> {
        let inner = self.inner.lock()
            .map_err(|_| Error::concurrency("Failed to acquire lock".to_string()))?;
        
        if let Some(head_ptr) = inner.head {
            unsafe {
                let head_ref = head_ptr.as_ref();
                Ok(Some((head_ref.key.clone(), head_ref.value.clone())))
            }
        } else {
            Ok(None)
        }
    }

    /// Get or insert a value for the given key
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rutool::cache::LruCache;
    ///
    /// let mut cache = LruCache::new(10);
    /// 
    /// let value = cache.get_or_insert("key", || "computed_value".to_string()).unwrap();
    /// assert_eq!(value, "computed_value");
    /// 
    /// // Second call should return cached value
    /// let cached_value = cache.get_or_insert("key", || "new_value".to_string()).unwrap();
    /// assert_eq!(cached_value, "computed_value");
    /// ```
    pub fn get_or_insert<F>(&self, key: K, compute_fn: F) -> Result<V>
    where
        F: FnOnce() -> V,
    {
        // Try to get existing value first
        if let Some(value) = self.get(&key)? {
            return Ok(value);
        }

        // Compute and store new value
        let value = compute_fn();
        self.put(key, value.clone())?;
        Ok(value)
    }
}

impl<K, V> LruCacheInner<K, V>
where
    K: Clone + Eq + std::hash::Hash,
    V: Clone,
{
    unsafe fn move_to_front(&mut self, node_ptr: NonNull<Node<K, V>>) {
        // Remove from current position
        self.remove_node(node_ptr);
        // Add to front
        self.add_to_front(node_ptr);
    }

    unsafe fn add_to_front(&mut self, mut node_ptr: NonNull<Node<K, V>>) {
        let node_ref = node_ptr.as_mut();
        node_ref.prev = None;
        node_ref.next = self.head;

        if let Some(mut old_head) = self.head {
            old_head.as_mut().prev = Some(node_ptr);
        } else {
            // First node, also set as tail
            self.tail = Some(node_ptr);
        }

        self.head = Some(node_ptr);
    }

    unsafe fn remove_node(&mut self, node_ptr: NonNull<Node<K, V>>) {
        let node_ref = node_ptr.as_ref();

        if let Some(mut prev) = node_ref.prev {
            prev.as_mut().next = node_ref.next;
        } else {
            // This was the head
            self.head = node_ref.next;
        }

        if let Some(mut next) = node_ref.next {
            next.as_mut().prev = node_ref.prev;
        } else {
            // This was the tail
            self.tail = node_ref.prev;
        }
    }

    unsafe fn remove_tail(&mut self) {
        if let Some(tail_ptr) = self.tail {
            let tail_ref = tail_ptr.as_ref();
            let key = tail_ref.key.clone();
            
            self.map.remove(&key);
            self.remove_node(tail_ptr);
            self.len -= 1;
            
            // Deallocate the node
            let _ = Box::from_raw(tail_ptr.as_ptr());
        }
    }
}

impl<K, V> Clone for LruCache<K, V>
where
    K: Clone + Eq + std::hash::Hash,
    V: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<K, V> Drop for LruCacheInner<K, V>
where
    K: Clone + Eq + std::hash::Hash,
    V: Clone,
{
    fn drop(&mut self) {
        // Deallocate all nodes
        unsafe {
            let mut current = self.head;
            while let Some(node_ptr) = current {
                let node_ref = node_ptr.as_ref();
                current = node_ref.next;
                let _ = Box::from_raw(node_ptr.as_ptr());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let cache: LruCache<String, String> = LruCache::new(2);
        
        // Test put and get
        cache.put("key1".to_string(), "value1".to_string()).unwrap();
        assert_eq!(cache.get(&"key1".to_string()).unwrap(), Some("value1".to_string()));
        
        // Test contains_key
        assert!(cache.contains_key(&"key1".to_string()).unwrap());
        assert!(!cache.contains_key(&"nonexistent".to_string()).unwrap());
        
        // Test len
        assert_eq!(cache.len().unwrap(), 1);
        assert!(!cache.is_empty().unwrap());
        
        // Test capacity
        assert_eq!(cache.capacity().unwrap(), 2);
    }

    #[test]
    fn test_lru_eviction() {
        let cache: LruCache<String, String> = LruCache::new(2);
        
        cache.put("key1".to_string(), "value1".to_string()).unwrap();
        cache.put("key2".to_string(), "value2".to_string()).unwrap();
        cache.put("key3".to_string(), "value3".to_string()).unwrap(); // Should evict key1
        
        assert_eq!(cache.len().unwrap(), 2);
        assert_eq!(cache.get(&"key1".to_string()).unwrap(), None); // Evicted
        assert_eq!(cache.get(&"key2".to_string()).unwrap(), Some("value2".to_string()));
        assert_eq!(cache.get(&"key3".to_string()).unwrap(), Some("value3".to_string()));
    }

    #[test]
    fn test_lru_order() {
        let cache: LruCache<String, String> = LruCache::new(3);
        
        cache.put("key1".to_string(), "value1".to_string()).unwrap();
        cache.put("key2".to_string(), "value2".to_string()).unwrap();
        cache.put("key3".to_string(), "value3".to_string()).unwrap();
        
        // Access key1 to make it most recent
        cache.get(&"key1".to_string()).unwrap();
        
        // Add key4, should evict key2 (least recent)
        cache.put("key4".to_string(), "value4".to_string()).unwrap();
        
        assert_eq!(cache.get(&"key2".to_string()).unwrap(), None); // Evicted
        assert_eq!(cache.get(&"key1".to_string()).unwrap(), Some("value1".to_string())); // Still there
        assert_eq!(cache.get(&"key3".to_string()).unwrap(), Some("value3".to_string()));
        assert_eq!(cache.get(&"key4".to_string()).unwrap(), Some("value4".to_string()));
    }

    #[test]
    fn test_update_existing_key() {
        let cache: LruCache<String, String> = LruCache::new(2);
        
        cache.put("key1".to_string(), "value1".to_string()).unwrap();
        cache.put("key2".to_string(), "value2".to_string()).unwrap();
        
        // Update existing key
        cache.put("key1".to_string(), "updated_value1".to_string()).unwrap();
        
        assert_eq!(cache.len().unwrap(), 2);
        assert_eq!(cache.get(&"key1".to_string()).unwrap(), Some("updated_value1".to_string()));
        assert_eq!(cache.get(&"key2".to_string()).unwrap(), Some("value2".to_string()));
    }

    #[test]
    fn test_remove() {
        let cache: LruCache<String, String> = LruCache::new(3);
        
        cache.put("key1".to_string(), "value1".to_string()).unwrap();
        cache.put("key2".to_string(), "value2".to_string()).unwrap();
        
        let removed = cache.remove(&"key1".to_string()).unwrap();
        assert_eq!(removed, Some("value1".to_string()));
        assert_eq!(cache.len().unwrap(), 1);
        assert_eq!(cache.get(&"key1".to_string()).unwrap(), None);
        assert_eq!(cache.get(&"key2".to_string()).unwrap(), Some("value2".to_string()));
        
        // Remove non-existent key
        let removed = cache.remove(&"nonexistent".to_string()).unwrap();
        assert_eq!(removed, None);
    }

    #[test]
    fn test_clear() {
        let cache: LruCache<String, String> = LruCache::new(3);
        
        cache.put("key1".to_string(), "value1".to_string()).unwrap();
        cache.put("key2".to_string(), "value2".to_string()).unwrap();
        cache.put("key3".to_string(), "value3".to_string()).unwrap();
        
        cache.clear().unwrap();
        assert!(cache.is_empty().unwrap());
        assert_eq!(cache.len().unwrap(), 0);
        assert_eq!(cache.get(&"key1".to_string()).unwrap(), None);
    }

    #[test]
    fn test_keys() {
        let cache: LruCache<String, String> = LruCache::new(3);
        
        cache.put("key1".to_string(), "value1".to_string()).unwrap();
        cache.put("key2".to_string(), "value2".to_string()).unwrap();
        cache.put("key3".to_string(), "value3".to_string()).unwrap();
        
        let keys = cache.keys().unwrap();
        // Most recent first
        assert_eq!(keys, vec!["key3".to_string(), "key2".to_string(), "key1".to_string()]);
        
        // Access key1 to make it most recent
        cache.get(&"key1".to_string()).unwrap();
        
        let keys = cache.keys().unwrap();
        assert_eq!(keys, vec!["key1".to_string(), "key3".to_string(), "key2".to_string()]);
    }

    #[test]
    fn test_peek_lru_mru() {
        let cache: LruCache<String, String> = LruCache::new(3);
        
        cache.put("key1".to_string(), "value1".to_string()).unwrap();
        cache.put("key2".to_string(), "value2".to_string()).unwrap();
        cache.put("key3".to_string(), "value3".to_string()).unwrap();
        
        let (lru_key, lru_value) = cache.peek_lru().unwrap().unwrap();
        assert_eq!(lru_key, "key1".to_string());
        assert_eq!(lru_value, "value1".to_string());
        
        let (mru_key, mru_value) = cache.peek_mru().unwrap().unwrap();
        assert_eq!(mru_key, "key3".to_string());
        assert_eq!(mru_value, "value3".to_string());
        
        // Access key1 to change order
        cache.get(&"key1".to_string()).unwrap();
        
        let (lru_key, _) = cache.peek_lru().unwrap().unwrap();
        assert_eq!(lru_key, "key2".to_string());
        
        let (mru_key, _) = cache.peek_mru().unwrap().unwrap();
        assert_eq!(mru_key, "key1".to_string());
    }

    #[test]
    fn test_get_or_insert() {
        let cache: LruCache<String, String> = LruCache::new(2);
        
        // First call should compute
        let value = cache.get_or_insert("key".to_string(), || "computed".to_string()).unwrap();
        assert_eq!(value, "computed".to_string());
        
        // Second call should return cached value
        let cached = cache.get_or_insert("key".to_string(), || "new_computed".to_string()).unwrap();
        assert_eq!(cached, "computed".to_string());
        
        assert_eq!(cache.len().unwrap(), 1);
    }

    #[test]
    fn test_clone() {
        let cache1 = LruCache::new(2);
        cache1.put("key".to_string(), "value".to_string()).unwrap();
        
        let cache2 = cache1.clone();
        assert_eq!(cache2.get(&"key".to_string()).unwrap(), Some("value".to_string()));
        
        // They should share the same underlying data
        cache2.put("key2".to_string(), "value2".to_string()).unwrap();
        assert_eq!(cache1.get(&"key2".to_string()).unwrap(), Some("value2".to_string()));
    }

    #[test]
    fn test_empty_cache() {
        let cache: LruCache<String, String> = LruCache::new(1);
        
        assert!(cache.is_empty().unwrap());
        assert_eq!(cache.peek_lru().unwrap(), None);
        assert_eq!(cache.peek_mru().unwrap(), None);
        assert_eq!(cache.keys().unwrap(), Vec::<String>::new());
    }

    #[test]
    #[should_panic(expected = "Capacity must be greater than 0")]
    fn test_zero_capacity() {
        LruCache::<i32, i32>::new(0);
    }
}
