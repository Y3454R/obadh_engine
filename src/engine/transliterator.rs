//! Main transliteration engine that combines tokenization and phonology.
//!
//! This module contains the core logic for transliterating Roman text to Bengali.

use crate::engine::tokenizer::{Tokenizer, Token};
use crate::engine::phonology::PhonologyEngine;
use crate::linguistic::phoneme::Phoneme;
use crate::linguistic::syllable::Syllable;

/// Stores the complete analysis of the transliteration process
/// for debugging and visualization purposes
#[derive(Debug, Clone)]
pub struct TransliterationAnalysis {
    /// The original input text
    pub input: String,
    /// The tokens extracted from input
    pub tokens: Vec<Token>,
    /// The phonemes derived from tokens
    pub phonemes: Vec<Phoneme>,
    /// The syllables formed from phonemes
    pub syllables: Vec<Syllable>,
    /// The final Bengali output
    pub output: String,
}

/// The main transliteration engine
pub struct Transliterator {
    tokenizer: Tokenizer,
    phonology_engine: PhonologyEngine,
}

impl Transliterator {
    /// Create a new transliterator with default settings
    pub fn new() -> Self {
        Transliterator {
            tokenizer: Tokenizer::new(),
            phonology_engine: PhonologyEngine::new(),
        }
    }
    
    /// Configure the tokenizer
    pub fn with_tokenizer(mut self, tokenizer: Tokenizer) -> Self {
        self.tokenizer = tokenizer;
        self
    }
    
    /// Configure the phonology engine
    pub fn with_phonology_engine(mut self, engine: PhonologyEngine) -> Self {
        self.phonology_engine = engine;
        self
    }
    
    /// Transliterate Roman text to Bengali
    ///
    /// This is the main entry point for the transliteration process.
    ///
    /// # Arguments
    ///
    /// * `text` - The Roman text to transliterate
    ///
    /// # Returns
    ///
    /// The transliterated Bengali text
    pub fn transliterate(&self, text: &str) -> String {
        // 1. Tokenize the input
        let tokens = self.tokenizer.tokenize(text);
        
        // 2. Convert tokens to phonemes
        let phonemes = self.phonology_engine.tokens_to_phonemes(&tokens);
        
        // 3. Organize phonemes into syllables
        let syllables = self.phonology_engine.organize_into_syllables(&phonemes);
        
        // 4. Format syllables into Bengali text
        self.phonology_engine.format_syllables(&syllables)
    }
    
    /// Get a detailed analysis of the transliteration process
    ///
    /// This is useful for debugging and understanding the
    /// steps involved in transliteration.
    ///
    /// # Arguments
    ///
    /// * `text` - The Roman text to analyze
    ///
    /// # Returns
    ///
    /// A detailed breakdown of the transliteration process
    pub fn analyze(&self, text: &str) -> TransliterationAnalysis {
        // 1. Tokenize the input
        let tokens = self.tokenizer.tokenize(text);
        
        // 2. Convert tokens to phonemes
        let phonemes = self.phonology_engine.tokens_to_phonemes(&tokens);
        
        // 3. Organize phonemes into syllables
        let syllables = self.phonology_engine.organize_into_syllables(&phonemes);
        
        // 4. Format syllables into Bengali text
        let output = self.phonology_engine.format_syllables(&syllables);
        
        TransliterationAnalysis {
            input: text.to_string(),
            tokens,
            phonemes,
            syllables,
            output,
        }
    }
}

impl Default for Transliterator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_transliterate_simple() {
        let transliterator = Transliterator::new();
        
        assert_eq!(transliterator.transliterate("ami"), "আমি");
        assert_eq!(transliterator.transliterate("bangla"), "বাংলা");
        assert_eq!(transliterator.transliterate("boi"), "বই");
    }
    
    #[test]
    fn test_transliterate_complex() {
        let transliterator = Transliterator::new();
        
        assert_eq!(transliterator.transliterate("bidyaloy"), "বিদ্যালয়");
        assert_eq!(transliterator.transliterate("shudhu"), "শুধু");
        assert_eq!(transliterator.transliterate("shrishti"), "সৃষ্টি");
    }
    
    #[test]
    fn test_transliterate_with_punctuation_and_numbers() {
        let transliterator = Transliterator::new();
        
        assert_eq!(
            transliterator.transliterate("ami 123 taka debo."),
            "আমি ১২৩ টাকা দেবো।"
        );
        
        assert_eq!(
            transliterator.transliterate("tumi ki bhalo acho?"),
            "তুমি কি ভালো আছো?"
        );
    }
    
    #[test]
    fn test_analysis() {
        let transliterator = Transliterator::new();
        let analysis = transliterator.analyze("ami");
        
        assert_eq!(analysis.input, "ami");
        assert_eq!(analysis.tokens.len(), 3);
        assert_eq!(analysis.phonemes.len(), 3);
        assert_eq!(analysis.output, "আমি");
    }
}