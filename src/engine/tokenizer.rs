//! Tokenizer for Roman text input.
//!
//! The tokenizer breaks input text into linguistically meaningful tokens
//! using a phonological approach rather than character-by-character.

use std::collections::{HashSet, HashMap};
use lazy_static::lazy_static;

/// Types of tokens recognized by the tokenizer
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    /// Consonant like 'k', 'kh', 'g', etc.
    Consonant,
    /// Vowel like 'a', 'i', 'u', etc.
    Vowel,
    /// Modifier like '.', '^', etc.
    Modifier,
    /// Whitespace
    Whitespace,
    /// Punctuation
    Punctuation,
    /// Number
    Number,
    /// Other characters
    Other,
}

/// A token representing a phonological unit in the input text
#[derive(Debug, Clone)]
pub struct Token {
    /// The text of the token
    pub text: String,
    /// The type of the token
    pub token_type: TokenType,
    /// Optional positional information
    pub position: Option<TokenPosition>,
}

/// Position of a token in the larger context
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenPosition {
    /// Token is at the start of a word
    Initial,
    /// Token is in the middle of a word
    Medial,
    /// Token is at the end of a word
    Final,
    /// Token is a standalone entity
    Isolated,
}

lazy_static! {
    /// Pattern sets for efficient token recognition
    static ref CONSONANT_PATTERNS: HashSet<String> = {
        let mut patterns = HashSet::new();
        
        // Simple consonants
        patterns.insert("k".into());
        patterns.insert("g".into());
        patterns.insert("c".into());
        patterns.insert("j".into());
        patterns.insert("T".into());
        patterns.insert("D".into());
        patterns.insert("N".into());
        patterns.insert("t".into());
        patterns.insert("d".into());
        patterns.insert("n".into());
        patterns.insert("p".into());
        patterns.insert("f".into());
        patterns.insert("b".into());
        patterns.insert("v".into());
        patterns.insert("m".into());
        patterns.insert("z".into());
        patterns.insert("r".into());
        patterns.insert("l".into());
        patterns.insert("sh".into());
        patterns.insert("S".into());
        patterns.insert("s".into());
        patterns.insert("h".into());
        patterns.insert("y".into());
        patterns.insert("w".into());
        
        // Aspirated consonants and compound sounds
        patterns.insert("kh".into());
        patterns.insert("gh".into());
        patterns.insert("ch".into());
        patterns.insert("jh".into());
        patterns.insert("Th".into());
        patterns.insert("Dh".into());
        patterns.insert("th".into());
        patterns.insert("dh".into());
        patterns.insert("ph".into());
        patterns.insert("bh".into());
        patterns.insert("ng".into());
        patterns.insert("chh".into());
        
        // Special combinations
        patterns.insert("ksh".into());
        patterns.insert("gj".into());
        patterns.insert("jn".into());
        
        patterns
    };
    
    static ref VOWEL_PATTERNS: HashSet<String> = {
        let mut patterns = HashSet::new();
        
        // Basic vowels as per the documentation
        patterns.insert("o".into());     // অ-কার (a-kar)
        patterns.insert("A".into());     // আ-কার (aa-kar)
        patterns.insert("i".into());     // ই-কার (i-kar)
        patterns.insert("I".into());     // ঈ-কার (dirgho i-kar)
        patterns.insert("u".into());     // উ-কার (u-kar)
        patterns.insert("U".into());     // ঊ-কার (dirgho u-kar)
        patterns.insert("e".into());     // এ-কার (e-kar)
        patterns.insert("OI".into());    // ঐ-কার (oi-kar)
        patterns.insert("O".into());     // ও-কার (o-kar)
        patterns.insert("OU".into());    // ঔ-কার (ou-kar)
        patterns.insert("rri".into());   // ঋ-কার (ri-kar)
        
        // Common alternative spellings for backward compatibility
        patterns.insert("a".into());     // Equivalent to 'A'
        patterns.insert("aa".into());    // Equivalent to 'A'
        patterns.insert("oi".into());    // Equivalent to 'OI'
        patterns.insert("ou".into());    // Equivalent to 'OU'
        
        // Vowel+vowel combinations
        patterns.insert("ai".into());    // a + i -> আই
        patterns.insert("au".into());    // a + u -> আউ
        patterns.insert("ae".into());    // a + e -> আএ
        patterns.insert("ao".into());    // a + o -> আও
        patterns.insert("ia".into());    // i + a -> ইয়া
        patterns.insert("io".into());    // i + o -> ইও
        patterns.insert("eo".into());    // e + o -> এও
        
        patterns
    };
    
    static ref MODIFIER_PATTERNS: HashSet<String> = {
        let mut patterns = HashSet::new();
        
        // Hasanta, chandrabindu, etc.
        patterns.insert(".".into());
        patterns.insert("^".into());
        patterns.insert("~".into());
        patterns.insert(":".into());
        
        patterns
    };
    
    static ref PUNCTUATION_MAP: HashMap<char, &'static str> = {
        let mut map = HashMap::new();
        
        // Map Roman punctuation to Bengali
        map.insert('.', "।");
        map.insert('?', "?");
        map.insert('!', "!");
        map.insert(',', ",");
        map.insert(';', ";");
        map.insert(':', ":");
        map.insert('(', "(");
        map.insert(')', ")");
        map.insert('[', "[");
        map.insert(']', "]");
        map.insert('{', "{");
        map.insert('}', "}");
        map.insert('"', "\"");
        map.insert('\'', "'");
        
        map
    };
    
    static ref NUMBER_MAP: HashMap<char, &'static str> = {
        let mut map = HashMap::new();
        
        // Map Roman numerals to Bengali
        map.insert('0', "০");
        map.insert('1', "১");
        map.insert('2', "২");
        map.insert('3', "৩");
        map.insert('4', "৪");
        map.insert('5', "৫");
        map.insert('6', "৬");
        map.insert('7', "৭");
        map.insert('8', "৮");
        map.insert('9', "৯");
        
        map
    };
}

/// Smart tokenizer that understands Bengali linguistic patterns
pub struct Tokenizer {
    // Configuration options
    preserve_punctuation: bool,
    bengali_numbers: bool,
}

impl Tokenizer {
    /// Create a new tokenizer with default settings
    pub fn new() -> Self {
        Tokenizer {
            preserve_punctuation: true,
            bengali_numbers: true,
        }
    }
    
    /// Configure whether to preserve punctuation in output
    pub fn with_preserve_punctuation(mut self, preserve: bool) -> Self {
        self.preserve_punctuation = preserve;
        self
    }
    
    /// Configure whether to convert numbers to Bengali
    pub fn with_bengali_numbers(mut self, bengali: bool) -> Self {
        self.bengali_numbers = bengali;
        self
    }
    
    /// Tokenize the input text using a longest-match approach
    pub fn tokenize(&self, text: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut i = 0;
        let chars: Vec<char> = text.chars().collect();
        
        while i < chars.len() {
            // Skip processing if we've already hit the end
            if i >= chars.len() {
                break;
            }
            
            // First try different pattern lengths for more accurate tokenization
            let mut found = false;
            
            // Try longest patterns first (3-char)
            if i + 2 < chars.len() {
                let three_chars: String = chars[i..=i+2].iter().collect();
                if CONSONANT_PATTERNS.contains(&three_chars) {
                    tokens.push(Token {
                        text: three_chars,
                        token_type: TokenType::Consonant,
                        position: None,
                    });
                    i += 3;
                    found = true;
                } else if VOWEL_PATTERNS.contains(&three_chars) {
                    tokens.push(Token {
                        text: three_chars,
                        token_type: TokenType::Vowel,
                        position: None,
                    });
                    i += 3;
                    found = true;
                }
            }
            
            // Try 2-char combinations
            if !found && i + 1 < chars.len() {
                let two_chars: String = chars[i..=i+1].iter().collect();
                
                // Special handling for "oi" - always break into separate vowels
                if two_chars == "oi" {
                    // Always separate "o" and "i" for correct Avro behavior
                    tokens.push(Token {
                        text: "o".to_string(),
                        token_type: TokenType::Vowel,
                        position: None,
                    });
                    
                    tokens.push(Token {
                        text: "i".to_string(),
                        token_type: TokenType::Vowel,
                        position: None,
                    });
                    
                    i += 2;
                    found = true;
                } else if CONSONANT_PATTERNS.contains(&two_chars) {
                    tokens.push(Token {
                        text: two_chars,
                        token_type: TokenType::Consonant,
                        position: None,
                    });
                    i += 2;
                    found = true;
                } else if VOWEL_PATTERNS.contains(&two_chars) {
                    tokens.push(Token {
                        text: two_chars,
                        token_type: TokenType::Vowel,
                        position: None,
                    });
                    i += 2;
                    found = true;
                } else if MODIFIER_PATTERNS.contains(&two_chars) {
                    tokens.push(Token {
                        text: two_chars,
                        token_type: TokenType::Modifier,
                        position: None,
                    });
                    i += 2;
                    found = true;
                }
            }
            
            // Handle single-character tokens if multi-char not found
            if !found {
                // Check for single character patterns, preserve case for 'o' vs 'O'
                let single_char = chars[i].to_string();
                
                if CONSONANT_PATTERNS.contains(&single_char) {
                    tokens.push(Token {
                        text: single_char,
                        token_type: TokenType::Consonant,
                        position: None,
                    });
                    i += 1;
                    continue;
                } else if VOWEL_PATTERNS.contains(&single_char) {
                    // Ensure we preserve case for 'o' vs 'O'
                    tokens.push(Token {
                        text: single_char,
                        token_type: TokenType::Vowel,
                        position: None,
                    });
                    i += 1;
                    continue;
                } else if MODIFIER_PATTERNS.contains(&single_char) {
                    tokens.push(Token {
                        text: single_char,
                        token_type: TokenType::Modifier,
                        position: None,
                    });
                    i += 1;
                    continue;
                }
                
                // Check for whitespace
                if chars[i].is_whitespace() {
                    let whitespace: String = chars[i..].iter()
                        .take_while(|c| c.is_whitespace())
                        .collect();
                    
                    tokens.push(Token {
                        text: whitespace.clone(),
                        token_type: TokenType::Whitespace,
                        position: None,
                    });
                    
                    i += whitespace.chars().count();
                    continue;
                }
                
                // Check for punctuation
                if self.preserve_punctuation && PUNCTUATION_MAP.contains_key(&chars[i]) {
                    tokens.push(Token {
                        text: chars[i].to_string(),
                        token_type: TokenType::Punctuation,
                        position: None,
                    });
                    
                    i += 1;
                    continue;
                }
                
                // Check for numbers
                if self.bengali_numbers && chars[i].is_ascii_digit() {
                    let number: String = chars[i..].iter()
                        .take_while(|c| c.is_ascii_digit())
                        .collect();
                    
                    tokens.push(Token {
                        text: number.clone(),
                        token_type: TokenType::Number,
                        position: None,
                    });
                    
                    i += number.chars().count();
                    continue;
                }
                
                // If no pattern matched, capture as "Other"
                tokens.push(Token {
                    text: chars[i].to_string(),
                    token_type: TokenType::Other,
                    position: None,
                });
                
                i += 1;
            }
        }
        
        // Determine token positions (initial, medial, final, isolated)
        self.determine_token_positions(&mut tokens);
        
        tokens
    }
    
    /// Determine the position of each token in its context
    /// This is critical for proper vowel handling in Bengali
    fn determine_token_positions(&self, tokens: &mut [Token]) {
        let mut i = 0;
        while i < tokens.len() {
            // Skip non-language tokens (whitespace, punctuation, etc.)
            if tokens[i].token_type == TokenType::Whitespace || 
               tokens[i].token_type == TokenType::Punctuation ||
               tokens[i].token_type == TokenType::Other {
                i += 1;
                continue;
            }
            
            // Find the start of the current word
            let word_start = i;
            
            // Find the end of the current word
            let mut word_end = word_start;
            while word_end < tokens.len() && 
                  tokens[word_end].token_type != TokenType::Whitespace && 
                  tokens[word_end].token_type != TokenType::Punctuation &&
                  tokens[word_end].token_type != TokenType::Other {
                word_end += 1;
            }
            
            // Now we have the range of the current word: [word_start, word_end)
            
            // Set positions for tokens in this word
            if word_end - word_start == 1 {
                // Single token word
                tokens[word_start].position = Some(TokenPosition::Isolated);
            } else {
                // Multi-token word
                tokens[word_start].position = Some(TokenPosition::Initial);
                
                for j in word_start + 1..word_end - 1 {
                    tokens[j].position = Some(TokenPosition::Medial);
                }
                
                tokens[word_end - 1].position = Some(TokenPosition::Final);
            }
            
            // Move to next token after the word
            i = word_end;
        }
    }
    
    /// Convert a punctuation token to its Bengali equivalent
    pub fn convert_punctuation(&self, token: &Token) -> String {
        if token.token_type == TokenType::Punctuation {
            if let Some(first_char) = token.text.chars().next() {
                if let Some(&bengali_punct) = PUNCTUATION_MAP.get(&first_char) {
                    return bengali_punct.to_string();
                }
            }
        }
        
        token.text.clone()
    }
    
    /// Convert a number token to its Bengali equivalent
    pub fn convert_number(&self, token: &Token) -> String {
        if token.token_type == TokenType::Number {
            token.text.chars()
                //.map(|c| NUMBER_MAP.get(&c).cloned().unwrap_or_else(|| c.to_string()))
                // .map(|c| NUMBER_MAP.get(&c).cloned().unwrap_or_else(|| c.to_string()))
                .map(|c| match NUMBER_MAP.get(&c) {
                    Some(bengali_digit) => bengali_digit.to_string(),
                    None => c.to_string()
                })
                .collect()
        } else {
            token.text.clone()
        }
    }
}

impl Default for Tokenizer {
    fn default() -> Self {
        Self::new()
    }
}