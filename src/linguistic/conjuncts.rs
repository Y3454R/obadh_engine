//! Algorithms for Bengali conjunct formation.
//!
//! This module provides algorithms for handling the complex
//! conjunct (juktakkhor) system in Bengali orthography.

use std::collections::HashMap;
use lazy_static::lazy_static;

/// Types of conjuncts in Bengali
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConjunctType {
    /// Regular stacked conjuncts
    Regular,
    /// Transparent conjuncts (first consonant visible)
    Transparent,
    /// Special forms with unique shapes
    Special,
}

/// Direction of reph (র্) rendering
#[derive(Debug, Clone, PartialEq)]
pub enum RephDirection {
    /// Reph appears before the consonant
    Before,
    /// Reph appears after the consonant
    After,
    /// Reph appears above the consonant
    Above,
}

lazy_static! {
    /// Special conjunct forms that can't be derived algorithmically
    static ref SPECIAL_CONJUNCTS: HashMap<String, String> = {
        let mut map = HashMap::new();
        
        // Truly exceptional forms
        map.insert("ক্ষ".to_string(), "ক্ষ".to_string());
        map.insert("জ্ঞ".to_string(), "জ্ঞ".to_string());
        
        map
    };
    
    /// Patterns for ya-phala (্য) transformations
    static ref YAPHALA_PATTERNS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        
        // Consonants that have special ya-phala forms
        map.insert("ক", "ক্য");
        map.insert("খ", "খ্য");
        map.insert("গ", "গ্য");
        map.insert("ঘ", "ঘ্য");
        map.insert("চ", "চ্য");
        map.insert("ছ", "ছ্য");
        map.insert("জ", "জ্য");
        map.insert("ঝ", "ঝ্য");
        map.insert("ট", "ট্য");
        map.insert("ঠ", "ঠ্য");
        map.insert("ড", "ড্য");
        map.insert("ঢ", "ঢ্য");
        map.insert("ত", "ত্য");
        map.insert("থ", "থ্য");
        map.insert("দ", "দ্য");
        map.insert("ধ", "ধ্য");
        map.insert("ন", "ন্য");
        map.insert("প", "প্য");
        map.insert("ফ", "ফ্য");
        map.insert("ব", "ব্য");
        map.insert("ভ", "ভ্য");
        map.insert("ম", "ম্য");
        map.insert("য", "য্য");
        map.insert("র", "র্য");
        map.insert("ল", "ল্য");
        map.insert("শ", "শ্য");
        map.insert("ষ", "ষ্য");
        map.insert("স", "স্য");
        map.insert("হ", "হ্য");
        
        map
    };
    
    /// Patterns for ra-phala (্র) transformations
    static ref RAPHALA_PATTERNS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        
        // Consonants that have special ra-phala forms
        map.insert("ক", "ক্র");
        map.insert("খ", "খ্র");
        map.insert("গ", "গ্র");
        map.insert("ঘ", "ঘ্র");
        map.insert("চ", "চ্র");
        map.insert("ছ", "ছ্র");
        map.insert("জ", "জ্র");
        map.insert("ঝ", "ঝ্র");
        map.insert("ট", "ট্র");
        map.insert("ঠ", "ঠ্র");
        map.insert("ড", "ড্র");
        map.insert("ঢ", "ঢ্র");
        map.insert("ত", "ত্র");
        map.insert("থ", "থ্র");
        map.insert("দ", "দ্র");
        map.insert("ধ", "ধ্র");
        map.insert("ন", "ন্র");
        map.insert("প", "প্র");
        map.insert("ফ", "ফ্র");
        map.insert("ব", "ব্র");
        map.insert("ভ", "ভ্র");
        map.insert("ম", "ম্র");
        map.insert("ল", "ল্র");
        map.insert("শ", "শ্র");
        map.insert("ষ", "ষ্র");
        map.insert("স", "স্র");
        map.insert("হ", "হ্র");
        
        map
    };
}

/// Handler for Bengali conjunct formation
pub struct ConjunctHandler {
    // Configuration options
    allow_special_forms: bool,
    reph_direction: RephDirection,
}

impl ConjunctHandler {
    /// Create a new conjunct handler with default settings
    pub fn new() -> Self {
        ConjunctHandler {
            allow_special_forms: true,
            reph_direction: RephDirection::Before,
        }
    }
    
    /// Configure whether to use special conjunct forms
    pub fn with_special_forms(mut self, allow: bool) -> Self {
        self.allow_special_forms = allow;
        self
    }
    
    /// Configure the direction of reph rendering
    pub fn with_reph_direction(mut self, direction: RephDirection) -> Self {
        self.reph_direction = direction;
        self
    }
    
    /// Determine if a sequence can form a valid conjunct
    pub fn can_form_conjunct(&self, first: &str, second: &str) -> bool {
        // In Bengali, almost any pair of consonants can form a conjunct
        // with a hasanta (virama) in between
        // Some exceptions and special cases are handled separately
        
        // Check if this is a special conjunct
        if self.allow_special_forms {
            let combined = format!("{}{}{}", first, "্", second);
            if SPECIAL_CONJUNCTS.contains_key(&combined) {
                return true;
            }
        }
        
        // Check if first is Bengali consonant
        let is_first_consonant = self.is_bengali_consonant(first);
        
        // Check if second is Bengali consonant
        let is_second_consonant = self.is_bengali_consonant(second);
        
        is_first_consonant && is_second_consonant
    }
    
    /// Check if a string is a Bengali consonant
    pub fn is_bengali_consonant(&self, s: &str) -> bool {
        if s.chars().count() != 1 {
            return false;
        }
        
        let c = s.chars().next().unwrap();
        // Bengali consonants are in the range U+0995 to U+09B9
        (c >= '\u{0995}' && c <= '\u{09B9}') || c == '\u{09CE}' || c == '\u{09DC}' || c == '\u{09DD}' || c == '\u{09DF}'
    }
    
    /// Form a conjunct from two consonants
    pub fn form_conjunct(&self, first: &str, second: &str) -> String {
        if !self.can_form_conjunct(first, second) {
            // Return the two consonants unchanged
            return format!("{}{}", first, second);
        }
        
        // Check for special forms
        if self.allow_special_forms {
            let combined = format!("{}{}{}", first, "্", second);
            if let Some(special) = SPECIAL_CONJUNCTS.get(&combined) {
                return special.clone();
            }
        }
        
        // Regular conjunct formation: first + hasanta + second
        format!("{}{}{}", first, "্", second)
    }
    
    /// Handle ya-phala (্য) formation
    pub fn form_yaphala(&self, consonant: &str) -> String {
        if let Some(yaphala) = YAPHALA_PATTERNS.get(consonant) {
            yaphala.to_string()
        } else {
            // Default formation
            format!("{}্য", consonant)
        }
    }
    
    /// Handle ra-phala (্র) formation
    pub fn form_raphala(&self, consonant: &str) -> String {
        if let Some(raphala) = RAPHALA_PATTERNS.get(consonant) {
            raphala.to_string()
        } else {
            // Default formation
            format!("{}্র", consonant)
        }
    }
    
    /// Handle reph (র্) formation with correct positioning
    pub fn form_reph(&self, with_consonant: &str) -> String {
        match self.reph_direction {
            RephDirection::Before => format!("র্{}", with_consonant),
            RephDirection::After => format!("{}র্", with_consonant),
            RephDirection::Above => format!("{}র্", with_consonant), // Simplified - would need font support
        }
    }
}

impl Default for ConjunctHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_can_form_conjunct() {
        let handler = ConjunctHandler::new();
        
        assert!(handler.can_form_conjunct("ক", "ত"));
        assert!(handler.can_form_conjunct("স", "ত"));
        assert!(handler.can_form_conjunct("ক", "ষ")); // Special conjunct ক্ষ
    }
    
    #[test]
    fn test_form_conjunct() {
        let handler = ConjunctHandler::new();
        
        assert_eq!(handler.form_conjunct("ক", "ত"), "ক্ত");
        assert_eq!(handler.form_conjunct("ন", "ত"), "ন্ত");
        assert_eq!(handler.form_conjunct("ক", "ষ"), "ক্ষ"); // Special conjunct
    }
    
    #[test]
    fn test_phala_formation() {
        let handler = ConjunctHandler::new();
        
        assert_eq!(handler.form_yaphala("ব"), "ব্য");
        assert_eq!(handler.form_raphala("প"), "প্র");
    }
    
    #[test]
    fn test_reph_formation() {
        let handler = ConjunctHandler::new();
        
        assert_eq!(handler.form_reph("ম"), "র্ম");
        
        let handler_after = handler.with_reph_direction(RephDirection::After);
        assert_eq!(handler_after.form_reph("ম"), "মর্");
    }
}