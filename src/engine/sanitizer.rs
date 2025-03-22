//! Input sanitization for the Obadh Engine
//!
//! This module provides functions to validate and sanitize input text
//! before passing it to the transliteration engine.

use std::collections::HashSet;

/// Result of sanitization, containing either the sanitized string or an error message
pub type SanitizeResult = Result<String, String>;

/// Sanitizer for input text
pub struct Sanitizer {
    /// Set of allowed characters
    allowed_chars: HashSet<char>,
}

impl Sanitizer {
    /// Create a new sanitizer with the default allowed character set
    pub fn new() -> Self {
        let mut allowed_chars = HashSet::new();
        
        // Add lowercase English letters (a-z)
        for c in 'a'..='z' {
            allowed_chars.insert(c);
        }
        
        // Add uppercase English letters (A-Z)
        for c in 'A'..='Z' {
            allowed_chars.insert(c);
        }
        
        // Add numerals (0-9)
        for c in '0'..='9' {
            allowed_chars.insert(c);
        }
        
        // Add common punctuation and symbols used in Avro transliteration
        for c in [' ', ',', '.', ':', ';', '!', '?', '(', ')', '[', ']', '{', '}', 
                  '"', '\'', '`', '-', '_', '+', '=', '/', '\\', '|', '@', '#', 
                  '$', '%', '^', '&', '*', '<', '>'] {
            allowed_chars.insert(c);
        }
        
        Sanitizer { allowed_chars }
    }
    
    /// Add additional allowed characters to the sanitizer
    pub fn with_allowed_chars(mut self, chars: &[char]) -> Self {
        for &c in chars {
            self.allowed_chars.insert(c);
        }
        self
    }
    
    /// Sanitize the input text, ensuring it contains only allowed characters
    ///
    /// Returns the sanitized string if successful, or an error message if invalid characters are found
    pub fn sanitize(&self, input: &str) -> SanitizeResult {
        let mut invalid_chars = HashSet::new();
        
        // Check for invalid characters
        for c in input.chars() {
            if !self.allowed_chars.contains(&c) {
                invalid_chars.insert(c);
            }
        }
        
        // If there are invalid characters, return an error
        if !invalid_chars.is_empty() {
            let invalid_list: String = invalid_chars.into_iter().collect();
            return Err(format!("Invalid characters found: {}", invalid_list));
        }
        
        // Otherwise, return the sanitized string
        Ok(input.to_string())
    }
    
    /// Remove invalid characters from the input and return the sanitized string
    pub fn clean(&self, input: &str) -> String {
        input.chars()
            .filter(|c| self.allowed_chars.contains(c))
            .collect()
    }
    
    /// Check if a string contains only valid characters
    pub fn is_valid(&self, input: &str) -> bool {
        input.chars().all(|c| self.allowed_chars.contains(&c))
    }
}

impl Default for Sanitizer {
    fn default() -> Self {
        Self::new()
    }
} 