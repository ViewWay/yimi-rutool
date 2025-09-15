//! Caching utilities for rutool
//!
//! This module provides comprehensive caching functionality including:
//! - In-memory cache with TTL support
//! - LRU (Least Recently Used) cache implementation
//! - Thread-safe caching solutions
//! - Cache statistics and management

pub mod lru_cache;
pub mod memory_cache;

pub use lru_cache::LruCache;
/// Re-export commonly used types for convenience
pub use memory_cache::MemoryCache;
