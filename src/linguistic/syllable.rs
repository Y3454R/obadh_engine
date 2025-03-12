//! Models for Bengali syllables.
//!
//! This module defines structures for representing syllables,
//! which are the basic units of pronunciation in Bengali.

use crate::linguistic::phoneme::{Phoneme, PhonemeType};

/// Represents a syllable in Bengali
#[derive(Debug, Clone)]
pub struct Syllable {
    /// Consonants in the syllable (may be empty)
    consonants: Vec<Phoneme>,
    /// Vowel in the syllable (may be None)
    vowel: Option<Phoneme>,
    /// Modifiers in the syllable (may be empty)
    modifiers: Vec<Phoneme>,
    /// Special phonemes (whitespace, punctuation, etc.)
    special: Option<Phoneme>,
    /// Whether this syllable is standalone (not part of a word)
    is_standalone: bool,
    /// Whether this syllable has a preceding reph (র্)
    has_preceding_reph: bool,
    /// Whether this syllable has a reph (র্)
    has_reph: bool,
    /// Whether this syllable has a ya-phala (্য)
    has_ya_phala: bool,
}

impl Syllable {
    /// Create a new empty syllable
    pub fn new() -> Self {
        Syllable {
            consonants: Vec::new(),
            vowel: None,
            modifiers: Vec::new(),
            special: None,
            is_standalone: false,
            has_preceding_reph: false,
            has_reph: false,
            has_ya_phala: false,
        }
    }
    
    /// Add a consonant to the syllable
    pub fn add_consonant(&mut self, consonant: Phoneme) {
        if consonant.is_consonant() {
            self.consonants.push(consonant);
        }
    }
    
    /// Set the vowel of the syllable
    pub fn set_vowel(&mut self, vowel: Phoneme) {
        if vowel.is_vowel() {
            self.vowel = Some(vowel);
        }
    }
    
    /// Add a modifier to the syllable
    pub fn add_modifier(&mut self, modifier: Phoneme) {
        if modifier.is_modifier() {
            self.modifiers.push(modifier);
        }
    }
    
    /// Add a special phoneme (whitespace, punctuation, etc.)
    pub fn add_special(&mut self, special: Phoneme) {
        self.special = Some(special);
    }
    
    /// Set whether this syllable is standalone
    pub fn set_standalone(&mut self, standalone: bool) {
        self.is_standalone = standalone;
    }
    
    /// Check if this syllable is standalone
    pub fn is_standalone(&self) -> bool {
        self.is_standalone
    }
    
    /// Set whether this syllable has a preceding reph
    pub fn set_has_preceding_reph(&mut self, has_reph: bool) {
        self.has_preceding_reph = has_reph;
    }
    
    /// Check if this syllable has a preceding reph
    pub fn has_preceding_reph(&self) -> bool {
        self.has_preceding_reph
    }
    
    /// Set whether this syllable has a reph
    pub fn set_has_reph(&mut self, has_reph: bool) {
        self.has_reph = has_reph;
    }
    
    /// Check if this syllable has a reph
    pub fn has_reph(&self) -> bool {
        self.has_reph
    }
    
    /// Set whether this syllable has a ya-phala
    pub fn set_has_ya_phala(&mut self, has_ya_phala: bool) {
        self.has_ya_phala = has_ya_phala;
    }
    
    /// Check if this syllable has a ya-phala
    pub fn has_ya_phala(&self) -> bool {
        self.has_ya_phala
    }
    
    /// Check if this syllable has consonants
    pub fn has_consonants(&self) -> bool {
        !self.consonants.is_empty()
    }
    
    /// Check if this syllable has a vowel
    pub fn has_vowel(&self) -> bool {
        self.vowel.is_some()
    }
    
    /// Check if this syllable has modifiers
    pub fn has_modifiers(&self) -> bool {
        !self.modifiers.is_empty()
    }
    
    /// Check if this syllable has a hasanta
    pub fn has_hasanta(&self) -> bool {
        self.modifiers.iter().any(|m| m.is_hasanta())
    }
    
    /// Check if this syllable is empty
    pub fn is_empty(&self) -> bool {
        self.consonants.is_empty() && self.vowel.is_none() && 
        self.modifiers.is_empty() && self.special.is_none()
    }
    
    /// Check if this syllable is a special syllable
    pub fn is_special(&self) -> bool {
        self.special.is_some()
    }
    
    /// Get the special text (if this is a special syllable)
    pub fn get_special_text(&self) -> String {
        if let Some(ref special) = self.special {
            special.bengali().to_string()
        } else {
            String::new()
        }
    }
    
    /// Get the consonant text
    pub fn get_consonant_text(&self) -> String {
        self.consonants.iter()
            .map(|c| c.bengali())
            .collect()
    }
    
    /// Get the vowel text
    pub fn get_vowel_text(&self) -> String {
        if let Some(ref vowel) = self.vowel {
            vowel.bengali().to_string()
        } else {
            String::new()
        }
    }
    
    /// Get the vowel's Roman representation
    pub fn get_vowel_roman(&self) -> String {
        if let Some(ref vowel) = self.vowel {
            vowel.roman().to_string()
        } else {
            String::new()
        }
    }
    
    /// Get the modifier text
    pub fn get_modifier_text(&self) -> String {
        self.modifiers.iter()
            .map(|m| m.bengali())
            .collect()
    }
    
    /// Check if this syllable has a specific consonant sequence
    pub fn has_consonant_sequence(&self, first: &str, second: &str) -> bool {
        for i in 0..self.consonants.len() {
            if self.consonants[i].bengali() == first {
                if i + 1 < self.modifiers.len() && self.modifiers[0].bengali() == second {
                    return true;
                }
            }
        }
        
        false
    }
    
    /// Remove a specific consonant sequence
    pub fn remove_consonant_sequence(&mut self, first: &str, second: &str) {
        let mut i = 0;
        while i < self.consonants.len() {
            if self.consonants[i].bengali() == first {
                if i + 1 < self.modifiers.len() && self.modifiers[0].bengali() == second {
                    self.consonants.remove(i);
                    self.modifiers.remove(0);
                    break;
                }
            }
            i += 1;
        }
    }
}

impl Default for Syllable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_syllable_creation() {
        let mut syllable = Syllable::new();
        
        let consonant = Phoneme::new("k", "ক", PhonemeType::Consonant);
        let vowel = Phoneme::new("a", "া", PhonemeType::Vowel);
        
        syllable.add_consonant(consonant);
        syllable.set_vowel(vowel);
        
        assert!(syllable.has_consonants());
        assert!(syllable.has_vowel());
        assert_eq!(syllable.get_consonant_text(), "ক");
        assert_eq!(syllable.get_vowel_text(), "া");
    }
    
    #[test]
    fn test_special_syllable() {
        let mut syllable = Syllable::new();
        
        let space = Phoneme::new(" ", " ", PhonemeType::Whitespace);
        syllable.add_special(space);
        
        assert!(syllable.is_special());
        assert_eq!(syllable.get_special_text(), " ");
    }
}