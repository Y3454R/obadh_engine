//! Implementation of Bengali vowel processing rules.
//!
//! This module handles the special rules for vowels in different
//! contexts in Bengali orthography.

use std::collections::HashMap;
use lazy_static::lazy_static;

/// Vowel form type in Bengali
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VowelForm {
    /// Independent form (e.g., অ, আ)
    Independent,
    /// Dependent form (e.g., া, ি)
    Dependent,
    /// Inherent form (implicit vowel 'অ' after consonant)
    Inherent,
}

lazy_static! {
    /// Map of Roman vowels to Bengali independent forms
    static ref INDEPENDENT_VOWEL_MAP: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        
        // Basic vowels as per the documentation
        map.insert("o", "অ");     // অ-কার (a-kar)
        map.insert("A", "আ");     // আ-কার (aa-kar)
        map.insert("i", "ই");     // ই-কার (i-kar)
        map.insert("I", "ঈ");     // ঈ-কার (dirgho i-kar)
        map.insert("u", "উ");     // উ-কার (u-kar)
        map.insert("U", "ঊ");     // ঊ-কার (dirgho u-kar)
        map.insert("e", "এ");     // এ-কার (e-kar)
        map.insert("OI", "ঐ");    // ঐ-কার (oi-kar)
        map.insert("O", "ও");     // ও-কার (o-kar)
        map.insert("OU", "ঔ");    // ঔ-কার (ou-kar)
        map.insert("rri", "ঋ");   // ঋ-কার (ri-kar)
        
        // Common alternative spellings for backward compatibility
        map.insert("a", "আ");     // Equivalent to 'A'
        map.insert("aa", "আ");    // Equivalent to 'A'
        map.insert("oi", "ঐ");    // Equivalent to 'OI'
        map.insert("ou", "ঔ");    // Equivalent to 'OU'
        
        map
    };
    
    /// Map of Roman vowels to Bengali dependent forms
    static ref DEPENDENT_VOWEL_MAP: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        
        // Basic vowels as per the documentation
        map.insert("o", "");      // Inherent vowel (no visible kar)
        map.insert("A", "া");     // আ-কার (aa-kar)
        map.insert("i", "ি");     // ই-কার (i-kar)
        map.insert("I", "ী");     // ঈ-কার (dirgho i-kar)
        map.insert("u", "ু");     // উ-কার (u-kar)
        map.insert("U", "ূ");     // ঊ-কার (dirgho u-kar)
        map.insert("e", "ে");     // এ-কার (e-kar)
        map.insert("OI", "ৈ");    // ঐ-কার (oi-kar)
        map.insert("O", "ো");     // ও-কার (o-kar)
        map.insert("OU", "ৌ");    // ঔ-কার (ou-kar)
        map.insert("rri", "ৃ");   // ঋ-কার (ri-kar)
        
        // Common alternative spellings for backward compatibility
        map.insert("a", "া");     // Equivalent to 'A'
        map.insert("aa", "া");    // Equivalent to 'A'
        map.insert("oi", "ৈ");    // Equivalent to 'OI'
        map.insert("ou", "ৌ");    // Equivalent to 'OU'
        
        map
    };
    
    /// Map for vowel+vowel combinations
    static ref VOWEL_VOWEL_COMBINATIONS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        
        // As specified in the documentation
        map.insert("aa", "আ");   // a + a
        map.insert("ai", "আই");  // a + i
        map.insert("au", "আউ");  // a + u
        map.insert("ae", "আএ");  // a + e
        map.insert("ao", "আও");  // a + o
        map.insert("ia", "ইয়া"); // i + a
        map.insert("io", "ইও");  // i + o
        map.insert("eo", "এও");  // e + o
        
        map
    };
}

/// Handler for Bengali vowel processing
pub struct VowelHandler {
    // Configuration options
    handle_inherent_a: bool,
}

impl VowelHandler {
    /// Create a new vowel handler with default settings
    pub fn new() -> Self {
        VowelHandler {
            handle_inherent_a: true,
        }
    }
    
    /// Configure whether to handle inherent vowel 'অ'
    pub fn with_inherent_a(mut self, enable: bool) -> Self {
        self.handle_inherent_a = enable;
        self
    }
    
    /// Get the Bengali form of a vowel
    pub fn get_vowel_form(&self, roman: &str, form: VowelForm) -> Option<String> {
        match form {
            VowelForm::Independent => {
                INDEPENDENT_VOWEL_MAP.get(roman).map(|s| s.to_string())
            },
            VowelForm::Dependent => {
                // For inherent 'o', return empty string if handling inherent vowels
                if (roman == "o") && self.handle_inherent_a {
                    Some(String::new())
                } else {
                    DEPENDENT_VOWEL_MAP.get(roman).map(|s| s.to_string())
                }
            },
            VowelForm::Inherent => {
                // Inherent form is empty (implicit 'অ')
                Some(String::new())
            },
        }
    }
    
    /// Determine the appropriate vowel form based on context
    pub fn determine_vowel_form(&self, roman: &str, after_consonant: bool, is_explicit: bool) -> VowelForm {
        if !after_consonant {
            // Vowel at the beginning of a word or standalone
            VowelForm::Independent
        } else if roman == "o" && !is_explicit && self.handle_inherent_a {
            // Implicit 'অ' after consonant
            VowelForm::Inherent
        } else {
            // Vowel after consonant
            VowelForm::Dependent
        }
    }
    
    /// Check if a Roman string represents a vowel
    pub fn is_vowel(&self, roman: &str) -> bool {
        INDEPENDENT_VOWEL_MAP.contains_key(roman)
    }
    
    /// Get all vowel strings that should be recognized
    pub fn get_vowel_strings(&self) -> Vec<&'static str> {
        INDEPENDENT_VOWEL_MAP.keys().cloned().collect()
    }
    
    /// Check if a string is a valid vowel+vowel combination
    pub fn is_vowel_vowel_combination(&self, roman: &str) -> bool {
        VOWEL_VOWEL_COMBINATIONS.contains_key(roman)
    }
    
    /// Get the Bengali form of a vowel+vowel combination
    pub fn get_vowel_vowel_combination(&self, roman: &str) -> Option<String> {
        VOWEL_VOWEL_COMBINATIONS.get(roman).map(|s| s.to_string())
    }
    
    /// Get all valid vowel+vowel combinations
    pub fn get_vowel_vowel_combinations(&self) -> Vec<&'static str> {
        VOWEL_VOWEL_COMBINATIONS.keys().cloned().collect()
    }
}

impl Default for VowelHandler {
    fn default() -> Self {
        Self::new()
    }
}