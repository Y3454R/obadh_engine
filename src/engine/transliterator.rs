//! Main transliteration engine for Roman to Bengali conversion.
//!
//! This module contains the core logic for transliterating Roman text to Bengali.
//! 
//! For detailed implementation rules, see docs/simplified_rules.md

use std::collections::HashMap;
use crate::definitions::{
    consonants, consonant_system, ConsonantSystem,
    vowels, BengaliVowel,
    diacritics, symbols, numerals, special_rules
};
use super::sanitizer::{Sanitizer, SanitizeResult};
use super::tokenizer::{Tokenizer, Token, TokenType, PhoneticUnit, PhoneticUnitType};

/// Main transliterator that performs the Roman to Bengali conversion
#[allow(dead_code)]  // Fields will be used when we implement the full transliteration
pub struct Transliterator {
    // Structured phonetic data
    consonant_system: ConsonantSystem,
    vowels: HashMap<&'static str, BengaliVowel>,
    
    // Lookup tables for conversion
    consonants: HashMap<&'static str, &'static str>,
    diacritics: HashMap<&'static str, &'static str>,
    symbols: HashMap<&'static str, &'static str>,
    numerals: HashMap<&'static str, &'static str>,
    special_rules: HashMap<&'static str, &'static str>,
    
    // Input sanitizer
    sanitizer: Sanitizer,
    
    // Tokenizer
    tokenizer: Tokenizer,
}

impl Transliterator {
    /// Create a new transliterator with default configuration
    pub fn new() -> Self {
        Transliterator {
            // Structured phonetic data
            consonant_system: consonant_system(),
            vowels: vowels(),
            
            // Lookup tables for conversion
            consonants: consonants(),
            diacritics: diacritics(),
            symbols: symbols(),
            numerals: numerals(),
            special_rules: special_rules(),
            
            // Input sanitizer
            sanitizer: Sanitizer::default(),
            
            // Tokenizer
            tokenizer: Tokenizer::default(),
        }
    }
    
    /// Create a conjunct by adding hasant between consonants
    #[allow(dead_code)]
    fn create_conjunct(&self, c1: &str, c2: &str) -> String {
        let hasant = self.diacritics.get(",,").unwrap_or(&"্");
        format!("{}{}{}", c1, hasant, c2)
    }
    
    /// Add a hasant to a consonant (used when explicitly adding hasant with ,,)
    #[allow(dead_code)]
    fn add_hasant(&self, consonant: &str) -> String {
        let hasant = self.diacritics.get(",,").unwrap_or(&"্");
        format!("{}{}", consonant, hasant)
    }
    
    /// Create a reph form when "rr" is followed by another consonant
    #[allow(dead_code)]
    fn create_reph(&self, consonant: &str) -> String {
        let hasant = self.diacritics.get(",,").unwrap_or(&"্");
        format!("র{}{}", hasant, consonant)
    }
    
    /// Create y-phola form (ya-phalā) by joining a consonant with য
    #[allow(dead_code)]
    fn create_ya_phala(&self, consonant: &str) -> String {
        // For y-phola, we join the consonant with য using hasant
        let hasant = self.diacritics.get(",,").unwrap_or(&"্");
        let ya = "য"; // Use regular yo (য) for jo-phola, NOT antastha ya (য়)
        format!("{}{}{}", consonant, hasant, ya)
    }
    
    /// Create w-phola form (ba-phalā) by joining a consonant with ব
    #[allow(dead_code)]
    fn create_ba_phala(&self, consonant: &str) -> String {
        // For w-phola, we join the consonant with ব using hasant
        let hasant = self.diacritics.get(",,").unwrap_or(&"্");
        let ba = self.consonants.get("b").unwrap_or(&"ব");
        format!("{}{}{}", consonant, hasant, ba)
    }
    
    /// Transliterate Roman text to Bengali
    pub fn transliterate(&self, text: &str) -> String {
        // First sanitize the input
        match self.sanitize(text) {
            Ok(sanitized) => {
                // Process the sanitized text using the tokenizer
                let tokens = self.tokenizer.tokenize_text(&sanitized);
                
                // Process each token based on its type
                let mut result = String::new();
                
                for token in tokens {
                    match token.token_type {
                        TokenType::Word => {
                            result.push_str(&self.transliterate_word(&token.content));
                        },
                        TokenType::Whitespace => {
                            result.push_str(&token.content);
                        },
                        TokenType::Punctuation => {
                            // For most punctuation, keep it as is
                            // However, some punctuation might need to be converted
                            if let Some(bengali_symbol) = self.symbols.get(token.content.as_str()) {
                                result.push_str(bengali_symbol);
                            } else {
                                result.push_str(&token.content);
                            }
                        },
                        TokenType::Number => {
                            // Convert numbers to Bengali numerals if applicable
                            let mut numeral_result = String::new();
                            let mut converted = false;
                            
                            for digit in token.content.chars() {
                                let digit_str = digit.to_string();
                                if let Some(bengali_digit) = self.numerals.get(digit_str.as_str()) {
                                    numeral_result.push_str(bengali_digit);
                                    converted = true;
                                } else {
                                    numeral_result.push(digit);
                                }
                            }
                            
                            if converted {
                                result.push_str(&numeral_result);
                            } else {
                                result.push_str(&token.content);
                            }
                        },
                        TokenType::Symbol => {
                            // Convert symbols if applicable
                            if let Some(bengali_symbol) = self.symbols.get(token.content.as_str()) {
                                result.push_str(bengali_symbol);
                            } else {
                                result.push_str(&token.content);
                            }
                        },
                    }
                }
                
                result
            },
            Err(e) => {
                // If sanitization failed, return the original text
                // In a real application, you might want to handle this differently
                eprintln!("Transliteration error: {}", e);
                text.to_string()
            }
        }
    }
    
    /// Tokenize the input text into words and other tokens
    pub fn tokenize(&self, text: &str) -> Vec<Token> {
        self.tokenizer.tokenize_text(text)
    }
    
    /// Tokenize a word into phonetic units
    pub fn tokenize_phonetic(&self, word: &str) -> Vec<PhoneticUnit> {
        self.tokenizer.tokenize_word(word)
    }
    
    /// Sanitize the input text, ensuring it contains only allowed characters
    pub fn sanitize(&self, text: &str) -> SanitizeResult {
        self.sanitizer.sanitize(text)
    }
    
    /// Transliterate Roman text to Bengali, cleaning invalid characters instead of returning an error
    pub fn transliterate_lenient(&self, text: &str) -> String {
        // Clean the input by removing invalid characters
        let cleaned = self.sanitizer.clean(text);
        
        // Process the cleaned text using the tokenizer
        let tokens = self.tokenizer.tokenize_text(&cleaned);
        
        // Process each token based on its type
        let mut result = String::new();
        
        for token in tokens {
            match token.token_type {
                TokenType::Word => {
                    result.push_str(&self.transliterate_word(&token.content));
                },
                TokenType::Whitespace => {
                    result.push_str(&token.content);
                },
                TokenType::Punctuation => {
                    // For most punctuation, keep it as is
                    // However, some punctuation might need to be converted
                    if let Some(bengali_symbol) = self.symbols.get(token.content.as_str()) {
                        result.push_str(bengali_symbol);
                    } else {
                        result.push_str(&token.content);
                    }
                },
                TokenType::Number => {
                    // Convert numbers to Bengali numerals if applicable
                    let mut numeral_result = String::new();
                    let mut converted = false;
                    
                    for digit in token.content.chars() {
                        let digit_str = digit.to_string();
                        if let Some(bengali_digit) = self.numerals.get(digit_str.as_str()) {
                            numeral_result.push_str(bengali_digit);
                            converted = true;
                        } else {
                            numeral_result.push(digit);
                        }
                    }
                    
                    if converted {
                        result.push_str(&numeral_result);
                    } else {
                        result.push_str(&token.content);
                    }
                },
                TokenType::Symbol => {
                    // Convert symbols if applicable
                    if let Some(bengali_symbol) = self.symbols.get(token.content.as_str()) {
                        result.push_str(bengali_symbol);
                    } else {
                        result.push_str(&token.content);
                    }
                },
            }
        }
        
        result
    }
    
    /// Transliterate a single word from Roman to Bengali
    fn transliterate_word(&self, word: &str) -> String {
        // Tokenize the word into phonetic units
        let phonetic_units = self.tokenizer.tokenize_word(word);
        
        println!("DEBUG: Transliterating word: {}", word);
        
        // Placeholder implementation - will be expanded later
        // For now, just mark the units in a debug-friendly way
        let mut result = String::new();
        let mut prev_was_consonant = false;
        let mut prev_was_bengali_consonant = false;
        
        for unit in phonetic_units {
            println!("DEBUG: Processing unit '{}' type: {:?}", unit.text, unit.unit_type);
            match unit.unit_type {
                PhoneticUnitType::Consonant => {
                    if let Some(bengali_consonant) = self.consonants.get(unit.text.as_str()) {
                        result.push_str(bengali_consonant);
                        prev_was_consonant = true;
                        prev_was_bengali_consonant = true;
                    } else {
                        // Fallback: keep original text
                        result.push_str(&unit.text);
                        prev_was_consonant = false;
                        prev_was_bengali_consonant = false;
                    }
                },
                PhoneticUnitType::Vowel => {
                    if let Some(vowel) = self.vowels.get(unit.text.as_str()) {
                        if prev_was_consonant {
                            // If preceded by a consonant, use dependent form if available
                            if let Some(dependent) = &vowel.dependent {
                                result.push_str(dependent);
                            } else {
                                // If no dependent form exists, use independent as fallback
                                result.push_str(&vowel.independent);
                            }
                        } else {
                            // Use the independent form for standalone vowels
                            result.push_str(&vowel.independent);
                        }
                        prev_was_consonant = false;
                    } else {
                        // Fallback: keep original text
                        result.push_str(&unit.text);
                        prev_was_consonant = false;
                    }
                },
                PhoneticUnitType::TerminatingVowel => {
                    if let Some(vowel) = self.vowels.get(unit.text.as_str()) {
                        if prev_was_consonant {
                            // If preceded by a consonant, use dependent form if available
                            if let Some(dependent) = &vowel.dependent {
                                result.push_str(dependent);
                            } else {
                                // If no dependent form exists, use independent as fallback
                                result.push_str(&vowel.independent);
                            }
                        } else {
                            // Use the independent form for standalone terminating vowels
                            result.push_str(&vowel.independent);
                        }
                        prev_was_consonant = false;
                    } else {
                        // Fallback: keep original text
                        result.push_str(&unit.text);
                        prev_was_consonant = false;
                    }
                },
                PhoneticUnitType::ConsonantWithVowel => {
                    println!("DEBUG: ConsonantWithVowel: '{}'", unit.text);
                    // Special case handling for 'chhi' sequence
                    if unit.text == "hi" && !result.is_empty() && result.ends_with('র') {
                        // If 'hi' follows a reph, handle differently
                        if let Some(bengali_consonant) = self.consonants.get("ch") {
                            // Create consonant with vowel
                            result.push_str(bengali_consonant);
                            if let Some(vowel) = self.vowels.get("i") {
                                if let Some(dependent) = &vowel.dependent {
                                    result.push_str(dependent);
                                } else {
                                    result.push_str(&vowel.independent);
                                }
                            }
                        } else {
                            // Fallback
                            result.push_str(&unit.text);
                        }
                    } else {
                        // Regular handling of consonant with vowel
                        // For consonants like "th" we need to check if they exist in our consonant map
                        // Extract the consonant and vowel parts
                        if let Some(pos) = find_vowel_position(&unit.text, &self.vowels) {
                            let consonant_part = &unit.text[0..pos];
                            let vowel_part = &unit.text[pos..];
                            
                            println!("DEBUG: Found vowel at position {}, consonant: '{}', vowel: '{}'", 
                                     pos, consonant_part, vowel_part);
                            
                            if let Some(bengali_consonant) = self.consonants.get(consonant_part) {
                                println!("DEBUG: Found consonant mapping: '{}' -> '{}'", consonant_part, bengali_consonant);
                                result.push_str(bengali_consonant);
                                if let Some(vowel) = self.vowels.get(vowel_part) {
                                    println!("DEBUG: Found vowel mapping: '{}' -> independent:'{}', dependent:{:?}", 
                                             vowel_part, vowel.independent, vowel.dependent);
                                    if let Some(dependent) = &vowel.dependent {
                                        result.push_str(dependent);
                                    } else {
                                        // Fallback to independent form if dependent not available
                                        result.push_str(&vowel.independent);
                                    }
                                } else {
                                    println!("DEBUG: Failed to find vowel mapping for: '{}'", vowel_part);
                                    // Vowel part not recognized, just append it
                                    result.push_str(vowel_part);
                                }
                            } else {
                                println!("DEBUG: Failed to find consonant mapping for: '{}'", consonant_part);
                                // Consonant not recognized, just use the original text
                                result.push_str(&unit.text);
                            }
                        } else {
                            // No vowel found, treat the whole thing as a consonant
                            if let Some(bengali_consonant) = self.consonants.get(unit.text.as_str()) {
                                result.push_str(bengali_consonant);
                            } else {
                                // Fallback: keep original text
                                result.push_str(&unit.text);
                            }
                        }
                    }
                    prev_was_consonant = false;
                },
                PhoneticUnitType::ConsonantWithTerminator => {
                    // Process consonant with terminating vowel (like o, O)
                    // For consonants like "th" we need to check if they exist in our consonant map
                    // Extract the consonant and terminator parts
                    if let Some(pos) = find_vowel_position(&unit.text, &self.vowels) {
                        let consonant_part = &unit.text[0..pos];
                        let terminator_part = &unit.text[pos..];
                        
                        if let Some(bengali_consonant) = self.consonants.get(consonant_part) {
                            // Add the consonant
                            result.push_str(bengali_consonant);
                            
                            // Handle the terminator - if it's 'o', it's the inherent vowel in Bengali
                            // and doesn't need a separate symbol
                            if terminator_part != "o" {
                                if let Some(vowel) = self.vowels.get(terminator_part) {
                                    if let Some(dependent) = &vowel.dependent {
                                        result.push_str(dependent);
                                    } else {
                                        // Fallback to independent form if dependent not available
                                        result.push_str(&vowel.independent);
                                    }
                                } else {
                                    // Terminator part not recognized, just append it
                                    result.push_str(terminator_part);
                                }
                            }
                        } else {
                            // Consonant not recognized, just use the original text
                            result.push_str(&unit.text);
                        }
                    } else {
                        // No vowel found, treat the whole thing as a consonant
                        if let Some(bengali_consonant) = self.consonants.get(unit.text.as_str()) {
                            result.push_str(bengali_consonant);
                        } else {
                            // Fallback: keep original text
                            result.push_str(&unit.text);
                        }
                    }
                    prev_was_consonant = false;
                },
                PhoneticUnitType::ConsonantWithHasant => {
                    // Special case for explicit hasant
                    if unit.text == ",," && !result.is_empty() {
                        // Apply hasant to the previous consonant
                        let hasant = self.diacritics.get(",,").unwrap_or(&"্");
                        result.push_str(hasant);
                    } else {
                        result.push_str(&unit.text);
                    }
                },
                PhoneticUnitType::Conjunct => {
                    // Process a conjunct based on the text structure
                    // Parse the text which will be in the format: consonant1,,consonant2,,...
                    let parts: Vec<&str> = unit.text.split(",,").collect();
                    
                    if parts.len() >= 2 {
                        // Process all parts as a multi-consonant conjunct
                        let mut valid_conjunct = true;
                        let mut conjunct_result = String::new();
                        let hasant = self.diacritics.get(",,").unwrap_or(&"্");
                        
                        // Build the conjunct by applying each consonant with hasant
                        for (i, consonant) in parts.iter().enumerate() {
                            if *consonant == "y" {
                                // Special case for য-ফলা (jo-phola)
                                conjunct_result.push_str("য");
                            } else if *consonant == "w" {
                                // Special case for ব-ফলা (bo-phola)
                                conjunct_result.push_str("ব");
                            } else if let Some(bengali) = self.consonants.get(*consonant) {
                                conjunct_result.push_str(bengali);
                            } else {
                                // Consonant not recognized
                                valid_conjunct = false;
                                break;
                            }
                            
                            // Add hasant to all except the last consonant
                            if i < parts.len() - 1 {
                                conjunct_result.push_str(hasant);
                            }
                        }
                        
                        if valid_conjunct {
                            result.push_str(&conjunct_result);
                        } else {
                            // Fallback if any consonant wasn't recognized
                            result.push_str(&unit.text);
                        }
                    } else {
                        // Invalid conjunct format
                        result.push_str(&unit.text);
                    }
                },
                PhoneticUnitType::ConjunctWithVowel => {
                    // Process a conjunct with vowel based on the text structure
                    // Parse the text which will be in format: consonant1,,consonant2,,consonant3...vowel
                    let parts: Vec<&str> = unit.text.split(",,").collect();
                    
                    if parts.len() >= 2 {
                        // Last part contains the final consonant with vowel
                        let last_part = parts.last().unwrap();
                        let consonant_parts = &parts[0..parts.len()-1];
                        
                        // Find where the vowel begins in the last part
                        if let Some(vowel_pos) = find_vowel_position(last_part, &self.vowels) {
                            let last_consonant = &last_part[0..vowel_pos];
                            let vowel_part = &last_part[vowel_pos..];
                            
                            // Build the multi-consonant conjunct
                            let mut valid_conjunct = true;
                            let mut conjunct_result = String::new();
                            let hasant = self.diacritics.get(",,").unwrap_or(&"্");
                            
                            // Add all consonants except the last one with hasant
                            for consonant in consonant_parts.iter() {
                                if *consonant == "y" {
                                    // Special case for য-ফলা (jo-phola)
                                    conjunct_result.push_str("য");
                                } else if *consonant == "w" {
                                    // Special case for ব-ফলা (bo-phola)
                                    conjunct_result.push_str("ব");
                                } else if let Some(bengali) = self.consonants.get(*consonant) {
                                    conjunct_result.push_str(bengali);
                                } else {
                                    valid_conjunct = false;
                                    break;
                                }
                                conjunct_result.push_str(hasant);
                            }
                            
                            // Add the last consonant
                            if valid_conjunct {
                                if last_consonant == "y" {
                                    // Special case for য-ফলা (jo-phola)
                                    conjunct_result.push_str("য");
                                } else if last_consonant == "w" {
                                    // Special case for ব-ফলা (bo-phola)
                                    conjunct_result.push_str("ব");
                                } else if let Some(last_bengali) = self.consonants.get(last_consonant) {
                                    conjunct_result.push_str(last_bengali);
                                } else {
                                    valid_conjunct = false;
                                }
                            }

                            // Add the vowel if the conjunct is valid
                            if valid_conjunct {
                                // Add the vowel as dependent form
                                if let Some(vowel) = self.vowels.get(vowel_part) {
                                    if let Some(dependent) = &vowel.dependent {
                                        conjunct_result.push_str(dependent);
                                    } else {
                                        // Fallback to independent form
                                        conjunct_result.push_str(&vowel.independent);
                                    }
                                    result.push_str(&conjunct_result);
                                } else {
                                    // Vowel not recognized, fallback to original text
                                    result.push_str(&unit.text);
                                }
                            } else {
                                // Conjunct formation failed, fallback to original text
                                result.push_str(&unit.text);
                            }
                        } else {
                            // No vowel found in last part
                            result.push_str(&unit.text);
                        }
                    } else {
                        // Invalid format
                        result.push_str(&unit.text);
                    }
                },
                PhoneticUnitType::ConjunctWithTerminator => {
                    // Process a conjunct with terminating vowel
                    // Parse the text which will be in format: consonant1,,consonant2,,consonant3...o
                    let parts: Vec<&str> = unit.text.split(",,").collect();
                    
                    if parts.len() >= 2 {
                        let last_part = parts.last().unwrap();
                        let consonant_parts = &parts[0..parts.len()-1];
                        
                        // Find where the 'o' terminator begins 
                        if let Some(vowel_pos) = last_part.find('o') {
                            let last_consonant = &last_part[0..vowel_pos];
                            
                            // Build the multi-consonant conjunct
                            let mut valid_conjunct = true;
                            let mut conjunct_result = String::new();
                            let hasant = self.diacritics.get(",,").unwrap_or(&"্");
                            
                            // Add all consonants except the last one with hasant
                            for consonant in consonant_parts.iter() {
                                if *consonant == "y" {
                                    // Special case for য-ফলা (jo-phola)
                                    conjunct_result.push_str("য");
                                } else if *consonant == "w" {
                                    // Special case for ব-ফলা (bo-phola)
                                    conjunct_result.push_str("ব");
                                } else if let Some(bengali) = self.consonants.get(*consonant) {
                                    conjunct_result.push_str(bengali);
                                } else {
                                    valid_conjunct = false;
                                    break;
                                }
                                conjunct_result.push_str(hasant);
                            }
                            
                            // Add the last consonant
                            if valid_conjunct {
                                if last_consonant == "y" {
                                    // Special case for য-ফলা (jo-phola)
                                    conjunct_result.push_str("য");
                                } else if last_consonant == "w" {
                                    // Special case for ব-ফলা (bo-phola)
                                    conjunct_result.push_str("ব");
                                } else if let Some(last_bengali) = self.consonants.get(last_consonant) {
                                    conjunct_result.push_str(last_bengali);
                                } else {
                                    valid_conjunct = false;
                                }
                            }
                            
                            // For 'o' terminator, no dependent vowel mark is needed
                            // as inherent 'o' sound is built into Bengali consonants
                            if valid_conjunct {
                                result.push_str(&conjunct_result);
                            } else {
                                // Conjunct formation failed, fallback to original text
                                result.push_str(&unit.text);
                            }
                        } else {
                            // No terminator found
                            result.push_str(&unit.text);
                        }
                    } else {
                        // Invalid format
                        result.push_str(&unit.text);
                    }
                },
                PhoneticUnitType::RephOverConsonant => {
                    // Process reph over consonant (র্ + consonant)
                    // Extract the consonant part (after "rr")
                    let consonant_text = &unit.text[2..]; // Skip the "rr" prefix
                    
                    if let Some(bengali_consonant) = self.consonants.get(consonant_text) {
                        // Create reph + consonant (reph comes before consonant in Bengali)
                        // In Bengali, reph is represented as র + hasant (্)
                        let reph = "র্"; // Fixed Bengali reph
                        result.push_str(reph);
                        result.push_str(bengali_consonant);
                    } else {
                        // Fallback: keep original text
                        result.push_str(&unit.text);
                    }
                },
                PhoneticUnitType::RephOverConsonantWithVowel => {
                    // Process reph over consonant with vowel (র্ + consonant + vowel)
                    // This is a complex form that needs to be processed properly
                    // For example, "rrka" should become "র্ক" + vowel sign
                    
                    // First identify where the consonant and vowel parts begin/end
                    // Skip the "rr" prefix to find the consonant
                    let consonant_start = 2; // After "rr"
                    let mut consonant_end = unit.text.len();
                    
                    // Find where the vowel begins by looking for the first non-consonant character
                    for i in consonant_start..unit.text.len() {
                        let c = &unit.text[i..i+1];
                        if self.vowels.contains_key(c) {
                            consonant_end = i;
                            break;
                        }
                    }
                    
                    let consonant_part = &unit.text[consonant_start..consonant_end];
                    let vowel_part = &unit.text[consonant_end..];
                    
                    if let Some(bengali_consonant) = self.consonants.get(consonant_part) {
                        if let Some(vowel) = self.vowels.get(vowel_part) {
                            // Create reph + consonant + vowel
                            let reph = "র্"; // Fixed Bengali reph
                            result.push_str(reph);
                            result.push_str(bengali_consonant);
                            
                            // Handle Option<&str> correctly for dependent vowel
                            if let Some(dependent_vowel) = &vowel.dependent {
                                result.push_str(dependent_vowel);
                            } else {
                                // If no dependent form exists, use independent as fallback
                                result.push_str(&vowel.independent);
                            }
                        } else {
                            // Vowel part not recognized
                            result.push_str(&unit.text);
                        }
                    } else {
                        // Consonant part not recognized
                        result.push_str(&unit.text);
                    }
                },
                PhoneticUnitType::RephOverConsonantWithTerminator => {
                    // Process reph over consonant with terminator (র্ + consonant + o)
                    // Similar to RephOverConsonantWithVowel but with terminator vowel
                    
                    // First identify where the consonant and vowel parts begin/end
                    // Skip the "rr" prefix to find the consonant
                    let consonant_start = 2; // After "rr"
                    let mut consonant_end = unit.text.len();
                    
                    // Find where the terminator begins by looking for the terminator character
                    for i in consonant_start..unit.text.len() {
                        let c = &unit.text[i..i+1];
                        if c == "o" {
                            consonant_end = i;
                            break;
                        }
                    }
                    
                    let consonant_part = &unit.text[consonant_start..consonant_end];
                    let terminator_part = if consonant_end < unit.text.len() {
                        &unit.text[consonant_end..]
                    } else {
                        ""
                    };
                    
                    if let Some(bengali_consonant) = self.consonants.get(consonant_part) {
                        // Create reph + consonant
                        let reph = "র্"; // Fixed Bengali reph
                        result.push_str(reph);
                        result.push_str(bengali_consonant);
                        
                        // Add terminator if present
                        if !terminator_part.is_empty() {
                            if let Some(vowel) = self.vowels.get(terminator_part) {
                                if let Some(dependent) = &vowel.dependent {
                                    result.push_str(dependent);
                                } else {
                                    result.push_str(&vowel.independent);
                                }
                            }
                        }
                    } else {
                        // Consonant part not recognized
                        result.push_str(&unit.text);
                    }
                },
                PhoneticUnitType::SpecialForm => {
                    // Special forms with proper text field handling
                    if unit.text == "rr" {
                        // Standalone reph is র্
                        result.push_str("র্");
                    } else if unit.text == "^" {
                        // Standalone Chandrabindu
                        if let Some(chandrabindu) = self.diacritics.get("^") {
                            result.push_str(chandrabindu);
                        } else {
                            result.push_str("ঁ");
                        }
                    } else if unit.text == ":" {
                        // Standalone Visarga - now handled directly here
                        if let Some(visarga) = self.diacritics.get(":") {
                            result.push_str(visarga);
                        } else {
                            result.push_str("ঃ");
                        }
                    } else if unit.text == "T``" {
                        // Handle Khanda Ta (special form of ta)
                        let khanda_ta = self.special_rules.get("T``").unwrap_or(&"ৎ");
                        result.push_str(khanda_ta);
                    } else if unit.text == "ng" {
                        // Handle anusvara (ং)
                        if let Some(anusvara) = self.diacritics.get("ng") {
                            result.push_str(anusvara);
                        } else {
                            result.push_str("ং");
                        }
                    } else {
                        // Try to find in special rules
                        if let Some(special_bengali) = self.special_rules.get(unit.text.as_str()) {
                            result.push_str(special_bengali);
                        } else {
                            // Fallback: keep original text
                            result.push_str(&unit.text);
                        }
                    }
                },
                PhoneticUnitType::Numeral => {
                    // Convert to Bengali numeral
                    let mut numeral_result = String::new();
                    for digit in unit.text.chars() {
                        let digit_str = digit.to_string();
                        if let Some(bengali_digit) = self.numerals.get(digit_str.as_str()) {
                            numeral_result.push_str(bengali_digit);
                        } else {
                            numeral_result.push(digit);
                        }
                    }
                    result.push_str(&numeral_result);
                },
                PhoneticUnitType::Symbol => {
                    // Convert to Bengali symbol if applicable
                    if let Some(bengali_symbol) = self.symbols.get(unit.text.as_str()) {
                        result.push_str(bengali_symbol);
                    } else {
                        result.push_str(&unit.text);
                    }
                },
                PhoneticUnitType::Unknown => {
                    // Special handling for 'w' as bo-phola when it follows a consonant
                    if unit.text == "w" && prev_was_bengali_consonant {
                        // Apply bo-phola (ব-ফলা) to the previous consonant
                        let hasant = self.diacritics.get(",,").unwrap_or(&"্");
                        let ba = "ব"; // Bengali letter 'ba' for bo-phola
                        result.push_str(hasant);
                        result.push_str(ba);
                        prev_was_consonant = false;
                        prev_was_bengali_consonant = false;
                    }
                    // Special handling for 'y' as jo-phola when it follows a consonant
                    else if unit.text == "y" && prev_was_bengali_consonant {
                        // Apply jo-phola (য-ফলা) to the previous consonant
                        let hasant = self.diacritics.get(",,").unwrap_or(&"্");
                        let ya = "য"; // Regular yo (য) for jo-phola
                        result.push_str(hasant);
                        result.push_str(ya);
                        prev_was_consonant = false;
                        prev_was_bengali_consonant = false;
                    }
                    else {
                        // Keep other unknown units as is
                        result.push_str(&unit.text);
                        prev_was_consonant = false;
                        prev_was_bengali_consonant = false;
                    }
                },
                PhoneticUnitType::ChandrabinduWithConsonant => {
                    // Handle consonant with chandrabindu (nasalization)
                    // Get the base consonant (all characters except the last one)
                    let consonant_text = &unit.text[0..unit.text.len()-1];
                    let chandrabindu = self.diacritics.get("^").unwrap_or(&"ঁ");
                    
                    if let Some(bengali_consonant) = self.consonants.get(consonant_text) {
                        result.push_str(bengali_consonant);
                        result.push_str(chandrabindu);
                    } else {
                        // Fallback: keep original text
                        result.push_str(&unit.text);
                    }
                },
                PhoneticUnitType::ChandrabinduWithVowel => {
                    // Handle vowel with chandrabindu (nasalization)
                    // Get the base vowel (all characters except the last one)
                    let vowel_text = &unit.text[0..unit.text.len()-1];
                    let chandrabindu = self.diacritics.get("^").unwrap_or(&"ঁ");
                    
                    if let Some(vowel) = self.vowels.get(vowel_text) {
                        if prev_was_consonant {
                            // If preceded by a consonant, use dependent form if available
                            if let Some(dependent) = &vowel.dependent {
                                result.push_str(dependent);
                            } else {
                                // If no dependent form exists, use independent as fallback
                                result.push_str(&vowel.independent);
                            }
                        } else {
                            // Use the independent form for standalone vowels
                            result.push_str(&vowel.independent);
                        }
                        result.push_str(chandrabindu);
                        prev_was_consonant = false;
                    } else {
                        // Fallback: keep original text
                        result.push_str(&unit.text);
                        prev_was_consonant = false;
                    }
                },
                PhoneticUnitType::ChandrabinduWithConsonantAndVowel => {
                    // Handle consonant + vowel + chandrabindu
                    // Extract the consonant, vowel, and chandrabindu parts
                    let base_text = &unit.text[0..unit.text.len()-1]; // Text without the chandrabindu
                    let chandrabindu = self.diacritics.get("^").unwrap_or(&"ঁ");
                    
                    if let Some(pos) = find_vowel_position(base_text, &self.vowels) {
                        let consonant_part = &base_text[0..pos];
                        let vowel_part = &base_text[pos..];
                        
                        if let Some(bengali_consonant) = self.consonants.get(consonant_part) {
                            result.push_str(bengali_consonant);
                            if let Some(vowel) = self.vowels.get(vowel_part) {
                                if let Some(dependent) = &vowel.dependent {
                                    result.push_str(dependent);
                                } else {
                                    // Fallback to independent form if dependent not available
                                    result.push_str(&vowel.independent);
                                }
                            } else {
                                // Vowel part not recognized, just append it
                                result.push_str(vowel_part);
                            }
                            result.push_str(chandrabindu);
                        } else {
                            // Consonant not recognized, just use the original text
                            result.push_str(base_text);
                            result.push_str(chandrabindu);
                        }
                    } else {
                        // No vowel found, treat the whole thing as a consonant with chandrabindu
                        if let Some(bengali_consonant) = self.consonants.get(base_text) {
                            result.push_str(bengali_consonant);
                            result.push_str(chandrabindu);
                        } else {
                            // Fallback: keep original text
                            result.push_str(base_text);
                            result.push_str(chandrabindu);
                        }
                    }
                    prev_was_consonant = false;
                },
            }
        }
        
        result
    }
}

impl Default for Transliterator {
    fn default() -> Self {
        Self::new()
    }
}

// Helper function to find where the vowel part starts in a string
fn find_vowel_position(text: &str, vowels: &HashMap<&str, BengaliVowel>) -> Option<usize> {
    println!("DEBUG: Finding vowel position in: '{}'", text);
    
    // Try longer vowels first
    let mut vowel_patterns: Vec<&&str> = vowels.keys().collect();
    vowel_patterns.sort_by(|a, b| b.len().cmp(&a.len())); // Sort by length, descending
    
    // Try each position from the start of the string
    for start_pos in 0..text.len() {
        for &vowel in &vowel_patterns {
            if start_pos + vowel.len() <= text.len() && &text[start_pos..start_pos + vowel.len()] == *vowel {
                println!("DEBUG: Found vowel '{}' at position {}", vowel, start_pos);
                return Some(start_pos);
            }
        }
    }
    
    println!("DEBUG: No vowel found in '{}'", text);
    None
}