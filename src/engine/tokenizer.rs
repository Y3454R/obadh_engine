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
    /// A reph (র্) over a consonant
    RephOverConsonant,
    /// A reph over a consonant with a vowel
    RephOverConsonantWithVowel,
    /// A reph over a consonant with a terminator
    RephOverConsonantWithTerminator,
    /// Chandrabindu (ঁ) with a vowel
    ChandrabinduWithVowel,
    /// Chandrabindu (ঁ) with a consonant 
    ChandrabinduWithConsonant,
    /// Chandrabindu (ঁ) with a consonant and vowel
    ChandrabinduWithConsonantAndVowel,
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
        
        // Add Chandrabindu (^), Visarga (:), and Khanda Ta (T``)
        special_sequences.insert("^".to_string(), PhoneticUnitType::SpecialForm);
        special_sequences.insert(":".to_string(), PhoneticUnitType::SpecialForm);
        special_sequences.insert("T``".to_string(), PhoneticUnitType::SpecialForm);
        
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
            
            // Special case: Check for diacritics that should attach to the previous word
            if !current_word.is_empty() && (c == '^' || c == ':' || c == '`') {
                // Special case for Khanda Ta (T``)
                if c == '`' && i + 1 < text.len() && text.chars().nth(i + 1) == Some('`') {
                    if current_word.ends_with('T') {
                        // Add the `` to mark it as Khanda Ta
                        current_word.push_str("``");
                        i += 2; // Skip both backticks
                        continue;
                    }
                }
                
                // Handle ^ (Chandrabindu) and : (Visarga) as part of the word
                if c == '^' || c == ':' {
                    current_word.push(c);
                    i += char_len;
                    continue;
                }
            }
            
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
        
        // Process the word character by character
        let mut _i = 0;
        
        // Pre-process special sequences
        let mut processed_word = word.to_string();
        
        // Check for chandrabindu (^) and visarga (:) at the end
        let has_chandrabindu = processed_word.ends_with('^');
        let has_visarga = processed_word.ends_with(':');
        
        // Remove the diacritics for processing
        if has_chandrabindu {
            processed_word.pop();  // Remove the chandrabindu
        } else if has_visarga {
            processed_word.pop();  // Remove the visarga
        }
        
        // Special case for standalone diacritics
        if processed_word.is_empty() && (has_chandrabindu || has_visarga) {
            // Handle standalone diacritics directly
            if has_chandrabindu {
                units.push(PhoneticUnit {
                    text: "^".to_string(),
                    unit_type: PhoneticUnitType::SpecialForm,
                    position: 0,
                });
            } else if has_visarga {
                units.push(PhoneticUnit {
                    text: ":".to_string(),
                    unit_type: PhoneticUnitType::SpecialForm,
                    position: 0,
                });
            }
            return units;
        }
        
        // Process the base word without diacritics
        while _i < processed_word.len() {
            // Try to match special sequences first
            let mut matched = false;
            
            // Try to match "ng" specifically before other sequences
            if _i + 2 <= processed_word.len() && &processed_word[_i.._i+2] == "ng" {
                units.push(PhoneticUnit {
                    text: "ng".to_string(),
                    unit_type: PhoneticUnitType::SpecialForm,
                    position: _i,
                });
                _i += 2;
                continue;
            }
            
            for (sequence, unit_type) in &self.special_sequences {
                if _i + sequence.len() <= processed_word.len() && &processed_word[_i.._i+sequence.len()] == sequence {
                    // Ensure all special forms are treated as SpecialForm, even T``
                    let final_unit_type = if sequence == "T``" {
                        PhoneticUnitType::SpecialForm
                    } else {
                        unit_type.clone()
                    };
                    
                    units.push(PhoneticUnit {
                        text: sequence.clone(),
                        unit_type: final_unit_type,
                        position: _i,
                    });
                    _i += sequence.len();
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
                if _i + pattern.len() <= processed_word.len() && &processed_word[_i.._i+pattern.len()] == pattern {
                    units.push(PhoneticUnit {
                        text: pattern.clone(),
                        unit_type: PhoneticUnitType::Consonant,
                        position: _i,
                    });
                    _i += pattern.len();
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
                if _i + pattern.len() <= processed_word.len() && &processed_word[_i.._i+pattern.len()] == pattern {
                    units.push(PhoneticUnit {
                        text: pattern.clone(),
                        unit_type: PhoneticUnitType::Vowel,
                        position: _i,
                    });
                    _i += pattern.len();
                    matched_vowel = true;
                    break;
                }
            }
            
            if matched_vowel {
                    continue;
                }
            
            // If no pattern matched, treat as unknown and advance by one character
            if _i < processed_word.len() {
                // Find the length of one UTF-8 character
                let char_len = processed_word[_i..].chars().next().map_or(1, |c| c.len_utf8());
                
                units.push(PhoneticUnit {
                    text: processed_word[_i.._i+char_len].to_string(),
                    unit_type: PhoneticUnitType::Unknown,
                    position: _i,
                });
                _i += char_len;
            }
        }
        
        // Post-processing to identify conjuncts and other complex forms
        self.identify_complex_forms(&mut units);
        
        // Reapply the diacritics if present
        if !units.is_empty() {
            if has_chandrabindu {
                let last_unit = units.last_mut().unwrap();
                // Update the text and apply chandrabindu type based on the current unit type
                last_unit.text = format!("{}^", last_unit.text);
                
                // Apply appropriate chandrabindu type based on the base unit
                match last_unit.unit_type {
                    PhoneticUnitType::Consonant => {
                        last_unit.unit_type = PhoneticUnitType::ChandrabinduWithConsonant;
                    },
                    PhoneticUnitType::Vowel => {
                        last_unit.unit_type = PhoneticUnitType::ChandrabinduWithVowel;
                    },
                    PhoneticUnitType::ConsonantWithVowel => {
                        last_unit.unit_type = PhoneticUnitType::ChandrabinduWithConsonantAndVowel;
                    },
                    PhoneticUnitType::ConsonantWithTerminator => {
                        last_unit.unit_type = PhoneticUnitType::ChandrabinduWithConsonantAndVowel;
                    },
                    PhoneticUnitType::Conjunct => {
                        last_unit.unit_type = PhoneticUnitType::ChandrabinduWithConsonant;
                    },
                    PhoneticUnitType::ConjunctWithVowel => {
                        last_unit.unit_type = PhoneticUnitType::ChandrabinduWithConsonantAndVowel;
                    },
                    PhoneticUnitType::ConjunctWithTerminator => {
                        last_unit.unit_type = PhoneticUnitType::ChandrabinduWithConsonantAndVowel;
                    },
                    _ => {
                        // If it doesn't fit any of the above, just keep the original type
                    }
                }
            } else if has_visarga {
                // For visarga, we now add it as a separate unit instead of combining
                // Get the position for the new visarga unit
                let position = {
                    let last = units.last().unwrap();
                    last.position + last.text.len()
                };
                
                // Now add the visarga as a separate unit
                units.push(PhoneticUnit {
                    text: ":".to_string(),
                    unit_type: PhoneticUnitType::SpecialForm,
                    position,
                });
            }
        }
        
        units
    }
    
    /// Identify complex phonetic forms like conjuncts and consonants with vowel modifiers
    fn identify_complex_forms(&self, units: &mut Vec<PhoneticUnit>) {
        let mut _i = 0;
        
        // First pass: Handle special "rr" cases
        // - "rri" as vocalic R vowel
        // - "rr" + consonant as reph
        _i = 0;
        while _i < units.len() {
            // Handle "rr" + "i" as vocalic R vowel
            if _i + 1 < units.len() && 
               units[_i].text == "rr" && 
               units[_i].unit_type == PhoneticUnitType::SpecialForm && 
               units[_i+1].text == "i" && 
               units[_i+1].unit_type == PhoneticUnitType::Vowel {
                
                let vowel_text = format!("rri");
                let _position = units[_i].position;
                
                // Replace with a single vowel unit
                units[_i] = PhoneticUnit {
                    text: vowel_text,
                    unit_type: PhoneticUnitType::Vowel, // Vocalic R is a vowel
                    position: _position,
                };
                
                // Remove the "i" unit
                units.remove(_i+1);
                continue;
            }
            
            // Handle "rr" + consonant as reph over consonant
            if _i + 1 < units.len() && 
               units[_i].text == "rr" && 
               units[_i].unit_type == PhoneticUnitType::SpecialForm && 
               units[_i+1].unit_type == PhoneticUnitType::Consonant {
                
                let reph_text = format!("rr{}", units[_i+1].text);
                let _position = units[_i].position;
                
                // Replace with a reph over consonant unit
                units[_i] = PhoneticUnit {
                    text: reph_text,
                    unit_type: PhoneticUnitType::RephOverConsonant,
                    position: _position,
                };
                
                // Remove the consonant unit
                units.remove(_i+1);
                continue;
            }
            
            // Handle "rr" + consonantWithVowel as reph over consonant with vowel
            if _i + 1 < units.len() && 
               units[_i].text == "rr" && 
               units[_i].unit_type == PhoneticUnitType::SpecialForm && 
               units[_i+1].unit_type == PhoneticUnitType::ConsonantWithVowel {
                
                let reph_text = format!("rr{}", units[_i+1].text);
                let _position = units[_i].position;
                
                // Replace with a reph over consonant with vowel unit
                units[_i] = PhoneticUnit {
                    text: reph_text,
                    unit_type: PhoneticUnitType::RephOverConsonantWithVowel,
                    position: _position,
                };
                
                // Remove the consonant with vowel unit
                units.remove(_i+1);
                continue;
            }
            
            // Handle "rr" + consonantWithTerminator as reph over consonant with terminator
            if _i + 1 < units.len() && 
               units[_i].text == "rr" && 
               units[_i].unit_type == PhoneticUnitType::SpecialForm && 
               units[_i+1].unit_type == PhoneticUnitType::ConsonantWithTerminator {
                
                let reph_text = format!("rr{}", units[_i+1].text);
                let _position = units[_i].position;
                
                // Replace with a reph over consonant with terminator unit
                units[_i] = PhoneticUnit {
                    text: reph_text,
                    unit_type: PhoneticUnitType::RephOverConsonantWithTerminator,
                    position: _position,
                };
                
                // Remove the consonant with terminator unit
                units.remove(_i+1);
                continue;
            }
            
                _i += 1;
        }
        
        // Second pass: Form basic units and conjuncts
        _i = 0;
        while _i < units.len() {
            // Identify consonant + hasant (,,) + consonant as an explicit conjunct
            if _i + 2 < units.len() && 
               units[_i].unit_type == PhoneticUnitType::Consonant &&
               units[_i+1].unit_type == PhoneticUnitType::ConsonantWithHasant &&
               units[_i+2].unit_type == PhoneticUnitType::Consonant {
                
                let conjunct_text = format!("{}{}{}", 
                    units[_i].text, units[_i+1].text, units[_i+2].text);
                
                let _position = units[_i].position;
                
                // Replace the three units with a single conjunct unit
                units[_i] = PhoneticUnit {
                    text: conjunct_text,
                    unit_type: PhoneticUnitType::Conjunct,
                    position: _position,
                };
                
                // Remove the next two units
                units.remove(_i+1);
                units.remove(_i+1);
                
                // Don't increment _i since we want to check if the new unit
                // is part of a larger complex form
                continue;
            }
            
            // Handle consonant + vocalic R vowel as consonant with vowel
            if _i + 1 < units.len() && 
               units[_i].unit_type == PhoneticUnitType::Consonant &&
               units[_i+1].text == "rri" && 
               units[_i+1].unit_type == PhoneticUnitType::Vowel {
                
                let combined_text = format!("{}{}", units[_i].text, units[_i+1].text);
                let _position = units[_i].position;
                
                // Replace with a single consonant with vowel unit
                units[_i] = PhoneticUnit {
                    text: combined_text,
                    unit_type: PhoneticUnitType::ConsonantWithVowel,
                    position: _position,
                };
                
                // Remove the vowel unit
                units.remove(_i+1);
                continue;
            }
            
            // Identify consonant + vowel as a consonant with vowel unit (general case)
            if _i + 1 < units.len() && 
               units[_i].unit_type == PhoneticUnitType::Consonant &&
               units[_i+1].unit_type == PhoneticUnitType::Vowel {
                
                let combined_text = format!("{}{}", units[_i].text, units[_i+1].text);
                let _position = units[_i].position;
                
                // Replace with a single consonant with vowel unit
                units[_i] = PhoneticUnit {
                    text: combined_text,
                    unit_type: PhoneticUnitType::ConsonantWithVowel,
                    position: _position,
                };
                
                // Remove the vowel unit
                units.remove(_i+1);
                
                // Don't increment _i since we want to check if the new unit
                // is part of a larger complex form
                continue;
            }
            
            // Form conjuncts from consecutive consonants (without explicit hasant)
            if _i + 1 < units.len() && 
               units[_i].unit_type == PhoneticUnitType::Consonant &&
               units[_i+1].unit_type == PhoneticUnitType::Consonant {
                
                // Form an implicit conjunct by adding virtual hasant
                let conjunct_text = format!("{}{}{}", units[_i].text, ",,", units[_i+1].text);
                let _position = units[_i].position;
                
                // Replace with a single conjunct unit
                units[_i] = PhoneticUnit {
                    text: conjunct_text,
                    unit_type: PhoneticUnitType::Conjunct,
                    position: _position,
                };
                
                // Remove the second consonant unit
                units.remove(_i+1);
                
                // Don't increment _i since we want to check if the new conjunct
                // can form part of a larger form
                continue;
            }
            
            // Form conjunct with vowel: consonant + consonantWithVowel
            if _i + 1 < units.len() && 
               units[_i].unit_type == PhoneticUnitType::Consonant &&
               (units[_i+1].unit_type == PhoneticUnitType::ConsonantWithVowel ||
                units[_i+1].unit_type == PhoneticUnitType::ConsonantWithTerminator) {
                
                // Separate the consonant and vowel parts from the second unit
                let cons2 = &units[_i+1].text[0..1]; // First character is the consonant
                let vowel_part = &units[_i+1].text[1..]; // Rest is the vowel
                
                // Form conjunct with vowel
                let conjunct_text = format!("{}{}{}{}", units[_i].text, ",,", cons2, vowel_part);
                let _position = units[_i].position;
                
                // Choose the right unit type based on whether it has a terminator vowel or regular vowel
                let unit_type = if units[_i+1].unit_type == PhoneticUnitType::ConsonantWithTerminator {
                    PhoneticUnitType::ConjunctWithTerminator
            } else {
                    PhoneticUnitType::ConjunctWithVowel
                };
                
                // Replace with a single conjunct with vowel unit
                units[_i] = PhoneticUnit {
                    text: conjunct_text,
                    unit_type,
                    position: _position,
                };
                
                // Remove the second unit
                units.remove(_i+1);
                
                continue;
            }
            
            // Identify consonant + terminating vowel as a consonant with terminator
            if _i + 1 < units.len() && 
               units[_i].unit_type == PhoneticUnitType::Consonant &&
               units[_i+1].unit_type == PhoneticUnitType::TerminatingVowel {
                
                let combined_text = format!("{}{}", units[_i].text, units[_i+1].text);
                let _position = units[_i].position;
                
                // Replace with a single consonant with terminator unit
                units[_i] = PhoneticUnit {
                    text: combined_text,
                    unit_type: PhoneticUnitType::ConsonantWithTerminator,
                    position: _position,
                };
                
                // Remove the vowel unit
                units.remove(_i+1);
                
                // Don't increment _i since we want to check if the new unit
                // is part of a larger complex form
                continue;
            }
            
            // Handle Chandrabindu (^) applying to the previous unit - ONLY THIS GETS SPECIAL TREATMENT
            if _i > 0 && _i < units.len() && 
               units[_i].text == "^" && 
               units[_i].unit_type == PhoneticUnitType::SpecialForm {
                
                let _position = units[_i-1].position;
                let combined_text = format!("{}{}", units[_i-1].text, units[_i].text);
                
                // Determine the new unit type based on what precedes the chandrabindu
                let new_unit_type = match units[_i-1].unit_type {
                    PhoneticUnitType::Consonant => PhoneticUnitType::ChandrabinduWithConsonant,
                    PhoneticUnitType::Vowel => PhoneticUnitType::ChandrabinduWithVowel,
                    PhoneticUnitType::ConsonantWithVowel => PhoneticUnitType::ChandrabinduWithConsonantAndVowel,
                    PhoneticUnitType::ConsonantWithTerminator => PhoneticUnitType::ChandrabinduWithConsonantAndVowel,
                    PhoneticUnitType::Conjunct => PhoneticUnitType::ChandrabinduWithConsonant,
                    PhoneticUnitType::ConjunctWithVowel => PhoneticUnitType::ChandrabinduWithConsonantAndVowel,
                    PhoneticUnitType::ConjunctWithTerminator => PhoneticUnitType::ChandrabinduWithConsonantAndVowel,
                    _ => units[_i-1].unit_type.clone(),
                };
                
                // Update the previous unit to include the chandrabindu
                units[_i-1].text = combined_text;
                units[_i-1].unit_type = new_unit_type;
                
                // Remove the chandrabindu unit
                units.remove(_i);
                
                // Decrement _i to check the new combined unit against further combinations
                _i -= 1;
                continue;
            }
            
            // For Visarga (:), "ng", "T``", and other diacritics - treat as separate units
            if (units[_i].text == ":" || units[_i].text == "ng" || units[_i].text == "T``") && 
               units[_i].unit_type == PhoneticUnitType::SpecialForm {
                // Keep as separate units - do nothing special
                _i += 1;
                continue;
            }
            
            _i += 1;
        }
        
        // Third pass: Handle vowels with conjuncts and reph
        _i = 0;
        while _i < units.len() - 1 {
            // Conjunct + Vowel -> ConjunctWithVowel
            if units[_i].unit_type == PhoneticUnitType::Conjunct && 
               units[_i+1].unit_type == PhoneticUnitType::Vowel {
                
                let combined_text = format!("{}{}", units[_i].text, units[_i+1].text);
                let _position = units[_i].position;
                
                // Replace with a single conjunct with vowel unit
                units[_i] = PhoneticUnit {
                    text: combined_text,
                    unit_type: PhoneticUnitType::ConjunctWithVowel,
                    position: _position,
                };
                
                // Remove the vowel unit
                units.remove(_i+1);
                continue;
            }
            
            // Conjunct + TerminatingVowel -> ConjunctWithTerminator
            if units[_i].unit_type == PhoneticUnitType::Conjunct && 
               units[_i+1].unit_type == PhoneticUnitType::TerminatingVowel {
                
                let combined_text = format!("{}{}", units[_i].text, units[_i+1].text);
                let _position = units[_i].position;
                
                // Replace with a single conjunct with terminator unit
                units[_i] = PhoneticUnit {
                    text: combined_text,
                    unit_type: PhoneticUnitType::ConjunctWithTerminator,
                    position: _position,
                };
                
                // Remove the vowel unit
                units.remove(_i+1);
                continue;
            }
            
            // RephOverConsonant + Vowel -> RephOverConsonantWithVowel
            if units[_i].unit_type == PhoneticUnitType::RephOverConsonant && 
               units[_i+1].unit_type == PhoneticUnitType::Vowel {
                
                let combined_text = format!("{}{}", units[_i].text, units[_i+1].text);
                let _position = units[_i].position;
                
                // Replace with a single reph over consonant with vowel unit
                units[_i] = PhoneticUnit {
                    text: combined_text,
                    unit_type: PhoneticUnitType::RephOverConsonantWithVowel,
                    position: _position,
                };
                
                // Remove the vowel unit
                units.remove(_i+1);
                continue;
            }
            
            // RephOverConsonant + TerminatingVowel -> RephOverConsonantWithTerminator
            if units[_i].unit_type == PhoneticUnitType::RephOverConsonant && 
               units[_i+1].unit_type == PhoneticUnitType::TerminatingVowel {
                
                let combined_text = format!("{}{}", units[_i].text, units[_i+1].text);
                let _position = units[_i].position;
                
                // Replace with a single reph over consonant with terminator unit
                units[_i] = PhoneticUnit {
                    text: combined_text,
                    unit_type: PhoneticUnitType::RephOverConsonantWithTerminator,
                    position: _position,
                };
                
                // Remove the vowel unit
                units.remove(_i+1);
                continue;
            }
            
            _i += 1;
        }
        
        // Fourth pass: Handle diacritics and special forms
        _i = 0;
        while _i < units.len() {
            // For Chandrabindu (^) - if a unit text ends with ^, update its type
            if units[_i].text.ends_with('^') {
                // Determine the type based on the current unit
                match units[_i].unit_type {
                    PhoneticUnitType::Consonant => {
                        units[_i].unit_type = PhoneticUnitType::ChandrabinduWithConsonant;
                    },
                    PhoneticUnitType::Vowel => {
                        units[_i].unit_type = PhoneticUnitType::ChandrabinduWithVowel;
                    },
                    PhoneticUnitType::ConsonantWithVowel => {
                        units[_i].unit_type = PhoneticUnitType::ChandrabinduWithConsonantAndVowel;
                    },
                    PhoneticUnitType::ConsonantWithTerminator => {
                        units[_i].unit_type = PhoneticUnitType::ChandrabinduWithConsonantAndVowel;
                    },
                    PhoneticUnitType::Conjunct => {
                        units[_i].unit_type = PhoneticUnitType::ChandrabinduWithConsonant;
                    },
                    PhoneticUnitType::ConjunctWithVowel => {
                        units[_i].unit_type = PhoneticUnitType::ChandrabinduWithConsonantAndVowel;
                    },
                    PhoneticUnitType::ConjunctWithTerminator => {
                        units[_i].unit_type = PhoneticUnitType::ChandrabinduWithConsonantAndVowel;
                    },
                    _ => {
                        // Keep the original type but preserve the text with chandrabindu
                    }
                }
            }
            
            // For Visarga (:), "ng", "T``" - keep them as separate units
            
            _i += 1;
        }
    }
}

impl Default for Tokenizer {
    fn default() -> Self {
        Self::new()
    }
}