//! Definitions for Bengali special symbols and punctuation
//!
//! This file contains mappings for Bengali special symbols and punctuation.

use std::collections::HashMap;

/// Returns a map of Bengali punctuation and special symbols
pub fn symbols() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    
    // Punctuation
    map.insert(".", "।");     // Bengali full stop (Dari)
    map.insert("$", "৳");      // BDT symbol
    
    map
} 