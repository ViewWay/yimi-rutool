//! Caching utilities for rutool
//!
//! This module provides comprehensive caching functionality including:
//! - In-memory cache with TTL support
//! - LRU (Least Recently Used) cache implementation
//! - Thread-safe caching solutions
//! - Cache statistics and management

pub mod memory_cache;
pub mod lru_cache;

/// Re-export commonly used types for convenience
pub use memory_cache::MemoryCache;
pub use lru_cache::LruCache;
