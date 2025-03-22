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
    /// The attached vowel diacritic (for consonants)
    pub vowel: Option<String>,
    /// Whether this phoneme is part of a consonant conjunct
    pub is_conjunct_former: bool,
    /// Whether this phoneme has a vowel after its conjunct
    pub has_vowel_after_conjunct: bool,
    /// Whether this phoneme has ya-phala (jofola)
    pub has_ya_phala: bool,
    /// Whether this phoneme has bo-fola (bo-fola)
    pub has_bo_fola: bool,
    /// Whether this phoneme is a reph (র্)
    pub is_reph: bool,
}

impl Phoneme {
    /// Create a new phoneme
    pub fn new(
        bengali: String, 
        phoneme_type: PhonemeType,
        position: Option<TokenPosition>
    ) -> Self {
        Phoneme {
            roman: String::new(), // Not needed with the new implementation
            bengali,
            phoneme_type,
            position,
            vowel: None,
            is_conjunct_former: false,
            has_vowel_after_conjunct: false,
            has_ya_phala: false,
            has_bo_fola: false,
            is_reph: false,
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