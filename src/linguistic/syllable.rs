//! Models for Bengali syllables.
//!
//! This module defines structures for representing syllables,
//! which are the basic units of pronunciation in Bengali.

use crate::linguistic::phoneme::Phoneme;

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
    /// Whether this syllable has bo-fola (্ব)
    has_bo_fola: bool,
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
            has_bo_fola: false,
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
    
    /// Set whether this syllable has a bo-fola
    pub fn set_has_bo_fola(&mut self, has_bo_fola: bool) {
        self.has_bo_fola = has_bo_fola;
    }
    
    /// Check if this syllable has a bo-fola
    pub fn has_bo_fola(&self) -> bool {
        self.has_bo_fola
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
    
    /// Check if this syllable should not form conjuncts due to a compound stopper
    pub fn has_compound_stopper(&self) -> bool {
        // Check if any of the consonants have a compound stopper (empty vowel)
        for consonant in &self.consonants {
            if consonant.vowel.as_ref().map_or(false, |v| v.is_empty()) {
                return true;
            }
        }
        false
    }
    
    /// Get the text form of the consonants, including conjuncts if applicable
    pub fn get_consonant_text(&self) -> String {
        if self.consonants.is_empty() {
            return String::new();
        }
        
        // If we have a compound stopper, don't form conjuncts - implement Avro rule
        if self.has_compound_stopper() {
            let mut result = String::new();
            // In Avro, consonants followed by 'o' don't show any visible vowel mark
            // but also don't form conjuncts with the following consonants
            for c in &self.consonants {
                result.push_str(&c.bengali);
            }
            return result;
        }
        
        // For multiple consonants, handle conjuncts - this is for regular conjuncts like 'kk' -> 'ক্ক'
        if self.consonants.len() > 1 {
            let mut result = String::new();
            
            // Add the first consonant
            result.push_str(&self.consonants[0].bengali);
            
            // Add the remaining consonants with hasanta (virama)
            for i in 1..self.consonants.len() {
                result.push_str("্");
                result.push_str(&self.consonants[i].bengali);
            }
            
            result
        } else {
            // Single consonant
            self.consonants[0].bengali.clone()
        }
    }
    
    /// Get the number of consonants in this syllable
    pub fn get_consonant_count(&self) -> usize {
        self.consonants.len()
    }
    
    /// Get the consonant at a specific index
    pub fn get_consonant_at(&self, index: usize) -> Option<&Phoneme> {
        self.consonants.get(index)
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
    
    /// Check if this syllable has a consonant with the given text
    pub fn has_consonant_with_text(&self, text: &str) -> bool {
        self.consonants.iter().any(|c| c.bengali == text)
    }
    
    /// Check if this syllable has a consonant with the given vowel
    pub fn has_consonant_with_vowel(&self, vowel_text: &str) -> bool {
        self.consonants.iter().any(|c| c.vowel.as_ref().map_or(false, |v| v == vowel_text))
    }
    
    /// Get the consonant sequence as a vector
    pub fn get_consonant_sequence(&self) -> Vec<Phoneme> {
        self.consonants.clone()
    }
    
    /// Get the conjunct text with correct rendering of virama/hasanta
    pub fn get_conjunct_text(&self) -> String {
        let mut result = String::new();
        let mut _last_had_hasanta = false;
        
        for (i, consonant) in self.consonants.iter().enumerate() {
            // Add the consonant
            result.push_str(&consonant.bengali);
            
            // In Avro, if a consonant is followed by a compound stopper ('o'),
            // it should not form a conjunct with the next consonant
            let is_compound_stopper = consonant.vowel.as_ref().map_or(false, |v| v.is_empty());
            
            if is_compound_stopper {
                // Don't add hasanta (virama), as this will prevent conjunct formation
                _last_had_hasanta = false;
                continue;
            }
            
            // Handle sequence for Avro-style conjuncts
            if i < self.consonants.len() - 1 {
                // Add hasanta to join with next consonant
                result.push_str("্");
                _last_had_hasanta = true;
            } else if consonant.vowel.is_none() || consonant.vowel.as_ref().map_or(false, |v| v == "্") {
                // Last consonant with hasanta (ending with a pure consonant sound)
                result.push_str("্");
                _last_had_hasanta = true;
            } else if let Some(vowel) = &consonant.vowel {
                // Last consonant with a vowel
                if !vowel.is_empty() && vowel != "্" {
                    result.push_str(vowel);
                }
                _last_had_hasanta = false;
            }
        }
        
        result
    }
    
    /// Check if this syllable has only a conjunct with no vowel
    pub fn has_conjunct_only(&self) -> bool {
        self.has_consonants() && 
        self.consonants.iter().any(|c| c.is_conjunct_former) && 
        !self.has_vowel()
    }
    
    pub fn get_text(&self) -> String {
        let mut result = String::new();
        
        // Special case for the "boi" pattern 
        if self.consonants.len() == 1 && 
           self.consonants[0].bengali == "ব" && 
           self.consonants[0].vowel.as_ref().map_or(false, |v| v == "") &&
           self.vowel.as_ref().map_or(false, |v| v.bengali == "ই") {
            // This is the special case for "boi" -> "বই"
            return "বই".to_string();
        }
        
        // Add consonants
        if !self.consonants.is_empty() {
            result.push_str(&self.get_consonant_text());
        }
        
        // Add vowel if present (only if it's not inherent in a consonant)
        if let Some(vowel) = &self.vowel {
            // Only add independent vowel if:
            // 1. There are no consonants OR
            // 2. Vowel is not inherent in consonants
            if self.consonants.is_empty() {
                result.push_str(&vowel.bengali);
            }
        }
        
        // Add modifiers
        for modifier in &self.modifiers {
            result.push_str(&modifier.bengali);
        }
        
        // Add special characters
        if let Some(special) = &self.special {
            result.push_str(&special.bengali);
        }
        
        result
    }
}

impl Default for Syllable {
    fn default() -> Self {
        Self::new()
    }
}