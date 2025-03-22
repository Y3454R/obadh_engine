//! Special transformation rules for Bengali transliteration
//!
//! This file contains a small set of compound characters that might need special handling.

use std::collections::HashMap;

/// Returns a map of special compound character combinations
pub fn special_rules() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    
    // This is a placeholder for compound consonant clusters
    // that may need special handling
    map.insert("kkh", "ক্ষ"); // khiyo (ক্ষ)
    map.insert("gg", "জ্ঞ"); // ggô (ঙ্গ)
    map.insert("hm", "হ্ম"); // hômô (হ্ম)
    map.insert("aya", "অ্যা"); // aya (অ্যা)
    map
} 