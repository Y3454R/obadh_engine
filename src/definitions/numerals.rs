//! Definitions for Bengali numerals
//!
//! This file contains mappings for Bengali numerals (০-৯).

use std::collections::HashMap;

/// Returns a map of Latin numerals to Bengali numerals
pub fn numerals() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    
    // Map Latin digits to Bengali digits
    map.insert("0", "০");
    map.insert("1", "১");
    map.insert("2", "২");
    map.insert("3", "৩");
    map.insert("4", "৪");
    map.insert("5", "৫");
    map.insert("6", "৬");
    map.insert("7", "৭");
    map.insert("8", "৮");
    map.insert("9", "৯");
    
    map
} 