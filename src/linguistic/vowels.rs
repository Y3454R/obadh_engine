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
        
        map.insert("a", "অ");
        map.insert("aa", "আ");
        map.insert("i", "ই");
        map.insert("ii", "ঈ");
        map.insert("u", "উ");
        map.insert("uu", "ঊ");
        map.insert("e", "এ");
        map.insert("oi", "ঐ");
        map.insert("o", "ও");
        map.insert("ou", "ঔ");
        map.insert("oo", "উ");  // Alternative for 'u'
        map.insert("ri", "ঋ");  // Vocalic R
        
        map
    };
    
    /// Map of Roman vowels to Bengali dependent forms
    static ref DEPENDENT_VOWEL_MAP: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        
        map.insert("a", "া");
        map.insert("aa", "া");
        map.insert("i", "ি");
        map.insert("ii", "ী");
        map.insert("u", "ু");
        map.insert("uu", "ূ");
        map.insert("e", "ে");
        map.insert("oi", "ৈ");
        map.insert("o", "ো");
        map.insert("ou", "ৌ");
        map.insert("oo", "ু");  // Alternative for 'u'
        map.insert("ri", "ৃ");  // Vocalic R
        
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
                // For inherent 'a', return empty string if handling inherent vowels
                if roman == "a" && self.handle_inherent_a {
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
        } else if roman == "a" && !is_explicit && self.handle_inherent_a {
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
}

impl Default for VowelHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vowel_forms() {
        let handler = VowelHandler::new();
        
        // Independent forms
        assert_eq!(handler.get_vowel_form("a", VowelForm::Independent), Some("অ".to_string()));
        assert_eq!(handler.get_vowel_form("i", VowelForm::Independent), Some("ই".to_string()));
        
        // Dependent forms
        assert_eq!(handler.get_vowel_form("a", VowelForm::Dependent), Some("".to_string()));  // Inherent vowel
        assert_eq!(handler.get_vowel_form("i", VowelForm::Dependent), Some("ি".to_string()));
        
        // With inherent 'a' disabled
        let handler_no_inherent = handler.with_inherent_a(false);
        assert_eq!(handler_no_inherent.get_vowel_form("a", VowelForm::Dependent), Some("া".to_string()));
    }
    
    #[test]
    fn test_determine_vowel_form() {
        let handler = VowelHandler::new();
        
        // Independent forms
        assert_eq!(handler.determine_vowel_form("a", false, false), VowelForm::Independent);
        assert_eq!(handler.determine_vowel_form("i", false, false), VowelForm::Independent);
        
        // Inherent 'a'
        assert_eq!(handler.determine_vowel_form("a", true, false), VowelForm::Inherent);
        
        // Explicit 'a' after consonant
        assert_eq!(handler.determine_vowel_form("a", true, true), VowelForm::Dependent);
        
        // Other vowels after consonant
        assert_eq!(handler.determine_vowel_form("i", true, false), VowelForm::Dependent);
    }
}