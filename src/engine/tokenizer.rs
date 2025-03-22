//! Tokenizer for the Obadh Engine
//!
//! This module provides functionality to tokenize input text into words
//! and letters/phonemes for processing by the transliteration engine.

use std::collections::HashMap;
use crate::definitions::{
    consonants, vowels, diacritics, special_rules
};

/// Types of tokens that can be identified
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    /// A standard word token
    Word,
    /// A punctuation mark
    Punctuation,
    /// A whitespace token
    Whitespace,
    /// A numeric token
    Number,
    /// A special symbol
    Symbol,
}

/// A token from the input text
#[derive(Debug, Clone)]
pub struct Token {
    /// The content of the token
    pub content: String,
    /// The type of the token
    pub token_type: TokenType,
    /// The position of the token in the original text
    pub position: usize,
}

/// Represents a sequence of phonetic components that make up a word
#[derive(Debug, Clone)]
pub struct PhoneticUnit {
    /// The original text
    pub text: String,
    /// What type of phonetic unit this is
    pub unit_type: PhoneticUnitType,
    /// Position in the original word
    pub position: usize,
}

/// Types of phonetic units in Bengali transliteration
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PhoneticUnitType {
    /// Single consonant
    Consonant,
    /// Vowel
    Vowel,
    /// A terminating vowel like 'o' that completes syllables
    TerminatingVowel,
    /// A consonant with a vowel modifier
    ConsonantWithVowel,
    /// A consonant with a terminating vowel
    ConsonantWithTerminator,
    /// A consonant followed by hasant
    ConsonantWithHasant,
    /// A conjunct (multiple consonants joined with hasant)
    Conjunct,
    /// A conjunct with a vowel modifier
    ConjunctWithVowel,
    /// A conjunct with a terminating vowel
    ConjunctWithTerminator,
    /// A special form (e.g., reph, ya-phala, etc.)
    SpecialForm,
    /// A numeral
    Numeral,
    /// A symbol or punctuation
    Symbol,
    /// Unknown unit
    Unknown,
}

/// Tokenizer for processing input text
pub struct Tokenizer {
    /// Map of special sequences to recognize
    special_sequences: HashMap<String, PhoneticUnitType>,
    /// Map of vowel patterns 
    vowel_patterns: HashMap<String, bool>,
    /// Map of consonant patterns
    consonant_patterns: HashMap<String, bool>,
}

impl Tokenizer {
    /// Create a new tokenizer with default configuration
    pub fn new() -> Self {
        let mut special_sequences = HashMap::new();
        let mut vowel_patterns = HashMap::new();
        let mut consonant_patterns = HashMap::new();
        
        // Get vowel patterns from the definitions
        let vowels_map = vowels();
        for roman in vowels_map.keys() {
            // Mark only 'o' as a terminating vowel
            if *roman == "o" {
                continue; // Skip adding to vowel_patterns, will add as terminator
            }
            vowel_patterns.insert(roman.to_string(), true);
        }
        
        // Add terminating vowel 'o' separately
        if vowels_map.contains_key("o") {
            special_sequences.insert("o".to_string(), PhoneticUnitType::TerminatingVowel);
        }
        
        // Get consonant patterns from the definitions
        let consonants_map = consonants();
        for roman in consonants_map.keys() {
            consonant_patterns.insert(roman.to_string(), true);
        }
        
        // Initialize special sequences
        // Reph (র্) is a special form
        special_sequences.insert("rr".to_string(), PhoneticUnitType::SpecialForm);
        
        // Hasant/Virama from diacritics
        let diacritics_map = diacritics();
        if let Some(hasant_key) = diacritics_map.iter().find_map(|(k, v)| {
            if *v == "্" { Some(k) } else { None }
        }) {
            special_sequences.insert(hasant_key.to_string(), PhoneticUnitType::ConsonantWithHasant);
        } else {
            // Fallback if not found
            special_sequences.insert(",,".to_string(), PhoneticUnitType::ConsonantWithHasant);
        }
        
        // Add special rules as appropriate
        let special_rules_map = special_rules();
        for roman in special_rules_map.keys() {
            special_sequences.insert(roman.to_string(), PhoneticUnitType::SpecialForm);
        }
        
        Tokenizer {
            special_sequences,
            vowel_patterns,
            consonant_patterns,
        }
    }
    
    /// Tokenize input text into words and other tokens
    pub fn tokenize_text(&self, text: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut current_word = String::new();
        let mut current_position = 0;
        
        // Helper function to add the current word as a token
        let add_current_word = |word: &mut String, pos: usize, tokens: &mut Vec<Token>| {
            if !word.is_empty() {
                // Determine if the word is a number
                let token_type = if word.chars().all(|c| c.is_numeric()) {
                    TokenType::Number
                } else {
                    TokenType::Word
                };
                
                tokens.push(Token {
                    content: word.clone(),
                    token_type,
                    position: pos,
                });
                word.clear();
            }
        };
        
        let mut i = 0;
        while i < text.len() {
            // Get the current character
            let c = text[i..].chars().next().unwrap();
            let char_len = c.len_utf8();
            
            // Special case: Check for hasanta sequence (,,)
            if c == ',' && i + 1 < text.len() && text.chars().nth(i + 1) == Some(',') {
                // If we're in a word context and there's a consonant before this
                if !current_word.is_empty() {
                    // Add the sequence to the current word
                    current_word.push_str(",,");
                    i += 2; // Skip both commas
                    continue;
                } else {
                    // If we're not in a word context, handle as regular punctuation
                    add_current_word(&mut current_word, current_position, &mut tokens);
                    
                    // Add the first comma as punctuation
                    tokens.push(Token {
                        content: ",".to_string(),
                        token_type: TokenType::Punctuation,
                        position: i,
                    });
                    
                    i += 1; // Move to the next comma
                    current_position = i;
                    continue;
                }
            }
            
            if c.is_whitespace() {
                // Add the current word if any
                add_current_word(&mut current_word, current_position, &mut tokens);
                
                // Add the whitespace as a token
                tokens.push(Token {
                    content: c.to_string(),
                    token_type: TokenType::Whitespace,
                    position: i,
                });
                
                current_position = i + char_len;
            } else if c.is_ascii_punctuation() {
                // Add the current word if any
                add_current_word(&mut current_word, current_position, &mut tokens);
                
                // Add the punctuation as a token
                tokens.push(Token {
                    content: c.to_string(),
                    token_type: TokenType::Punctuation,
                    position: i,
                });
                
                current_position = i + char_len;
            } else if !c.is_alphanumeric() && !current_word.is_empty() {
                // Special symbol - add the current word if any
                add_current_word(&mut current_word, current_position, &mut tokens);
                
                // Add the symbol as a token
                tokens.push(Token {
                    content: c.to_string(),
                    token_type: TokenType::Symbol,
                    position: i,
                });
                
                current_position = i + char_len;
            } else {
                // If we have an empty current word, update the position
                if current_word.is_empty() {
                    current_position = i;
                }
                // Add the character to the current word
                current_word.push(c);
            }
            
            i += char_len;
        }
        
        // Add any remaining word
        add_current_word(&mut current_word, current_position, &mut tokens);
        
        tokens
    }
    
    /// Tokenize a word into phonetic units for Bengali transliteration
    pub fn tokenize_word(&self, word: &str) -> Vec<PhoneticUnit> {
        let mut units = Vec::new();
        let mut i = 0;
        
        while i < word.len() {
            // Try to match special sequences first
            let mut matched = false;
            for (sequence, unit_type) in &self.special_sequences {
                if i + sequence.len() <= word.len() && &word[i..i+sequence.len()] == sequence {
                    units.push(PhoneticUnit {
                        text: sequence.clone(),
                        unit_type: unit_type.clone(),
                        position: i,
                    });
                    i += sequence.len();
                    matched = true;
                    break;
                }
            }
            
            if matched {
                continue;
            }
            
            // Try to match consonant patterns (longer patterns first)
            let mut matched_consonant = false;
            let mut consonant_patterns: Vec<_> = self.consonant_patterns.keys().collect();
            consonant_patterns.sort_by(|a, b| b.len().cmp(&a.len())); // Sort by length, descending
            
            for pattern in consonant_patterns {
                if i + pattern.len() <= word.len() && &word[i..i+pattern.len()] == pattern {
                    units.push(PhoneticUnit {
                        text: pattern.clone(),
                        unit_type: PhoneticUnitType::Consonant,
                        position: i,
                    });
                    i += pattern.len();
                    matched_consonant = true;
                    break;
                }
            }
            
            if matched_consonant {
                continue;
            }
            
            // Try to match vowel patterns (longer patterns first)
            let mut matched_vowel = false;
            let mut vowel_patterns: Vec<_> = self.vowel_patterns.keys().collect();
            vowel_patterns.sort_by(|a, b| b.len().cmp(&a.len())); // Sort by length, descending
            
            for pattern in vowel_patterns {
                if i + pattern.len() <= word.len() && &word[i..i+pattern.len()] == pattern {
                    units.push(PhoneticUnit {
                        text: pattern.clone(),
                        unit_type: PhoneticUnitType::Vowel,
                        position: i,
                    });
                    i += pattern.len();
                    matched_vowel = true;
                    break;
                }
            }
            
            if matched_vowel {
                continue;
            }
            
            // If no pattern matched, treat as unknown and advance by one character
            if i < word.len() {
                // Find the length of one UTF-8 character
                let char_len = word[i..].chars().next().map_or(1, |c| c.len_utf8());
                
                units.push(PhoneticUnit {
                    text: word[i..i+char_len].to_string(),
                    unit_type: PhoneticUnitType::Unknown,
                    position: i,
                });
                i += char_len;
            }
        }
        
        // Post-processing to identify conjuncts and other complex forms
        self.identify_complex_forms(&mut units);
        
        units
    }
    
    /// Identify complex phonetic forms like conjuncts and consonants with vowel modifiers
    fn identify_complex_forms(&self, units: &mut Vec<PhoneticUnit>) {
        let mut i = 0;
        while i < units.len() {
            // First pass: Form basic units and conjuncts

            // Identify consonant + hasant (,,) + consonant as an explicit conjunct
            if i + 2 < units.len() && 
               units[i].unit_type == PhoneticUnitType::Consonant &&
               units[i+1].unit_type == PhoneticUnitType::ConsonantWithHasant &&
               units[i+2].unit_type == PhoneticUnitType::Consonant {
                
                let conjunct_text = format!("{}{}{}", 
                    units[i].text, units[i+1].text, units[i+2].text);
                
                let position = units[i].position;
                
                // Replace the three units with a single conjunct unit
                units[i] = PhoneticUnit {
                    text: conjunct_text,
                    unit_type: PhoneticUnitType::Conjunct,
                    position,
                };
                
                // Remove the next two units
                units.remove(i+1);
                units.remove(i+1);
                
                // Don't increment i since we want to check if the new unit
                // is part of a larger complex form
                continue;
            }
            
            // Form conjuncts from consecutive consonants (without explicit hasant)
            if i + 1 < units.len() && 
               units[i].unit_type == PhoneticUnitType::Consonant &&
               units[i+1].unit_type == PhoneticUnitType::Consonant {
                
                // Form an implicit conjunct by adding virtual hasant
                let conjunct_text = format!("{}{}{}", units[i].text, ",,", units[i+1].text);
                let position = units[i].position;
                
                // Replace with a single conjunct unit
                units[i] = PhoneticUnit {
                    text: conjunct_text,
                    unit_type: PhoneticUnitType::Conjunct,
                    position,
                };
                
                // Remove the second consonant unit
                units.remove(i+1);
                
                // Don't increment i since we want to check if the new conjunct
                // can form part of a larger form
                continue;
            }
            
            // Form conjunct with vowel: consonant + consonantWithVowel
            if i + 1 < units.len() && 
               units[i].unit_type == PhoneticUnitType::Consonant &&
               (units[i+1].unit_type == PhoneticUnitType::ConsonantWithVowel ||
                units[i+1].unit_type == PhoneticUnitType::ConsonantWithTerminator) {
                
                // Separate the consonant and vowel parts from the second unit
                let cons2 = &units[i+1].text[0..1]; // First character is the consonant
                let vowel_part = &units[i+1].text[1..]; // Rest is the vowel
                
                // Form conjunct with vowel
                let conjunct_text = format!("{}{}{}{}", units[i].text, ",,", cons2, vowel_part);
                let position = units[i].position;
                
                // Choose the right unit type based on whether it has a terminator vowel or regular vowel
                let unit_type = if units[i+1].unit_type == PhoneticUnitType::ConsonantWithTerminator {
                    PhoneticUnitType::ConjunctWithTerminator
                } else {
                    PhoneticUnitType::ConjunctWithVowel
                };
                
                // Replace with a single conjunct with vowel unit
                units[i] = PhoneticUnit {
                    text: conjunct_text,
                    unit_type,
                    position,
                };
                
                // Remove the second unit
                units.remove(i+1);
                
                continue;
            }
            
            // Identify consonant + terminating vowel as a consonant with terminator
            if i + 1 < units.len() && 
               units[i].unit_type == PhoneticUnitType::Consonant &&
               units[i+1].unit_type == PhoneticUnitType::TerminatingVowel {
                
                let combined_text = format!("{}{}", units[i].text, units[i+1].text);
                let position = units[i].position;
                
                // Replace the two units with a single consonant+terminator unit
                units[i] = PhoneticUnit {
                    text: combined_text,
                    unit_type: PhoneticUnitType::ConsonantWithTerminator,
                    position,
                };
                
                // Remove the vowel unit
                units.remove(i+1);
                
                // Don't increment i since we want to check if the new unit
                // is part of a larger complex form
                continue;
            }
            
            // Identify consonant + vowel as a consonant with vowel modifier
            if i + 1 < units.len() && 
               units[i].unit_type == PhoneticUnitType::Consonant &&
               units[i+1].unit_type == PhoneticUnitType::Vowel {
                
                let combined_text = format!("{}{}", units[i].text, units[i+1].text);
                let position = units[i].position;
                
                // Replace the two units with a single consonant+vowel unit
                units[i] = PhoneticUnit {
                    text: combined_text,
                    unit_type: PhoneticUnitType::ConsonantWithVowel,
                    position,
                };
                
                // Remove the vowel unit
                units.remove(i+1);
                
                // Don't increment i since we want to check if the new unit
                // is part of a larger complex form
                continue;
            }
            
            i += 1;
        }
        
        // Second pass: Handle vowels with conjuncts
        i = 0;
        while i < units.len() - 1 {
            // Conjunct + Vowel -> ConjunctWithVowel
            if units[i].unit_type == PhoneticUnitType::Conjunct && 
               units[i+1].unit_type == PhoneticUnitType::Vowel {
                
                let combined_text = format!("{}{}", units[i].text, units[i+1].text);
                let position = units[i].position;
                
                // Replace with a single conjunct with vowel unit
                units[i] = PhoneticUnit {
                    text: combined_text,
                    unit_type: PhoneticUnitType::ConjunctWithVowel,
                    position,
                };
                
                // Remove the vowel unit
                units.remove(i+1);
                continue;
            }
            
            // Conjunct + TerminatingVowel -> ConjunctWithTerminator
            if units[i].unit_type == PhoneticUnitType::Conjunct && 
               units[i+1].unit_type == PhoneticUnitType::TerminatingVowel {
                
                let combined_text = format!("{}{}", units[i].text, units[i+1].text);
                let position = units[i].position;
                
                // Replace with a single conjunct with terminator unit
                units[i] = PhoneticUnit {
                    text: combined_text,
                    unit_type: PhoneticUnitType::ConjunctWithTerminator,
                    position,
                };
                
                // Remove the vowel unit
                units.remove(i+1);
                continue;
            }
            
            i += 1;
        }
    }
}

impl Default for Tokenizer {
    fn default() -> Self {
        Self::new()
    }
} 