//! Models for Bengali phonemes.
//!
//! This module defines the structures for representing phonemes,
//! the basic units of sound in Bengali language.

use crate::engine::tokenizer::TokenPosition;

/// Types of phonemes in Bengali
#[derive(Debug, Clone, PartialEq)]
pub enum PhonemeType {
    /// Consonant phoneme
    Consonant,
    /// Vowel phoneme
    Vowel,
    /// Modifier (hasanta, etc.)
    Modifier,
    /// Whitespace
    Whitespace,
    /// Punctuation
    Punctuation,
    /// Number
    Number,
    /// Other type
    Other,
}

/// Represents a phoneme in the Bengali language
#[derive(Debug, Clone)]
pub struct Phoneme {
    /// The Roman representation
    pub roman: String,
    /// The Bengali representation
    pub bengali: String,
    /// The type of phoneme
    pub phoneme_type: PhonemeType,
    /// The position in context
    pub position: Option<TokenPosition>,
}

impl Phoneme {
    /// Create a new phoneme
    pub fn new(roman: &str, bengali: &str, phoneme_type: PhonemeType) -> Self {
        Phoneme {
            roman: roman.to_string(),
            bengali: bengali.to_string(),
            phoneme_type,
            position: None,
        }
    }
    
    /// Check if this is a consonant
    pub fn is_consonant(&self) -> bool {
        self.phoneme_type == PhonemeType::Consonant
    }
    
    /// Check if this is a vowel
    pub fn is_vowel(&self) -> bool {
        self.phoneme_type == PhonemeType::Vowel
    }
    
    /// Check if this is a modifier
    pub fn is_modifier(&self) -> bool {
        self.phoneme_type == PhonemeType::Modifier
    }
    
    /// Check if this is the hasanta (virama) modifier
    pub fn is_hasanta(&self) -> bool {
        self.phoneme_type == PhonemeType::Modifier && self.bengali == "্"
    }
    
    /// Check if this is whitespace
    pub fn is_whitespace(&self) -> bool {
        self.phoneme_type == PhonemeType::Whitespace
    }
    
    /// Check if this is punctuation
    pub fn is_punctuation(&self) -> bool {
        self.phoneme_type == PhonemeType::Punctuation
    }
    
    /// Get the Bengali representation
    pub fn bengali(&self) -> &str {
        &self.bengali
    }
    
    /// Get the Roman representation
    pub fn roman(&self) -> &str {
        &self.roman
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_phoneme_creation() {
        let consonant = Phoneme::new("k", "ক", PhonemeType::Consonant);
        let vowel = Phoneme::new("a", "অ", PhonemeType::Vowel);
        let modifier = Phoneme::new(".", "্", PhonemeType::Modifier);
        
        assert!(consonant.is_consonant());
        assert!(vowel.is_vowel());
        assert!(modifier.is_modifier());
        assert!(modifier.is_hasanta());
        
        assert_eq!(consonant.bengali(), "ক");
        assert_eq!(vowel.roman(), "a");
    }
}