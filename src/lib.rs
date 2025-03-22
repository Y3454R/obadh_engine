//! Obadh Engine - A linguistically accurate Roman to Bengali transliteration engine
//!
//! This library provides a transliteration engine for converting Roman script
//! to Bengali script, focusing on accuracy and linguistic correctness.

pub mod definitions;
pub mod engine;

// Re-export commonly used types for convenience
pub use engine::{Sanitizer, SanitizeResult};
pub use engine::{Tokenizer, Token, TokenType, PhoneticUnit, PhoneticUnitType};

/// Main entry point for the Obadh transliteration engine
pub struct ObadhEngine {
    transliterator: engine::Transliterator,
}

impl ObadhEngine {
    /// Create a new engine with default settings
    pub fn new() -> Self {
        Self {
            transliterator: engine::Transliterator::new(),
        }
    }
    
    /// Transliterate Roman text to Bengali
    pub fn transliterate(&self, text: &str) -> String {
        self.transliterator.transliterate(text)
    }
    
    /// Sanitize input text to ensure it contains only valid characters
    pub fn sanitize(&self, text: &str) -> SanitizeResult {
        self.transliterator.sanitize(text)
    }
    
    /// Tokenize input text into words and other tokens
    pub fn tokenize(&self, text: &str) -> Vec<Token> {
        self.transliterator.tokenize(text)
    }
    
    /// Tokenize a word into phonetic units for Bengali transliteration
    pub fn tokenize_phonetic(&self, word: &str) -> Vec<PhoneticUnit> {
        self.transliterator.tokenize_phonetic(word)
    }
    
    /// Get a new tokenizer instance for custom tokenization
    pub fn get_tokenizer(&self) -> Tokenizer {
        Tokenizer::new()
    }
}

impl Default for ObadhEngine {
    fn default() -> Self {
        Self::new()
    }
}