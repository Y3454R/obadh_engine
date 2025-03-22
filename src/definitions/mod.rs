//! Definitions for the Obadh Engine
//!
//! This module contains all character definitions and mappings used in the transliteration process,
//! organized by linguistic categories.

pub mod consonants;
pub mod vowels;
pub mod diacritics;
pub mod symbols;
pub mod modifiers;
pub mod numerals;

// Re-export commonly used functions
pub use consonants::{consonants, consonant_system, ConsonantSystem};
pub use vowels::{vowels, independent_vowels, vowel_modifiers, BengaliVowel};
pub use diacritics::diacritics;
pub use symbols::symbols;
pub use numerals::numerals;
pub use modifiers::special_rules; 