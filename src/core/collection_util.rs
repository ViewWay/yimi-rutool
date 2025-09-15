//! Collection utility functions
//!
//! This module provides comprehensive collection manipulation utilities,
//! including lists, sets, maps, and various collection operations.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::hash::Hash;

/// Collection utility functions
pub struct CollUtil;

impl CollUtil {
    /// Check if collection is empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    ///
    /// let empty_vec: Vec<i32> = vec![];
    /// let non_empty_vec = vec![1, 2, 3];
    ///
    /// assert!(CollUtil::is_empty(&empty_vec));
    /// assert!(!CollUtil::is_empty(&non_empty_vec));
    /// ```
    pub fn is_empty<T>(collection: &[T]) -> bool {
        collection.is_empty()
    }

    /// Check if collection is not empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    ///
    /// let empty_vec: Vec<i32> = vec![];
    /// let non_empty_vec = vec![1, 2, 3];
    ///
    /// assert!(!CollUtil::is_not_empty(&empty_vec));
    /// assert!(CollUtil::is_not_empty(&non_empty_vec));
    /// ```
    pub fn is_not_empty<T>(collection: &[T]) -> bool {
        !collection.is_empty()
    }

    /// Get collection size
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    ///
    /// let vec = vec![1, 2, 3, 4, 5];
    /// assert_eq!(CollUtil::size(&vec), 5);
    /// ```
    pub fn size<T>(collection: &[T]) -> usize {
        collection.len()
    }

    /// Check if collection contains an element
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    ///
    /// let vec = vec![1, 2, 3, 4, 5];
    /// assert!(CollUtil::contains(&vec, &3));
    /// assert!(!CollUtil::contains(&vec, &6));
    /// ```
    pub fn contains<T: PartialEq>(collection: &[T], element: &T) -> bool {
        collection.contains(element)
    }

    /// Get first element of collection
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    ///
    /// let vec = vec![1, 2, 3, 4, 5];
    /// assert_eq!(CollUtil::get_first(&vec), Some(&1));
    ///
    /// let empty_vec: Vec<i32> = vec![];
    /// assert_eq!(CollUtil::get_first(&empty_vec), None);
    /// ```
    pub fn get_first<T>(collection: &[T]) -> Option<&T> {
        collection.first()
    }

    /// Get last element of collection
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    ///
    /// let vec = vec![1, 2, 3, 4, 5];
    /// assert_eq!(CollUtil::get_last(&vec), Some(&5));
    ///
    /// let empty_vec: Vec<i32> = vec![];
    /// assert_eq!(CollUtil::get_last(&empty_vec), None);
    /// ```
    pub fn get_last<T>(collection: &[T]) -> Option<&T> {
        collection.last()
    }

    /// Get element at specific index
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    ///
    /// let vec = vec![1, 2, 3, 4, 5];
    /// assert_eq!(CollUtil::get(&vec, 2), Some(&3));
    /// assert_eq!(CollUtil::get(&vec, 10), None);
    /// ```
    pub fn get<T>(collection: &[T], index: usize) -> Option<&T> {
        collection.get(index)
    }

    /// Get sublist from start index to end index
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    ///
    /// let vec = vec![1, 2, 3, 4, 5];
    /// let sublist = CollUtil::sub(&vec, 1, 4);
    /// assert_eq!(sublist, &[2, 3, 4]);
    /// ```
    pub fn sub<T>(collection: &[T], start: usize, end: usize) -> &[T] {
        let start = start.min(collection.len());
        let end = end.min(collection.len());
        if start >= end {
            &[]
        } else {
            &collection[start..end]
        }
    }

    /// Reverse a collection
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    ///
    /// let vec = vec![1, 2, 3, 4, 5];
    /// let reversed = CollUtil::reverse(&vec);
    /// assert_eq!(reversed, vec![5, 4, 3, 2, 1]);
    /// ```
    pub fn reverse<T: Clone>(collection: &[T]) -> Vec<T> {
        let mut result = collection.to_vec();
        result.reverse();
        result
    }

    /// Add element to collection (returns new vector)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    ///
    /// let vec = vec![1, 2, 3];
    /// let new_vec = CollUtil::add(&vec, &4);
    /// assert_eq!(new_vec, vec![1, 2, 3, 4]);
    /// ```
    pub fn add<T: Clone>(collection: &[T], element: &T) -> Vec<T> {
        let mut result = collection.to_vec();
        result.push(element.clone());
        result
    }

    /// Add all elements from another collection
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    ///
    /// let vec1 = vec![1, 2, 3];
    /// let vec2 = vec![4, 5, 6];
    /// let combined = CollUtil::add_all(&vec1, &vec2);
    /// assert_eq!(combined, vec![1, 2, 3, 4, 5, 6]);
    /// ```
    pub fn add_all<T: Clone>(collection: &[T], other: &[T]) -> Vec<T> {
        let mut result = collection.to_vec();
        result.extend_from_slice(other);
        result
    }

    /// Remove element from collection
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    ///
    /// let vec = vec![1, 2, 3, 4, 5];
    /// let new_vec = CollUtil::remove(&vec, 2);
    /// assert_eq!(new_vec, vec![1, 2, 4, 5]);
    /// ```
    pub fn remove<T: Clone + PartialEq>(collection: &[T], index: usize) -> Vec<T> {
        let mut result = collection.to_vec();
        if index < result.len() {
            result.remove(index);
        }
        result
    }

    /// Remove element by value
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    ///
    /// let vec = vec![1, 2, 3, 4, 5];
    /// let new_vec = CollUtil::remove_element(&vec, &3);
    /// assert_eq!(new_vec, vec![1, 2, 4, 5]);
    /// ```
    pub fn remove_element<T: Clone + PartialEq>(collection: &[T], element: &T) -> Vec<T> {
        collection
            .iter()
            .filter(|&item| item != element)
            .cloned()
            .collect()
    }

    /// Remove all elements that match a predicate
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    ///
    /// let vec = vec![1, 2, 3, 4, 5, 6];
    /// let new_vec = CollUtil::remove_if(&vec, |&x| x % 2 == 0);
    /// assert_eq!(new_vec, vec![1, 3, 5]);
    /// ```
    pub fn remove_if<T: Clone, F>(collection: &[T], predicate: F) -> Vec<T>
    where
        F: Fn(&T) -> bool,
    {
        collection
            .iter()
            .filter(|&item| !predicate(item))
            .cloned()
            .collect()
    }

    /// Filter collection by predicate
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    ///
    /// let vec = vec![1, 2, 3, 4, 5, 6];
    /// let filtered = CollUtil::filter(&vec, |&x| x % 2 == 0);
    /// assert_eq!(filtered, vec![2, 4, 6]);
    /// ```
    pub fn filter<T: Clone, F>(collection: &[T], predicate: F) -> Vec<T>
    where
        F: Fn(&T) -> bool,
    {
        collection
            .iter()
            .filter(|&item| predicate(item))
            .cloned()
            .collect()
    }

    /// Map collection elements to new type
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    ///
    /// let vec = vec![1, 2, 3, 4, 5];
    /// let mapped = CollUtil::map(&vec, |&x| x * 2);
    /// assert_eq!(mapped, vec![2, 4, 6, 8, 10]);
    /// ```
    pub fn map<T: Clone, U, F>(collection: &[T], mapper: F) -> Vec<U>
    where
        F: Fn(&T) -> U,
    {
        collection.iter().map(mapper).collect()
    }

    /// Find first element that matches predicate
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    ///
    /// let vec = vec![1, 2, 3, 4, 5];
    /// let found = CollUtil::find_first(&vec, |&x| x > 3);
    /// assert_eq!(found, Some(&4));
    ///
    /// let not_found = CollUtil::find_first(&vec, |&x| x > 10);
    /// assert_eq!(not_found, None);
    /// ```
    pub fn find_first<T, F>(collection: &[T], predicate: F) -> Option<&T>
    where
        F: Fn(&T) -> bool,
    {
        collection.iter().find(|&item| predicate(item))
    }

    /// Find all elements that match predicate
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    ///
    /// let vec = vec![1, 2, 3, 4, 5, 6];
    /// let found = CollUtil::find_all(&vec, |&x| x % 2 == 0);
    /// assert_eq!(found, vec![2, 4, 6]);
    /// ```
    pub fn find_all<T: Clone, F>(collection: &[T], predicate: F) -> Vec<T>
    where
        F: Fn(&T) -> bool,
    {
        Self::filter(collection, predicate)
    }

    /// Check if any element matches predicate
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    ///
    /// let vec = vec![1, 2, 3, 4, 5];
    /// assert!(CollUtil::any_match(&vec, |&x| x > 3));
    /// assert!(!CollUtil::any_match(&vec, |&x| x > 10));
    /// ```
    pub fn any_match<T, F>(collection: &[T], predicate: F) -> bool
    where
        F: Fn(&T) -> bool,
    {
        collection.iter().any(predicate)
    }

    /// Check if all elements match predicate
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    ///
    /// let vec = vec![2, 4, 6, 8, 10];
    /// assert!(CollUtil::all_match(&vec, |&x| x % 2 == 0));
    /// assert!(!CollUtil::all_match(&vec, |&x| x > 5));
    /// ```
    pub fn all_match<T, F>(collection: &[T], predicate: F) -> bool
    where
        F: Fn(&T) -> bool,
    {
        collection.iter().all(predicate)
    }

    /// Check if no elements match predicate
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    ///
    /// let vec = vec![1, 3, 5, 7, 9];
    /// assert!(CollUtil::none_match(&vec, |&x| x % 2 == 0));
    /// assert!(!CollUtil::none_match(&vec, |&x| x > 5));
    /// ```
    pub fn none_match<T, F>(collection: &[T], predicate: F) -> bool
    where
        F: Fn(&T) -> bool,
    {
        !Self::any_match(collection, predicate)
    }

    /// Get distinct elements (remove duplicates)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    ///
    /// let vec = vec![1, 2, 2, 3, 3, 3, 4, 5, 5];
    /// let distinct = CollUtil::distinct(&vec);
    /// assert_eq!(distinct, vec![1, 2, 3, 4, 5]);
    /// ```
    pub fn distinct<T: Clone + Hash + Eq>(collection: &[T]) -> Vec<T> {
        let mut set = HashSet::new();
        collection
            .iter()
            .filter(|&item| set.insert(item.clone()))
            .cloned()
            .collect()
    }

    /// Sort collection
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    ///
    /// let vec = vec![3, 1, 4, 1, 5, 9, 2, 6];
    /// let sorted = CollUtil::sort(&vec);
    /// assert_eq!(sorted, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    /// ```
    pub fn sort<T: Clone + Ord>(collection: &[T]) -> Vec<T> {
        let mut result = collection.to_vec();
        result.sort();
        result
    }

    /// Sort collection by key function
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    ///
    /// let vec = vec![3, 1, 4, 1, 5];
    /// let sorted = CollUtil::sort_by(&vec, |&x| -x); // descending order
    /// assert_eq!(sorted, vec![5, 4, 3, 1, 1]);
    /// ```
    pub fn sort_by<T: Clone, F, K>(collection: &[T], key_fn: F) -> Vec<T>
    where
        F: Fn(&T) -> K,
        K: Ord,
    {
        let mut result = collection.to_vec();
        result.sort_by_key(key_fn);
        result
    }

    /// Get maximum element
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    ///
    /// let vec = vec![3, 1, 4, 1, 5, 9, 2, 6];
    /// assert_eq!(CollUtil::max(&vec), Some(9));
    ///
    /// let empty_vec: Vec<i32> = vec![];
    /// assert_eq!(CollUtil::max(&empty_vec), None);
    /// ```
    pub fn max<T: Clone + PartialOrd>(collection: &[T]) -> Option<T> {
        collection
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .cloned()
    }

    /// Get minimum element
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    ///
    /// let vec = vec![3, 1, 4, 1, 5, 9, 2, 6];
    /// assert_eq!(CollUtil::min(&vec), Some(1));
    ///
    /// let empty_vec: Vec<i32> = vec![];
    /// assert_eq!(CollUtil::min(&empty_vec), None);
    /// ```
    pub fn min<T: Clone + PartialOrd>(collection: &[T]) -> Option<T> {
        collection
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .cloned()
    }

    /// Calculate sum of numeric collection
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    ///
    /// let vec = vec![1, 2, 3, 4, 5];
    /// assert_eq!(CollUtil::sum(&vec), 15);
    /// ```
    pub fn sum<T: Clone + std::iter::Sum + Default>(collection: &[T]) -> T {
        if collection.is_empty() {
            T::default()
        } else {
            collection.iter().cloned().sum()
        }
    }

    /// Calculate average of numeric collection
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    ///
    /// let vec = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// assert_eq!(CollUtil::average(&vec), Some(3.0));
    ///
    /// let empty_vec: Vec<f64> = vec![];
    /// assert_eq!(CollUtil::average(&empty_vec), None);
    /// ```
    pub fn average<T>(collection: &[T]) -> Option<T>
    where
        T: Clone + std::iter::Sum + std::ops::Div<Output = T> + From<u32>,
    {
        if collection.is_empty() {
            None
        } else {
            let sum: T = collection.iter().cloned().sum();
            Some(sum / T::from(collection.len() as u32))
        }
    }

    /// Group elements by key function
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    /// use std::collections::HashMap;
    ///
    /// let vec = vec![1, 2, 3, 4, 5, 6];
    /// let grouped = CollUtil::group_by(&vec, |&x| x % 2);
    ///
    /// assert_eq!(grouped.get(&0), Some(&vec![2, 4, 6]));
    /// assert_eq!(grouped.get(&1), Some(&vec![1, 3, 5]));
    /// ```
    pub fn group_by<T: Clone, K: Hash + Eq, F>(collection: &[T], key_fn: F) -> HashMap<K, Vec<T>>
    where
        F: Fn(&T) -> K,
    {
        let mut result = HashMap::new();
        for item in collection {
            let key = key_fn(item);
            result
                .entry(key)
                .or_insert_with(Vec::new)
                .push(item.clone());
        }
        result
    }

    /// Partition collection into chunks of specified size
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    ///
    /// let vec = vec![1, 2, 3, 4, 5, 6, 7];
    /// let chunks = CollUtil::partition(&vec, 3);
    /// assert_eq!(chunks, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7]]);
    /// ```
    pub fn partition<T: Clone>(collection: &[T], size: usize) -> Vec<Vec<T>> {
        if size == 0 {
            return vec![collection.to_vec()];
        }

        collection.chunks(size).map(<[T]>::to_vec).collect()
    }

    /// Zip two collections together
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    ///
    /// let vec1 = vec![1, 2, 3];
    /// let vec2 = vec!["a", "b", "c"];
    /// let zipped = CollUtil::zip(&vec1, &vec2);
    /// assert_eq!(zipped, vec![(1, "a"), (2, "b"), (3, "c")]);
    /// ```
    pub fn zip<T: Clone, U: Clone>(collection1: &[T], collection2: &[U]) -> Vec<(T, U)> {
        collection1
            .iter()
            .zip(collection2.iter())
            .map(|(a, b)| (a.clone(), b.clone()))
            .collect()
    }

    /// Create HashSet from collection
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    /// use std::collections::HashSet;
    ///
    /// let vec = vec![1, 2, 2, 3, 3, 3];
    /// let set = CollUtil::to_set(&vec);
    /// let mut expected = HashSet::new();
    /// expected.insert(1);
    /// expected.insert(2);
    /// expected.insert(3);
    /// assert_eq!(set, expected);
    /// ```
    pub fn to_set<T: Clone + Hash + Eq>(collection: &[T]) -> HashSet<T> {
        collection.iter().cloned().collect()
    }

    /// Create BTreeSet from collection (sorted)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    /// use std::collections::BTreeSet;
    ///
    /// let vec = vec![3, 1, 4, 1, 5];
    /// let set = CollUtil::to_sorted_set(&vec);
    /// let expected: BTreeSet<i32> = [1, 3, 4, 5].into();
    /// assert_eq!(set, expected);
    /// ```
    pub fn to_sorted_set<T: Clone + Ord>(collection: &[T]) -> BTreeSet<T> {
        collection.iter().cloned().collect()
    }

    /// Create HashMap from key-value pairs collection
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    /// use std::collections::HashMap;
    ///
    /// let pairs = vec![("a", 1), ("b", 2), ("c", 3)];
    /// let map = CollUtil::to_map(&pairs);
    /// assert_eq!(map.get("a"), Some(&1));
    /// assert_eq!(map.get("b"), Some(&2));
    /// assert_eq!(map.get("c"), Some(&3));
    /// ```
    pub fn to_map<K: Clone + Hash + Eq, V: Clone>(pairs: &[(K, V)]) -> HashMap<K, V> {
        pairs.iter().cloned().collect()
    }

    /// Create BTreeMap from key-value pairs collection (sorted by key)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::CollUtil;
    /// use std::collections::BTreeMap;
    ///
    /// let pairs = vec![("c", 3), ("a", 1), ("b", 2)];
    /// let map = CollUtil::to_sorted_map(&pairs);
    /// let mut expected = BTreeMap::new();
    /// expected.insert("a", 1);
    /// expected.insert("b", 2);
    /// expected.insert("c", 3);
    /// assert_eq!(map, expected);
    /// ```
    pub fn to_sorted_map<K: Clone + Ord, V: Clone>(pairs: &[(K, V)]) -> BTreeMap<K, V> {
        pairs.iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_empty() {
        let empty: Vec<i32> = vec![];
        let non_empty = vec![1, 2, 3];
        assert!(CollUtil::is_empty(&empty));
        assert!(!CollUtil::is_empty(&non_empty));
    }

    #[test]
    fn test_get_first_last() {
        let vec = vec![1, 2, 3, 4, 5];
        assert_eq!(CollUtil::get_first(&vec), Some(&1));
        assert_eq!(CollUtil::get_last(&vec), Some(&5));

        let empty: Vec<i32> = vec![];
        assert_eq!(CollUtil::get_first(&empty), None);
        assert_eq!(CollUtil::get_last(&empty), None);
    }

    #[test]
    fn test_contains() {
        let vec = vec![1, 2, 3, 4, 5];
        assert!(CollUtil::contains(&vec, &3));
        assert!(!CollUtil::contains(&vec, &6));
    }

    #[test]
    fn test_sub() {
        let vec = vec![1, 2, 3, 4, 5];
        assert_eq!(CollUtil::sub(&vec, 1, 4), &[2, 3, 4]);
        assert_eq!(CollUtil::sub(&vec, 0, 10), &[1, 2, 3, 4, 5]);
        let empty_slice: &[i32] = &[];
        assert_eq!(CollUtil::sub(&vec, 3, 3), empty_slice);
    }

    #[test]
    fn test_add_and_remove() {
        let vec = vec![1, 2, 3];
        let added = CollUtil::add(&vec, &4);
        assert_eq!(added, vec![1, 2, 3, 4]);

        let removed = CollUtil::remove(&vec, 1);
        assert_eq!(removed, vec![1, 3]);
    }

    #[test]
    fn test_filter() {
        let vec = vec![1, 2, 3, 4, 5, 6];
        let even = CollUtil::filter(&vec, |&x| x % 2 == 0);
        assert_eq!(even, vec![2, 4, 6]);
    }

    #[test]
    fn test_map() {
        let vec = vec![1, 2, 3, 4, 5];
        let doubled = CollUtil::map(&vec, |&x| x * 2);
        assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_find_first() {
        let vec = vec![1, 2, 3, 4, 5];
        assert_eq!(CollUtil::find_first(&vec, |&x| x > 3), Some(&4));
        assert_eq!(CollUtil::find_first(&vec, |&x| x > 10), None);
    }

    #[test]
    fn test_distinct() {
        let vec = vec![1, 2, 2, 3, 3, 3, 4, 5, 5];
        let distinct = CollUtil::distinct(&vec);
        assert_eq!(distinct.len(), 5);
        assert!(distinct.contains(&1));
        assert!(distinct.contains(&2));
        assert!(distinct.contains(&3));
        assert!(distinct.contains(&4));
        assert!(distinct.contains(&5));
    }

    #[test]
    fn test_sort() {
        let vec = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let sorted = CollUtil::sort(&vec);
        assert_eq!(sorted, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_max_min() {
        let vec = vec![3, 1, 4, 1, 5, 9, 2, 6];
        assert_eq!(CollUtil::max(&vec), Some(9));
        assert_eq!(CollUtil::min(&vec), Some(1));

        let empty: Vec<i32> = vec![];
        assert_eq!(CollUtil::max(&empty), None);
        assert_eq!(CollUtil::min(&empty), None);
    }

    #[test]
    fn test_sum_average() {
        let vec = vec![1, 2, 3, 4, 5];
        assert_eq!(CollUtil::sum(&vec), 15);

        let float_vec = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(CollUtil::average(&float_vec), Some(3.0));

        let empty: Vec<f64> = vec![];
        assert_eq!(CollUtil::average(&empty), None);
    }

    #[test]
    fn test_partition() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7];
        let chunks = CollUtil::partition(&vec, 3);
        assert_eq!(chunks, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7]]);
    }

    #[test]
    fn test_zip() {
        let vec1 = vec![1, 2, 3];
        let vec2 = vec!["a", "b", "c"];
        let zipped = CollUtil::zip(&vec1, &vec2);
        assert_eq!(zipped, vec![(1, "a"), (2, "b"), (3, "c")]);
    }
}
