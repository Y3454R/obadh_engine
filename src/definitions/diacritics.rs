//! Definitions for Bengali diacritics
//!
//! This file contains mappings for Bengali diacritics like
//! hasanta (virama), visarga, chandrabindu, etc.

use std::collections::HashMap;

/// Returns a map of Bengali diacritics
pub fn diacritics() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    
    // Hasanta (virama) - suppresses the inherent vowel
    // Note: In Avro, when ",," is followed by non-whitespace, it acts as "o" sound
    // and terminates both conjunct formation and vowel modification
    map.insert(",,", "্");   // Hasant/Virama
    
    // Nasalization
    map.insert("^", "ঁ");    // Chandrabindu
    
    // Other diacritics
    map.insert(":", "ঃ");    // Visarga
    map.insert("T``", "ৎ");   // Khanda Ta
    map.insert("ng", "ং");   // Khanda Ta
    
    map
} 