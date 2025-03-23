//! Definitions for Bengali consonants
//!
//! This file contains the mappings for Bengali consonants, organized by their phonetic categories,
//! as well as information about conjunct formation.

use std::collections::HashMap;

/// Organizes consonants by their phonetic groups (vargas) and characteristics
pub struct ConsonantSystem {
    /// Velar consonants (k-varga)
    pub velars: Vec<(&'static str, &'static str)>,
    /// Palatal consonants (c-varga)
    pub palatals: Vec<(&'static str, &'static str)>,
    /// Retroflex consonants (ṭ-varga)
    pub retroflexes: Vec<(&'static str, &'static str)>,
    /// Dental consonants (t-varga)
    pub dentals: Vec<(&'static str, &'static str)>,
    /// Labial consonants (p-varga)
    pub labials: Vec<(&'static str, &'static str)>,
    /// Semivowels and liquids
    pub semivowels: Vec<(&'static str, &'static str)>,
    /// Fricatives and others
    pub fricatives: Vec<(&'static str, &'static str)>,
    /// Special consonants
    pub special: Vec<(&'static str, &'static str)>,
}

/// Returns a structured system of Bengali consonants
pub fn consonant_system() -> ConsonantSystem {
    ConsonantSystem {
        // Velars (ka-varga)
        velars: vec![
            ("k", "ক"),    // ka
            ("kh", "খ"),   // kha
            ("g", "গ"),    // ga
            ("gh", "ঘ"),   // gha
            ("Ng", "ঙ"),   // nga
        ],
        
        // Palatals (ca-varga)
        palatals: vec![
            ("c", "চ"),    // ca
            ("ch", "ছ"),   // cha
            ("J", "জ"),    // ja
            ("j", "জ"),    // ja
            ("jh", "ঝ"),   // jha
            ("NG", "ঞ"),    // nya (palatized)
        ],
        
        // Retroflex (ṭa-varga)
        retroflexes: vec![
            ("T", "ট"),    // Ta
            ("Th", "ঠ"),   // Tha
            ("D", "ড"),    // Da
            ("Dh", "ঢ"),   // Dha
            ("N", "ণ"),    // Na
        ],
        
        // Dentals (ta-varga)
        dentals: vec![
            ("t", "ত"),    // ta
            ("th", "থ"),   // tha
            ("d", "দ"),    // da
            ("dh", "ধ"),   // dha
            ("n", "ন"),    // na (non-palatized)
        ],
        
        // Labials (pa-varga)
        labials: vec![
            ("p", "প"),    // pa
            ("ph", "ফ"),   // pha
            ("f", "ফ"),    // alternative for pha
            ("b", "ব"),    // ba
            ("bh", "ভ"),   // bha
            ("v", "ভ"),    // alternative for bha
            ("m", "ম"),    // ma
        ],
        
        // Semivowels and liquids
        semivowels: vec![
            ("z", "য"),    // yô
            ("r", "র"),    // rô
            ("l", "ল"),    // lô
        ],
        
        // Fricatives
        fricatives: vec![
            ("sh", "শ"),   // palatal śô
            ("S", "শ"),   // palatal śô
            ("Sh", "ষ"),    // retroflex ṣô
            ("s", "স"),    // dental sô
            ("h", "হ"),    // hô
        ],
        
        // Special/modified sounds
        special: vec![
            ("R", "ড়"),    // ṛô
            ("Rh", "ঢ়"),   // ṛhô
            ("y", "য়"),    // antastô yô
            ("Y", "য়"),    // antastô yô
        ],
    }
}

/// Returns a flattened map of all Bengali consonants
pub fn consonants() -> HashMap<&'static str, &'static str> {
    let system = consonant_system();
    let mut map = HashMap::new();
    
    // Add all consonants from each category
    for (roman, bengali) in system.velars.iter()
        .chain(system.palatals.iter())
        .chain(system.retroflexes.iter())
        .chain(system.dentals.iter())
        .chain(system.labials.iter())
        .chain(system.semivowels.iter())
        .chain(system.fricatives.iter())
        .chain(system.special.iter()) {
            map.insert(*roman, *bengali);
        }
    
    map
} 