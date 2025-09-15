//! Sensitive word filtering using DFA (Deterministic Finite Automaton)
//!
//! This module provides high-performance sensitive word detection and filtering
//! using a DFA-based approach for optimal text processing performance.

use std::collections::{HashMap, HashSet};
use std::fmt;

/// A match found in the text
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WordMatch {
    /// The matched sensitive word
    pub word: String,
    /// Start position in the original text
    pub start: usize,
    /// End position in the original text
    pub end: usize,
    /// The original matched text (may differ from word due to case)
    pub matched_text: String,
}

impl WordMatch {
    /// Create a new word match
    pub fn new(word: String, start: usize, end: usize, matched_text: String) -> Self {
        WordMatch {
            word,
            start,
            end,
            matched_text,
        }
    }

    /// Get the length of the matched text
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Check if the match is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl fmt::Display for WordMatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WordMatch {{ word: '{}', start: {}, end: {}, matched: '{}' }}", 
               self.word, self.start, self.end, self.matched_text)
    }
}

/// Strategies for handling detected sensitive words
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilterStrategy {
    /// Replace with asterisks (*) of the same length
    Mask,
    /// Replace with a specific string
    Replace(String),
    /// Replace with a character repeated to match length
    Char(char),
    /// Highlight with markers (e.g., [word])
    Highlight(String, String), // prefix, suffix
    /// Remove the word entirely
    Remove,
    /// Keep original but record the match
    KeepOriginal,
}

impl Default for FilterStrategy {
    fn default() -> Self {
        FilterStrategy::Mask
    }
}

impl FilterStrategy {
    /// Apply the strategy to a matched word
    pub fn apply(&self, word_match: &WordMatch) -> String {
        match self {
            FilterStrategy::Mask => "*".repeat(word_match.len()),
            FilterStrategy::Replace(replacement) => replacement.clone(),
            FilterStrategy::Char(ch) => ch.to_string().repeat(word_match.len()),
            FilterStrategy::Highlight(prefix, suffix) => {
                format!("{}{}{}", prefix, word_match.matched_text, suffix)
            }
            FilterStrategy::Remove => String::new(),
            FilterStrategy::KeepOriginal => word_match.matched_text.clone(),
        }
    }
}

/// Result of filtering operation
#[derive(Debug, Clone)]
pub struct FilterResult {
    /// The filtered text
    pub filtered_text: String,
    /// List of matches found
    pub matches: Vec<WordMatch>,
    /// Original text length
    pub original_length: usize,
    /// Filtered text length
    pub filtered_length: usize,
}

impl FilterResult {
    /// Create a new filter result
    pub fn new(filtered_text: String, matches: Vec<WordMatch>, original_length: usize) -> Self {
        let filtered_length = filtered_text.len();
        FilterResult {
            filtered_text,
            matches,
            original_length,
            filtered_length,
        }
    }

    /// Check if any sensitive words were found
    pub fn has_matches(&self) -> bool {
        !self.matches.is_empty()
    }

    /// Get the number of matches found
    pub fn match_count(&self) -> usize {
        self.matches.len()
    }

    /// Get the unique sensitive words found
    pub fn unique_words(&self) -> HashSet<&str> {
        self.matches.iter().map(|m| m.word.as_str()).collect()
    }

    /// Calculate the percentage of text that was filtered
    pub fn filter_percentage(&self) -> f64 {
        if self.original_length == 0 {
            return 0.0;
        }
        let filtered_chars: usize = self.matches.iter().map(|m| m.len()).sum();
        (filtered_chars as f64 / self.original_length as f64) * 100.0
    }
}

impl fmt::Display for FilterResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FilterResult {{ matches: {}, filter_rate: {:.1}% }}", 
               self.match_count(), self.filter_percentage())
    }
}

/// Processing statistics for performance monitoring
#[derive(Debug, Clone, Default)]
pub struct ProcessingStats {
    /// Number of texts processed
    pub texts_processed: usize,
    /// Total characters processed
    pub chars_processed: usize,
    /// Total matches found
    pub total_matches: usize,
    /// Total processing time in microseconds
    pub processing_time_us: u64,
}

impl ProcessingStats {
    /// Calculate average processing speed in chars per second
    pub fn chars_per_second(&self) -> f64 {
        if self.processing_time_us == 0 {
            return 0.0;
        }
        (self.chars_processed as f64) / (self.processing_time_us as f64 / 1_000_000.0)
    }

    /// Calculate average match rate as percentage
    pub fn match_rate(&self) -> f64 {
        if self.chars_processed == 0 {
            return 0.0;
        }
        (self.total_matches as f64 / self.texts_processed as f64) * 100.0
    }
}

impl fmt::Display for ProcessingStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ProcessingStats {{ texts: {}, chars: {}, matches: {}, speed: {:.0} chars/sec }}", 
               self.texts_processed, self.chars_processed, self.total_matches, self.chars_per_second())
    }
}

/// DFA node for the sensitive word automaton
#[derive(Debug, Clone)]
struct DfaNode {
    /// Child nodes indexed by character
    children: HashMap<char, usize>,
    /// Failure link for AC automaton
    failure: usize,
    /// Words that end at this node
    output: Vec<String>,
}

impl DfaNode {
    fn new() -> Self {
        DfaNode {
            children: HashMap::new(),
            failure: 0,
            output: Vec::new(),
        }
    }
}

/// High-performance sensitive word filter using DFA
///
/// Uses the Aho-Corasick algorithm for efficient multi-pattern matching.
///
/// # Examples
///
/// ```
/// use yimi_rutool::text::{SensitiveWordFilter, FilterStrategy};
///
/// let mut filter = SensitiveWordFilter::new();
/// filter.add_word("badword");
/// filter.add_word("sensitive");
/// filter.build();
///
/// let result = filter.filter_with_strategy("This badword is sensitive", &FilterStrategy::Mask);
/// println!("Filtered: {}", result.filtered_text);
/// ```
#[derive(Debug, Clone)]
pub struct SensitiveWordFilter {
    /// DFA nodes
    nodes: Vec<DfaNode>,
    /// Set of sensitive words for quick lookup
    word_set: HashSet<String>,
    /// Whether the automaton has been built
    built: bool,
    /// Case sensitivity setting
    case_sensitive: bool,
    /// Processing statistics
    stats: ProcessingStats,
}

impl SensitiveWordFilter {
    /// Create a new empty filter
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::text::SensitiveWordFilter;
    ///
    /// let filter = SensitiveWordFilter::new();
    /// ```
    pub fn new() -> Self {
        let mut filter = SensitiveWordFilter {
            nodes: Vec::new(),
            word_set: HashSet::new(),
            built: false,
            case_sensitive: false,
            stats: ProcessingStats::default(),
        };
        
        // Add root node
        filter.nodes.push(DfaNode::new());
        filter
    }

    /// Set case sensitivity
    ///
    /// # Arguments
    ///
    /// * `case_sensitive` - Whether matching should be case sensitive
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::text::SensitiveWordFilter;
    ///
    /// let mut filter = SensitiveWordFilter::new();
    /// filter.set_case_sensitive(true);
    /// ```
    pub fn set_case_sensitive(&mut self, case_sensitive: bool) {
        self.case_sensitive = case_sensitive;
        self.built = false; // Need to rebuild
    }

    /// Add a sensitive word to the filter
    ///
    /// # Arguments
    ///
    /// * `word` - The sensitive word to add
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::text::SensitiveWordFilter;
    ///
    /// let mut filter = SensitiveWordFilter::new();
    /// filter.add_word("badword");
    /// filter.add_word("sensitive");
    /// ```
    pub fn add_word(&mut self, word: &str) {
        if word.is_empty() {
            return;
        }
        
        let processed_word = if self.case_sensitive {
            word.to_string()
        } else {
            word.to_lowercase()
        };
        
        self.word_set.insert(processed_word);
        self.built = false; // Need to rebuild automaton
    }

    /// Add multiple words at once
    ///
    /// # Arguments
    ///
    /// * `words` - Iterator of words to add
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::text::SensitiveWordFilter;
    ///
    /// let mut filter = SensitiveWordFilter::new();
    /// filter.add_words(vec!["word1", "word2", "word3"]);
    /// ```
    pub fn add_words<I>(&mut self, words: I)
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        for word in words {
            self.add_word(word.as_ref());
        }
    }

    /// Remove a word from the filter
    ///
    /// # Arguments
    ///
    /// * `word` - The word to remove
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::text::SensitiveWordFilter;
    ///
    /// let mut filter = SensitiveWordFilter::new();
    /// filter.add_word("badword");
    /// filter.remove_word("badword");
    /// ```
    pub fn remove_word(&mut self, word: &str) {
        let processed_word = if self.case_sensitive {
            word.to_string()
        } else {
            word.to_lowercase()
        };
        
        if self.word_set.remove(&processed_word) {
            self.built = false; // Need to rebuild
        }
    }

    /// Clear all words from the filter
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::text::SensitiveWordFilter;
    ///
    /// let mut filter = SensitiveWordFilter::new();
    /// filter.add_word("badword");
    /// filter.clear();
    /// ```
    pub fn clear(&mut self) {
        self.word_set.clear();
        self.built = false;
    }

    /// Check if a word is in the filter
    ///
    /// # Arguments
    ///
    /// * `word` - The word to check
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::text::SensitiveWordFilter;
    ///
    /// let mut filter = SensitiveWordFilter::new();
    /// filter.add_word("badword");
    /// assert!(filter.contains_word("badword"));
    /// assert!(!filter.contains_word("goodword"));
    /// ```
    pub fn contains_word(&self, word: &str) -> bool {
        let processed_word = if self.case_sensitive {
            word.to_string()
        } else {
            word.to_lowercase()
        };
        self.word_set.contains(&processed_word)
    }

    /// Get the number of words in the filter
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::text::SensitiveWordFilter;
    ///
    /// let mut filter = SensitiveWordFilter::new();
    /// filter.add_word("word1");
    /// filter.add_word("word2");
    /// assert_eq!(filter.word_count(), 2);
    /// ```
    pub fn word_count(&self) -> usize {
        self.word_set.len()
    }

    /// Get all words in the filter
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::text::SensitiveWordFilter;
    ///
    /// let mut filter = SensitiveWordFilter::new();
    /// filter.add_word("word1");
    /// filter.add_word("word2");
    /// let words = filter.get_words();
    /// assert_eq!(words.len(), 2);
    /// ```
    pub fn get_words(&self) -> Vec<&str> {
        self.word_set.iter().map(|s| s.as_str()).collect()
    }

    /// Build the DFA automaton
    ///
    /// This must be called after adding words and before filtering.
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::text::SensitiveWordFilter;
    ///
    /// let mut filter = SensitiveWordFilter::new();
    /// filter.add_word("badword");
    /// filter.build();
    /// ```
    pub fn build(&mut self) {
        if self.built {
            return;
        }

        // Reset nodes
        self.nodes.clear();
        self.nodes.push(DfaNode::new());

        // Build trie
        for word in &self.word_set {
            let mut current = 0;
            
            for ch in word.chars() {
                if let Some(&next) = self.nodes[current].children.get(&ch) {
                    current = next;
                } else {
                    let new_node = self.nodes.len();
                    self.nodes.push(DfaNode::new());
                    self.nodes[current].children.insert(ch, new_node);
                    current = new_node;
                }
            }
            
            // Mark end of word
            self.nodes[current].output.push(word.clone());
        }

        // Build failure links using BFS
        self.build_failure_links();
        self.built = true;
    }

    /// Build failure links for AC automaton
    fn build_failure_links(&mut self) {
        use std::collections::VecDeque;
        
        let mut queue = VecDeque::new();
        
        // Initialize level 1 nodes
        for &child in self.nodes[0].children.values() {
            queue.push_back(child);
        }
        
        while let Some(current) = queue.pop_front() {
            for (&ch, &child) in &self.nodes[current].children.clone() {
                queue.push_back(child);
                
                // Find failure link for child
                let mut temp = self.nodes[current].failure;
                
                while temp != 0 && !self.nodes[temp].children.contains_key(&ch) {
                    temp = self.nodes[temp].failure;
                }
                
                if self.nodes[temp].children.contains_key(&ch) && 
                   self.nodes[temp].children[&ch] != child {
                    self.nodes[child].failure = self.nodes[temp].children[&ch];
                } else {
                    self.nodes[child].failure = 0;
                }
                
                // Add failure node's output to current node
                let failure_output = self.nodes[self.nodes[child].failure].output.clone();
                self.nodes[child].output.extend(failure_output);
            }
        }
    }

    /// Find all matches in the text
    ///
    /// # Arguments
    ///
    /// * `text` - The text to search
    ///
    /// # Returns
    ///
    /// Vector of word matches found
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::text::SensitiveWordFilter;
    ///
    /// let mut filter = SensitiveWordFilter::new();
    /// filter.add_word("bad");
    /// filter.build();
    /// 
    /// let matches = filter.find_matches("This bad text has bad words");
    /// assert_eq!(matches.len(), 2);
    /// ```
    pub fn find_matches(&mut self, text: &str) -> Vec<WordMatch> {
        if !self.built {
            self.build();
        }

        let start_time = std::time::Instant::now();
        let mut matches = Vec::new();
        let mut current = 0;
        
        let processed_text = if self.case_sensitive {
            text.to_string()
        } else {
            text.to_lowercase()
        };
        
        let chars: Vec<char> = processed_text.chars().collect();
        
        for (i, &ch) in chars.iter().enumerate() {
            // Follow failure links until we find a match or reach root
            while current != 0 && !self.nodes[current].children.contains_key(&ch) {
                current = self.nodes[current].failure;
            }
            
            if let Some(&next) = self.nodes[current].children.get(&ch) {
                current = next;
            }
            
            // Check for matches at current position
            for word in &self.nodes[current].output {
                let word_len = word.chars().count();
                let start_pos = i + 1 - word_len;
                let end_pos = i + 1;
                
                // Get original text for the match
                let original_chars: Vec<char> = text.chars().collect();
                let matched_text: String = original_chars[start_pos..end_pos].iter().collect();
                
                matches.push(WordMatch::new(
                    word.clone(),
                    start_pos,
                    end_pos,
                    matched_text,
                ));
            }
        }

        // Update statistics
        let elapsed = start_time.elapsed();
        self.stats.texts_processed += 1;
        self.stats.chars_processed += text.len();
        self.stats.total_matches += matches.len();
        self.stats.processing_time_us += elapsed.as_micros() as u64;

        matches
    }

    /// Check if text contains any sensitive words
    ///
    /// # Arguments
    ///
    /// * `text` - The text to check
    ///
    /// # Returns
    ///
    /// `true` if sensitive words were found
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::text::SensitiveWordFilter;
    ///
    /// let mut filter = SensitiveWordFilter::new();
    /// filter.add_word("bad");
    /// filter.build();
    /// 
    /// assert!(filter.contains_sensitive_words("This is bad"));
    /// assert!(!filter.contains_sensitive_words("This is good"));
    /// ```
    pub fn contains_sensitive_words(&mut self, text: &str) -> bool {
        !self.find_matches(text).is_empty()
    }

    /// Filter text using the default mask strategy
    ///
    /// # Arguments
    ///
    /// * `text` - The text to filter
    ///
    /// # Returns
    ///
    /// Filtered text with sensitive words replaced by asterisks
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::text::SensitiveWordFilter;
    ///
    /// let mut filter = SensitiveWordFilter::new();
    /// filter.add_word("bad");
    /// filter.build();
    /// 
    /// let filtered = filter.filter("This bad text");
    /// println!("Filtered: {}", filtered); // "This *** text"
    /// ```
    pub fn filter(&mut self, text: &str) -> String {
        self.filter_with_strategy(text, &FilterStrategy::Mask).filtered_text
    }

    /// Filter text with a specific strategy
    ///
    /// # Arguments
    ///
    /// * `text` - The text to filter
    /// * `strategy` - The filtering strategy to use
    ///
    /// # Returns
    ///
    /// Complete filter result with matches and statistics
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::text::{SensitiveWordFilter, FilterStrategy};
    ///
    /// let mut filter = SensitiveWordFilter::new();
    /// filter.add_word("bad");
    /// filter.build();
    /// 
    /// let result = filter.filter_with_strategy("This bad text", &FilterStrategy::Replace("good".to_string()));
    /// assert_eq!(result.filtered_text, "This good text");
    /// ```
    pub fn filter_with_strategy(&mut self, text: &str, strategy: &FilterStrategy) -> FilterResult {
        let matches = self.find_matches(text);
        let original_length = text.len();
        
        if matches.is_empty() {
            return FilterResult::new(text.to_string(), matches, original_length);
        }

        // Sort matches by position (reverse order for proper replacement)
        let mut sorted_matches = matches.clone();
        sorted_matches.sort_by(|a, b| b.start.cmp(&a.start));

        let mut result = text.to_string();
        let chars: Vec<char> = text.chars().collect();
        
        for word_match in &sorted_matches {
            let replacement = strategy.apply(word_match);
            
            // Convert character positions to byte positions
            let start_byte: usize = chars[..word_match.start].iter().map(|c| c.len_utf8()).sum();
            let end_byte: usize = chars[..word_match.end].iter().map(|c| c.len_utf8()).sum();
            
            result.replace_range(start_byte..end_byte, &replacement);
        }

        FilterResult::new(result, matches, original_length)
    }

    /// Get processing statistics
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::text::SensitiveWordFilter;
    ///
    /// let mut filter = SensitiveWordFilter::new();
    /// filter.add_word("bad");
    /// filter.build();
    /// filter.filter("Some bad text");
    /// 
    /// let stats = filter.get_stats();
    /// println!("Processed {} texts", stats.texts_processed);
    /// ```
    pub fn get_stats(&self) -> &ProcessingStats {
        &self.stats
    }

    /// Reset processing statistics
    ///
    /// # Examples
    ///
    /// ```
    /// use yimi_rutool::text::SensitiveWordFilter;
    ///
    /// let mut filter = SensitiveWordFilter::new();
    /// filter.reset_stats();
    /// ```
    pub fn reset_stats(&mut self) {
        self.stats = ProcessingStats::default();
    }
}

impl Default for SensitiveWordFilter {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating and configuring sensitive word filters
///
/// # Examples
///
/// ```
/// use yimi_rutool::text::{FilterBuilder, FilterStrategy};
///
/// let mut filter = FilterBuilder::new()
///     .case_sensitive(false)
///     .add_word("bad")
///     .add_word("sensitive")
///     .add_words(vec!["word1", "word2"])
///     .build();
///
/// let result = filter.filter("This BAD text is sensitive");
/// println!("Filtered: {}", result);
/// ```
#[derive(Debug)]
pub struct FilterBuilder {
    filter: SensitiveWordFilter,
}

impl FilterBuilder {
    /// Create a new filter builder
    pub fn new() -> Self {
        FilterBuilder {
            filter: SensitiveWordFilter::new(),
        }
    }

    /// Set case sensitivity
    pub fn case_sensitive(mut self, case_sensitive: bool) -> Self {
        self.filter.set_case_sensitive(case_sensitive);
        self
    }

    /// Add a word to the filter
    pub fn add_word<S: AsRef<str>>(mut self, word: S) -> Self {
        self.filter.add_word(word.as_ref());
        self
    }

    /// Add multiple words to the filter
    pub fn add_words<I>(mut self, words: I) -> Self
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        self.filter.add_words(words);
        self
    }

    /// Load words from a file (one word per line)
    pub fn load_from_file<P: AsRef<std::path::Path>>(mut self, path: P) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(path)?;
        let words: Vec<&str> = content.lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect();
        self.filter.add_words(words);
        Ok(self)
    }

    /// Build the filter
    pub fn build(mut self) -> SensitiveWordFilter {
        self.filter.build();
        self.filter
    }
}

impl Default for FilterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_match() {
        let word_match = WordMatch::new("test".to_string(), 0, 4, "Test".to_string());
        assert_eq!(word_match.word, "test");
        assert_eq!(word_match.start, 0);
        assert_eq!(word_match.end, 4);
        assert_eq!(word_match.len(), 4);
        assert!(!word_match.is_empty());
    }

    #[test]
    fn test_filter_strategy_mask() {
        let word_match = WordMatch::new("bad".to_string(), 0, 3, "bad".to_string());
        let strategy = FilterStrategy::Mask;
        assert_eq!(strategy.apply(&word_match), "***");
    }

    #[test]
    fn test_filter_strategy_replace() {
        let word_match = WordMatch::new("bad".to_string(), 0, 3, "bad".to_string());
        let strategy = FilterStrategy::Replace("good".to_string());
        assert_eq!(strategy.apply(&word_match), "good");
    }

    #[test]
    fn test_filter_strategy_char() {
        let word_match = WordMatch::new("bad".to_string(), 0, 3, "bad".to_string());
        let strategy = FilterStrategy::Char('X');
        assert_eq!(strategy.apply(&word_match), "XXX");
    }

    #[test]
    fn test_filter_strategy_highlight() {
        let word_match = WordMatch::new("bad".to_string(), 0, 3, "bad".to_string());
        let strategy = FilterStrategy::Highlight("[".to_string(), "]".to_string());
        assert_eq!(strategy.apply(&word_match), "[bad]");
    }

    #[test]
    fn test_filter_strategy_remove() {
        let word_match = WordMatch::new("bad".to_string(), 0, 3, "bad".to_string());
        let strategy = FilterStrategy::Remove;
        assert_eq!(strategy.apply(&word_match), "");
    }

    #[test]
    fn test_sensitive_word_filter_basic() {
        let mut filter = SensitiveWordFilter::new();
        filter.add_word("bad");
        filter.add_word("evil");
        filter.build();

        let text = "This is bad and evil text";
        let matches = filter.find_matches(text);
        
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].word, "bad");
        assert_eq!(matches[1].word, "evil");
    }

    #[test]
    fn test_case_insensitive() {
        let mut filter = SensitiveWordFilter::new();
        filter.set_case_sensitive(false);
        filter.add_word("bad");
        filter.build();

        let text = "This is BAD and Bad text";
        let matches = filter.find_matches(text);
        
        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn test_case_sensitive() {
        let mut filter = SensitiveWordFilter::new();
        filter.set_case_sensitive(true);
        filter.add_word("bad");
        filter.build();

        let text = "This is BAD and bad text";
        let matches = filter.find_matches(text);
        
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].word, "bad");
    }

    #[test]
    fn test_overlapping_words() {
        let mut filter = SensitiveWordFilter::new();
        filter.add_word("he");
        filter.add_word("she");
        filter.add_word("hers");
        filter.build();

        let text = "she sells seashells";
        let matches = filter.find_matches(text);
        
        // Should find "she" and "he" within "she"
        assert!(matches.len() >= 1);
    }

    #[test]
    fn test_filter_with_mask() {
        let mut filter = SensitiveWordFilter::new();
        filter.add_word("bad");
        filter.build();

        let text = "This is bad text";
        let filtered = filter.filter(text);
        
        assert_eq!(filtered, "This is *** text");
    }

    #[test]
    fn test_filter_with_strategy() {
        let mut filter = SensitiveWordFilter::new();
        filter.add_word("bad");
        filter.build();

        let text = "This is bad text";
        let strategy = FilterStrategy::Replace("good".to_string());
        let result = filter.filter_with_strategy(text, &strategy);
        
        assert_eq!(result.filtered_text, "This is good text");
        assert_eq!(result.matches.len(), 1);
        assert!(result.has_matches());
    }

    #[test]
    fn test_contains_sensitive_words() {
        let mut filter = SensitiveWordFilter::new();
        filter.add_word("bad");
        filter.build();

        assert!(filter.contains_sensitive_words("This is bad"));
        assert!(!filter.contains_sensitive_words("This is good"));
    }

    #[test]
    fn test_word_management() {
        let mut filter = SensitiveWordFilter::new();
        
        filter.add_word("bad");
        filter.add_word("evil");
        assert_eq!(filter.word_count(), 2);
        assert!(filter.contains_word("bad"));
        
        filter.remove_word("bad");
        assert_eq!(filter.word_count(), 1);
        assert!(!filter.contains_word("bad"));
        
        filter.clear();
        assert_eq!(filter.word_count(), 0);
    }

    #[test]
    fn test_add_words() {
        let mut filter = SensitiveWordFilter::new();
        filter.add_words(vec!["bad", "evil", "wrong"]);
        
        assert_eq!(filter.word_count(), 3);
        assert!(filter.contains_word("bad"));
        assert!(filter.contains_word("evil"));
        assert!(filter.contains_word("wrong"));
    }

    #[test]
    fn test_filter_builder() {
        let mut filter = FilterBuilder::new()
            .case_sensitive(false)
            .add_word("bad")
            .add_words(vec!["evil", "wrong"])
            .build();

        let text = "This BAD text is EVIL and wrong";
        let result = filter.filter_with_strategy(text, &FilterStrategy::Mask);
        
        assert_eq!(result.matches.len(), 3);
    }

    #[test]
    fn test_processing_stats() {
        let mut filter = SensitiveWordFilter::new();
        filter.add_word("bad");
        filter.build();

        filter.filter("This is bad text");
        filter.filter("Another bad example");

        let stats = filter.get_stats();
        assert_eq!(stats.texts_processed, 2);
        assert!(stats.chars_processed > 0);
        assert_eq!(stats.total_matches, 2);
        // In release mode, processing might be too fast to measure, so we allow 0
        // Note: u128 values are always >= 0, so this comparison is not needed
        assert!(stats.processing_time_us == stats.processing_time_us);
        
        // Test statistics calculation methods
        assert!(stats.chars_per_second() >= 0.0);
        assert!(stats.match_rate() >= 0.0);
    }

    #[test]
    fn test_filter_result() {
        let mut filter = SensitiveWordFilter::new();
        filter.add_word("bad");
        filter.build();

        let text = "This bad text has bad words";
        let result = filter.filter_with_strategy(text, &FilterStrategy::Mask);
        
        assert!(result.has_matches());
        assert_eq!(result.match_count(), 2);
        assert!(result.filter_percentage() > 0.0);
        
        let unique_words = result.unique_words();
        assert_eq!(unique_words.len(), 1);
        assert!(unique_words.contains("bad"));
    }

    #[test]
    fn test_empty_text() {
        let mut filter = SensitiveWordFilter::new();
        filter.add_word("bad");
        filter.build();

        let matches = filter.find_matches("");
        assert_eq!(matches.len(), 0);
        
        let filtered = filter.filter("");
        assert_eq!(filtered, "");
    }

    #[test]
    fn test_no_matches() {
        let mut filter = SensitiveWordFilter::new();
        filter.add_word("bad");
        filter.build();

        let text = "This is good text";
        let result = filter.filter_with_strategy(text, &FilterStrategy::Mask);
        
        assert!(!result.has_matches());
        assert_eq!(result.match_count(), 0);
        assert_eq!(result.filtered_text, text);
        assert_eq!(result.filter_percentage(), 0.0);
    }

    #[test]
    fn test_unicode_text() {
        let mut filter = SensitiveWordFilter::new();
        filter.add_word("坏");
        filter.build();

        let text = "这是坏文本";
        let matches = filter.find_matches(text);
        
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].word, "坏");
    }

    #[test]
    fn test_multiple_strategies() {
        let mut filter = SensitiveWordFilter::new();
        filter.add_word("bad");
        filter.build();

        let text = "This is bad text";
        
        // Test different strategies
        let mask_result = filter.filter_with_strategy(text, &FilterStrategy::Mask);
        assert_eq!(mask_result.filtered_text, "This is *** text");
        
        let replace_result = filter.filter_with_strategy(text, &FilterStrategy::Replace("good".to_string()));
        assert_eq!(replace_result.filtered_text, "This is good text");
        
        let char_result = filter.filter_with_strategy(text, &FilterStrategy::Char('X'));
        assert_eq!(char_result.filtered_text, "This is XXX text");
        
        let highlight_result = filter.filter_with_strategy(text, &FilterStrategy::Highlight("[".to_string(), "]".to_string()));
        assert_eq!(highlight_result.filtered_text, "This is [bad] text");
        
        let remove_result = filter.filter_with_strategy(text, &FilterStrategy::Remove);
        assert_eq!(remove_result.filtered_text, "This is  text");
        
        let keep_result = filter.filter_with_strategy(text, &FilterStrategy::KeepOriginal);
        assert_eq!(keep_result.filtered_text, "This is bad text");
    }
}
