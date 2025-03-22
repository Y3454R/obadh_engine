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
    fn create_conjunct(&self, c1: &str, c2: &str) -> String {
        let hasant = self.diacritics.get(",,").unwrap_or(&"্");
        format!("{}{}{}", c1, hasant, c2)
    }
    
    /// Add a hasant to a consonant (used when explicitly adding hasant with ,,)
    fn add_hasant(&self, consonant: &str) -> String {
        let hasant = self.diacritics.get(",,").unwrap_or(&"্");
        format!("{}{}", consonant, hasant)
    }
    
    /// Create a reph form when "rr" is followed by another consonant
    fn create_reph(&self, consonant: &str) -> String {
        let hasant = self.diacritics.get(",,").unwrap_or(&"্");
        format!("র{}{}", hasant, consonant)
    }
    
    /// Create y-phola form (ya-phalā) by joining a consonant with য
    fn create_ya_phala(&self, consonant: &str) -> String {
        // For y-phola, we join the consonant with য using hasant
        let hasant = self.diacritics.get(",,").unwrap_or(&"্");
        let ya = self.consonants.get("z").unwrap_or(&"য");
        format!("{}{}{}", consonant, hasant, ya)
    }
    
    /// Create w-phola form (ba-phalā) by joining a consonant with ব
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
        
        // Placeholder implementation - will be expanded later
        // For now, just mark the units in a debug-friendly way
        let mut result = String::new();
        
        for unit in phonetic_units {
            match unit.unit_type {
                PhoneticUnitType::Consonant => {
                    if let Some(bengali_consonant) = self.consonants.get(unit.text.as_str()) {
                        result.push_str(bengali_consonant);
                    } else {
                        // Fallback: keep original text
                        result.push_str(&unit.text);
                    }
                },
                PhoneticUnitType::Vowel => {
                    if let Some(vowel) = self.vowels.get(unit.text.as_str()) {
                        // Use the independent form for standalone vowels
                        result.push_str(&vowel.independent);
                    } else {
                        // Fallback: keep original text
                        result.push_str(&unit.text);
                    }
                },
                PhoneticUnitType::ConsonantWithVowel => {
                    // This is a complex unit that needs to be processed
                    // For now, just return the original text as a placeholder
                    result.push_str(&unit.text);
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
                    // This is a complex unit that represents a conjunct
                    // For now, just return the original text as a placeholder
                    result.push_str(&unit.text);
                },
                PhoneticUnitType::SpecialForm => {
                    // Special forms like reph
                    if unit.text == "rr" && !result.is_empty() {
                        // Placeholder for reph handling
                        result.push_str("র্");
                    } else {
                        result.push_str(&unit.text);
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
                    // Keep unknown units as is
                    result.push_str(&unit.text);
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