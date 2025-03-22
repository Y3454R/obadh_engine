//! Definitions for Bengali vowels
//!
//! This file contains the mappings for Bengali vowels in both their 
//! independent forms and dependent forms (vowel signs/kars).

use std::collections::HashMap;

/// A complete Bengali vowel with both independent and dependent forms
pub struct BengaliVowel {
    /// Independent form (used at word beginning or standalone)
    pub independent: &'static str,
    /// Dependent form (used after consonants as modifiers/kars)
    pub dependent: Option<&'static str>,
}

impl BengaliVowel {
    /// Create a new Bengali vowel with both forms
    pub fn new(independent: &'static str, dependent: Option<&'static str>) -> Self {
        Self { independent, dependent }
    }
}

/// Returns a map of Bengali vowels with their independent and dependent forms
pub fn vowels() -> HashMap<&'static str, BengaliVowel> {
    let mut map = HashMap::new();
    
    // Inherent vowel (no visible kar when used with consonants)
    map.insert("o", BengaliVowel::new("অ", None));
    
    // The remaining vowels have both independent and dependent forms
    map.insert("A", BengaliVowel::new("আ", Some("া")));
    map.insert("a", BengaliVowel::new("আ", Some("া")));
    map.insert("i", BengaliVowel::new("ই", Some("ি")));
    map.insert("I", BengaliVowel::new("ঈ", Some("ী")));
    map.insert("u", BengaliVowel::new("উ", Some("ু")));
    map.insert("U", BengaliVowel::new("ঊ", Some("ূ")));
    map.insert("e", BengaliVowel::new("এ", Some("ে")));
    map.insert("OI", BengaliVowel::new("ঐ", Some("ৈ")));
    map.insert("O", BengaliVowel::new("ও", Some("ো")));
    map.insert("OU", BengaliVowel::new("ঔ", Some("ৌ")));
    map.insert("rri", BengaliVowel::new("ঋ", Some("ৃ")));
    
    map
}

/// Returns only the independent vowels for convenience
pub fn independent_vowels() -> HashMap<&'static str, &'static str> {
    let vowels_map = vowels();
    let mut map = HashMap::new();
    
    for (key, value) in vowels_map.iter() {
        map.insert(*key, value.independent);
    }
    
    map
}

/// Returns only the vowel modifiers (kars) for convenience
pub fn vowel_modifiers() -> HashMap<&'static str, &'static str> {
    let vowels_map = vowels();
    let mut map = HashMap::new();
    
    for (key, value) in vowels_map.iter() {
        if let Some(dependent) = value.dependent {
            map.insert(*key, dependent);
        }
    }
    
    map
} 