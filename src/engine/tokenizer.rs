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
        
        // Simple vowels
        patterns.insert("a".into());
        patterns.insert("i".into());
        patterns.insert("u".into());
        patterns.insert("e".into());
        patterns.insert("o".into());
        
        // Long vowels
        patterns.insert("aa".into());
        patterns.insert("ii".into());
        patterns.insert("uu".into());
        
        // Diphthongs
        patterns.insert("oi".into());
        patterns.insert("ou".into());
        patterns.insert("oo".into());
        patterns.insert("ri".into());
        
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
        let mut current_position = 0;
        let text_chars: Vec<char> = text.chars().collect();
        
        while current_position < text_chars.len() {
            let remaining: String = text_chars[current_position..].iter().collect();
            
            // Check for whitespace
            if text_chars[current_position].is_whitespace() {
                let whitespace: String = text_chars[current_position..].iter()
                    .take_while(|c| c.is_whitespace())
                    .collect();
                
                tokens.push(Token {
                    text: whitespace.clone(),
                    token_type: TokenType::Whitespace,
                    position: None,
                });
                
                current_position += whitespace.chars().count();
                continue;
            }
            
            // Check for punctuation
            if self.preserve_punctuation && PUNCTUATION_MAP.contains_key(&text_chars[current_position]) {
                tokens.push(Token {
                    text: text_chars[current_position].to_string(),
                    token_type: TokenType::Punctuation,
                    position: None,
                });
                
                current_position += 1;
                continue;
            }
            
            // Check for numbers
            if self.bengali_numbers && text_chars[current_position].is_ascii_digit() {
                let number: String = text_chars[current_position..].iter()
                    .take_while(|c| c.is_ascii_digit())
                    .collect();
                
                tokens.push(Token {
                    text: number.clone(),
                    token_type: TokenType::Number,
                    position: None,
                });
                
                current_position += number.chars().count();
                continue;
            }
            
            // Try to match the longest pattern first
            let mut matched = false;
            let patterns_to_check = [
                // Check consonant patterns
                (&*CONSONANT_PATTERNS, TokenType::Consonant),
                // Check vowel patterns
                (&*VOWEL_PATTERNS, TokenType::Vowel),
                // Check modifier patterns
                (&*MODIFIER_PATTERNS, TokenType::Modifier),
            ];
            
            for (patterns, token_type) in patterns_to_check.iter() {
                // Sort patterns by length (longest first)
                let mut sorted_patterns: Vec<&String> = patterns.iter().collect();
                sorted_patterns.sort_by(|a, b| b.len().cmp(&a.len()));
                
                for pattern in sorted_patterns {
                    if remaining.starts_with(pattern) {
                        tokens.push(Token {
                            text: pattern.clone(),
                            token_type: token_type.clone(),
                            position: None,  // Will be determined later
                        });
                        
                        current_position += pattern.chars().count();
                        matched = true;
                        break;
                    }
                }
                
                if matched {
                    break;
                }
            }
            
            // If no pattern matched, capture as "Other"
            if !matched {
                tokens.push(Token {
                    text: text_chars[current_position].to_string(),
                    token_type: TokenType::Other,
                    position: None,
                });
                
                current_position += 1;
            }
        }
        
        // Determine token positions (initial, medial, final, isolated)
        self.determine_token_positions(&mut tokens);
        
        tokens
    }
    
    /// Determine the position of each token in its context
    fn determine_token_positions(&self, tokens: &mut [Token]) {
        // Group tokens by words (separated by whitespace/punctuation)
        let mut word_start = 0;
        
        while word_start < tokens.len() {
            // Skip whitespace and punctuation
            while word_start < tokens.len() && 
                  (tokens[word_start].token_type == TokenType::Whitespace || 
                   tokens[word_start].token_type == TokenType::Punctuation) {
                word_start += 1;
            }
            
            if word_start >= tokens.len() {
                break;
            }
            
            // Find word end
            let mut word_end = word_start + 1;
            while word_end < tokens.len() && 
                  tokens[word_end].token_type != TokenType::Whitespace && 
                  tokens[word_end].token_type != TokenType::Punctuation {
                word_end += 1;
            }
            
            // Set positions for tokens in this word
            if word_end - word_start == 1 {
                // Single token word
                tokens[word_start].position = Some(TokenPosition::Isolated);
            } else {
                // Multi-token word
                tokens[word_start].position = Some(TokenPosition::Initial);
                
                for i in word_start + 1..word_end - 1 {
                    tokens[i].position = Some(TokenPosition::Medial);
                }
                
                tokens[word_end - 1].position = Some(TokenPosition::Final);
            }
            
            // Move to next word
            word_start = word_end;
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tokenize_simple_word() {
        let tokenizer = Tokenizer::new();
        let tokens = tokenizer.tokenize("ami");
        
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].text, "a");
        assert_eq!(tokens[0].token_type, TokenType::Vowel);
        assert_eq!(tokens[1].text, "m");
        assert_eq!(tokens[1].token_type, TokenType::Consonant);
        assert_eq!(tokens[2].text, "i");
        assert_eq!(tokens[2].token_type, TokenType::Vowel);
    }
    
    #[test]
    fn test_tokenize_with_conjuncts() {
        let tokenizer = Tokenizer::new();
        let tokens = tokenizer.tokenize("bangla");
        
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].text, "b");
        assert_eq!(tokens[1].text, "a");
        assert_eq!(tokens[2].text, "ng");
        assert_eq!(tokens[3].text, "l");
        assert_eq!(tokens[4].text, "a");
    }
    
    #[test]
    fn test_token_positions() {
        let tokenizer = Tokenizer::new();
        let tokens = tokenizer.tokenize("ami tumi");
        
        assert_eq!(tokens[0].position, Some(TokenPosition::Initial));
        assert_eq!(tokens[1].position, Some(TokenPosition::Medial));
        assert_eq!(tokens[2].position, Some(TokenPosition::Final));
        assert_eq!(tokens[3].position, None); // Whitespace
        assert_eq!(tokens[4].position, Some(TokenPosition::Initial));
        assert_eq!(tokens[5].position, Some(TokenPosition::Medial));
        assert_eq!(tokens[6].position, Some(TokenPosition::Final));
    }
}