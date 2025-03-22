//! Handling of special cases in Bengali orthography.
//!
//! This module handles special cases that require specific
//! treatment beyond regular phonological rules.

use std::collections::HashMap;
use lazy_static::lazy_static;

/// Special transformation types in Bengali
#[derive(Debug, Clone, PartialEq)]
pub enum SpecialTransform {
    /// Zero-width joiner (ZWJ)
    ZWJ,
    /// Zero-width non-joiner (ZWNJ)
    ZWNJ,
    /// Khanda Ta (ৎ)
    KhandaTa,
    /// Visarga (ঃ)
    Visarga,
    /// Chandrabindu (ঁ)
    Chandrabindu,
    /// Anusvar (ং)
    Anusvar,
}

lazy_static! {
    /// Map of Bengali special characters
    static ref SPECIAL_CHAR_MAP: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        
        map.insert("t.", "ৎ");  // Khanda Ta
        map.insert("h.", "ঃ");  // Visarga
        map.insert("n.", "ং");  // Anusvar
        map.insert("m.", "ং");  // Alternative for Anusvar
        map.insert("^", "ঁ");   // Chandrabindu
        map.insert("~", "ঁ");   // Alternative for Chandrabindu
        
        map
    };
    
    /// Map of zero-width joiner patterns
    static ref ZWJ_PATTERNS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        
        // Patterns that require ZWJ to render correctly
        map.insert("র্‍য", "র্‍য");  // Reph + Ya
        map.insert("র্‍র", "র্‍র");  // Reph + Ra
        
        map
    };
    
    /// Map of zero-width non-joiner patterns
    static ref ZWNJ_PATTERNS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        
        // Patterns that require ZWNJ to prevent conjunct formation
        map.insert("ন্‌ন", "ন্‌ন");  // N + ZWNJ + N
        map.insert("স্‌প", "স্‌প");  // S + ZWNJ + P
        
        map
    };
}

/// Handler for special transformations in Bengali
pub struct SpecialHandler {
    // Configuration options
    support_zwj: bool,
    support_zwnj: bool,
}

impl SpecialHandler {
    /// Create a new special handler with default settings
    pub fn new() -> Self {
        SpecialHandler {
            support_zwj: true,
            support_zwnj: true,
        }
    }
    
    /// Configure whether to support zero-width joiner
    pub fn with_zwj(mut self, enable: bool) -> Self {
        self.support_zwj = enable;
        self
    }
    
    /// Configure whether to support zero-width non-joiner
    pub fn with_zwnj(mut self, enable: bool) -> Self {
        self.support_zwnj = enable;
        self
    }
    
    /// Get a special character mapping
    pub fn get_special_char(&self, roman: &str) -> Option<String> {
        SPECIAL_CHAR_MAP.get(roman).map(|s| s.to_string())
    }
    
    /// Apply ZWJ to a sequence if needed
    pub fn apply_zwj(&self, sequence: &str) -> String {
        if !self.support_zwj {
            return sequence.to_string();
        }
        
        if let Some(zwj_sequence) = ZWJ_PATTERNS.get(sequence) {
            zwj_sequence.to_string()
        } else {
            sequence.to_string()
        }
    }
    
    /// Apply ZWNJ to a sequence if needed
    pub fn apply_zwnj(&self, sequence: &str) -> String {
        if !self.support_zwnj {
            return sequence.to_string();
        }
        
        if let Some(zwnj_sequence) = ZWNJ_PATTERNS.get(sequence) {
            zwnj_sequence.to_string()
        } else {
            sequence.to_string()
        }
    }
    
    /// Add Anusvar (ং) to text
    pub fn add_anusvar(&self, text: &str) -> String {
        format!("{}ং", text)
    }
    
    /// Add Chandrabindu (ঁ) to text
    pub fn add_chandrabindu(&self, text: &str) -> String {
        format!("{}ঁ", text)
    }
    
    /// Add Visarga (ঃ) to text
    pub fn add_visarga(&self, text: &str) -> String {
        format!("{}ঃ", text)
    }
    
    /// Process special sequences algorithmically
    pub fn process_special_sequences(&self, text: &str) -> String {
        let mut result = text.to_string();
        
        // Replace ZWJ sequences
        if self.support_zwj {
            for (pattern, replacement) in ZWJ_PATTERNS.iter() {
                result = result.replace(pattern, replacement);
            }
        }
        
        // Replace ZWNJ sequences
        if self.support_zwnj {
            for (pattern, replacement) in ZWNJ_PATTERNS.iter() {
                result = result.replace(pattern, replacement);
            }
        }
        
        result
    }
}

impl Default for SpecialHandler {
    fn default() -> Self {
        Self::new()
    }
}