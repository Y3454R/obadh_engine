//! Implementation of Bengali phonological rules.
//!
//! This module handles the application of linguistic rules to
//! transform Roman tokens into Bengali phonological units.

use crate::engine::tokenizer::{Token, TokenType, TokenPosition};
use crate::linguistic::phoneme::{Phoneme, PhonemeType};
use crate::linguistic::syllable::Syllable;
use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    /// Mapping of Roman consonants to Bengali phonemes
    static ref CONSONANT_MAP: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        
        // Velar consonants
        map.insert("k", "ক");
        map.insert("kh", "খ");
        map.insert("g", "গ");
        map.insert("gh", "ঘ");
        map.insert("ng", "ঙ");
        
        // Palatal consonants
        map.insert("c", "চ");
        map.insert("ch", "চ");
        map.insert("chh", "ছ");
        map.insert("j", "জ");
        map.insert("jh", "ঝ");
        map.insert("n", "ঞ");  // Palatal nasal (context-dependent)
        
        // Retroflex consonants
        map.insert("T", "ট");
        map.insert("Th", "ঠ");
        map.insert("D", "ড");
        map.insert("Dh", "ঢ");
        map.insert("N", "ণ");
        
        // Dental consonants
        map.insert("t", "ত");
        map.insert("th", "থ");
        map.insert("d", "দ");
        map.insert("dh", "ধ");
        map.insert("n", "ন");  // Dental nasal (default)
        
        // Labial consonants
        map.insert("p", "প");
        map.insert("ph", "ফ");
        map.insert("f", "ফ");  // Alternative for 'ph'
        map.insert("b", "ব");
        map.insert("bh", "ভ");
        map.insert("m", "ম");
        
        // Semivowels and approximants
        map.insert("y", "য়");
        map.insert("r", "র");
        map.insert("l", "ল");
        map.insert("sh", "শ");  // Post-alveolar fricative
        map.insert("S", "ষ");   // Retroflex fricative
        map.insert("s", "স");   // Dental fricative
        map.insert("h", "হ");
        map.insert("v", "ভ");   // Often pronounced as 'bh' in Bengali
        map.insert("w", "ও");   // For foreign words
        map.insert("z", "য");   // For foreign words
        
        // Special phonemes
        map.insert("Y", "য়");  // Ya-phala
        map.insert("R", "ড়");  // Bengali ড়
        map.insert("Rh", "ঢ়"); // Bengali ঢ়
        
        map
    };
    
    /// Mapping of Roman vowels to Bengali phonemes (independent form)
    static ref VOWEL_MAP_INDEPENDENT: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        
        map.insert("a", "অ");
        map.insert("aa", "আ");
        map.insert("i", "ই");
        map.insert("ii", "ঈ");
        map.insert("u", "উ");
        map.insert("uu", "ঊ");
        map.insert("e", "এ");
        map.insert("oi", "ঐ");
        map.insert("o", "ও");
        map.insert("ou", "ঔ");
        map.insert("oo", "উ");  // Alternative for 'u'
        map.insert("ri", "ঋ");  // Vocalic R
        
        map
    };
    
    /// Mapping of Roman vowels to Bengali phonemes (dependent form)
    static ref VOWEL_MAP_DEPENDENT: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        
        map.insert("a", "া");    // া is used for explicit 'a' after consonant
        map.insert("aa", "া");
        map.insert("i", "ি");
        map.insert("ii", "ী");
        map.insert("u", "ু");
        map.insert("uu", "ূ");
        map.insert("e", "ে");
        map.insert("oi", "ৈ");
        map.insert("o", "ো");
        map.insert("ou", "ৌ");
        map.insert("oo", "ু");   // Alternative for 'u'
        map.insert("ri", "ৃ");   // Vocalic R
        
        map
    };
    
    /// Mapping of Roman modifiers to Bengali phonemes
    static ref MODIFIER_MAP: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        
        map.insert(".", "্");     // Hasanta/virama
        map.insert(":", "ঃ");     // Visarga
        map.insert("^", "ঁ");     // Chandrabindu
        map.insert("~", "ঁ");     // Alternative for chandrabindu
        
        map
    };
    
    /// Bengali conjunct components to special forms
    static ref SPECIAL_CONJUNCTS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        
        // Special conjunct forms (truly exceptional)
        map.insert("ক্ষ", "ক্ষ");  // Khiyo (kSa)
        map.insert("জ্ঞ", "জ্ঞ");  // Gyanau (jina)
        map.insert("ন্ন", "ন্ন");  // Double n
        map.insert("র্‍", "র্‍");  // Special reph
        
        map
    };
}

/// Handles application of Bengali phonological rules
pub struct PhonologyEngine {
    // Configuration options
    handle_inherent_vowels: bool,
    handle_reph: bool,
    handle_ya_phala: bool,
}

impl PhonologyEngine {
    /// Create a new phonology engine with default settings
    pub fn new() -> Self {
        PhonologyEngine {
            handle_inherent_vowels: true,
            handle_reph: true,
            handle_ya_phala: true,
        }
    }
    
    /// Configure whether to apply inherent vowel rules
    pub fn with_inherent_vowels(mut self, enable: bool) -> Self {
        self.handle_inherent_vowels = enable;
        self
    }
    
    /// Configure whether to handle reph (র্)
    pub fn with_reph(mut self, enable: bool) -> Self {
        self.handle_reph = enable;
        self
    }
    
    /// Configure whether to handle ya-phala (্য)
    pub fn with_ya_phala(mut self, enable: bool) -> Self {
        self.handle_ya_phala = enable;
        self
    }
    
    /// Convert tokens to phonemes
    pub fn tokens_to_phonemes(&self, tokens: &[Token]) -> Vec<Phoneme> {
        let mut phonemes = Vec::new();
        
        for token in tokens {
            match token.token_type {
                TokenType::Consonant => {
                    if let Some(bengali) = CONSONANT_MAP.get(token.text.as_str()) {
                        phonemes.push(Phoneme {
                            roman: token.text.clone(),
                            bengali: bengali.to_string(),
                            phoneme_type: PhonemeType::Consonant,
                            position: token.position.clone(),
                        });
                    } else {
                        // Fallback for unrecognized consonants
                        phonemes.push(Phoneme {
                            roman: token.text.clone(),
                            bengali: token.text.clone(),
                            phoneme_type: PhonemeType::Consonant,
                            position: token.position.clone(),
                        });
                    }
                },
                TokenType::Vowel => {
                    let map = match token.position {
                        Some(TokenPosition::Initial) | 
                        Some(TokenPosition::Isolated) => &*VOWEL_MAP_INDEPENDENT,
                        _ => &*VOWEL_MAP_DEPENDENT,
                    };
                    
                    if let Some(bengali) = map.get(token.text.as_str()) {
                        phonemes.push(Phoneme {
                            roman: token.text.clone(),
                            bengali: bengali.to_string(),
                            phoneme_type: PhonemeType::Vowel,
                            position: token.position.clone(),
                        });
                    } else {
                        // Fallback for unrecognized vowels
                        phonemes.push(Phoneme {
                            roman: token.text.clone(),
                            bengali: token.text.clone(),
                            phoneme_type: PhonemeType::Vowel,
                            position: token.position.clone(),
                        });
                    }
                },
                TokenType::Modifier => {
                    if let Some(bengali) = MODIFIER_MAP.get(token.text.as_str()) {
                        phonemes.push(Phoneme {
                            roman: token.text.clone(),
                            bengali: bengali.to_string(),
                            phoneme_type: PhonemeType::Modifier,
                            position: token.position.clone(),
                        });
                    } else {
                        // Fallback for unrecognized modifiers
                        phonemes.push(Phoneme {
                            roman: token.text.clone(),
                            bengali: token.text.clone(),
                            phoneme_type: PhonemeType::Modifier,
                            position: token.position.clone(),
                        });
                    }
                },
                TokenType::Whitespace => {
                    phonemes.push(Phoneme {
                        roman: token.text.clone(),
                        bengali: token.text.clone(),
                        phoneme_type: PhonemeType::Whitespace,
                        position: None,
                    });
                },
                TokenType::Punctuation => {
                    // Convert to Bengali punctuation if available
                    // let bengali_punct = if let Some(first_char) = token.text.chars().next() {
                    //     match first_char {
                    //         '.' => "।",
                    //         '?' => "?",
                    //         '!' => "!",
                    //         // c => c.to_string().as_str(),
                    //         c => {
                    //             c.to_string().as_str()
                    //         },
                    //     }.to_string()
                    // } else {
                    //     token.text.clone()
                    // };
                    let bengali_punct = if let Some(first_char) = token.text.chars().next() {
                        match first_char {
                            '.' => "।".to_string(),
                            '?' => "?".to_string(),
                            '!' => "!".to_string(),
                            c => c.to_string(),
                        }
                    } else {
                        token.text.clone()
                    };
                    
                    phonemes.push(Phoneme {
                        roman: token.text.clone(),
                        bengali: bengali_punct,
                        phoneme_type: PhonemeType::Punctuation,
                        position: None,
                    });
                },
                TokenType::Number => {
                    // Convert to Bengali numerals
                    let bengali_number = token.text.chars()
                        .map(|c| match c {
                            '0' => '০',
                            '1' => '১',
                            '2' => '২',
                            '3' => '৩',
                            '4' => '৪',
                            '5' => '৫',
                            '6' => '৬',
                            '7' => '৭',
                            '8' => '৮',
                            '9' => '৯',
                            _ => c,
                        })
                        .collect::<String>();
                    
                    phonemes.push(Phoneme {
                        roman: token.text.clone(),
                        bengali: bengali_number,
                        phoneme_type: PhonemeType::Number,
                        position: None,
                    });
                },
                TokenType::Other => {
                    phonemes.push(Phoneme {
                        roman: token.text.clone(),
                        bengali: token.text.clone(),
                        phoneme_type: PhonemeType::Other,
                        position: None,
                    });
                },
            }
        }
        
        phonemes
    }
    
    /// Organize phonemes into syllables
    pub fn organize_into_syllables(&self, phonemes: &[Phoneme]) -> Vec<Syllable> {
        let mut syllables = Vec::new();
        let mut current_syllable = Syllable::new();
        let mut i = 0;
        
        while i < phonemes.len() {
            let phoneme = &phonemes[i];
            
            match phoneme.phoneme_type {
                PhonemeType::Consonant => {
                    // If we already have a vowel in this syllable, start a new one
                    if current_syllable.has_vowel() {
                        syllables.push(current_syllable);
                        current_syllable = Syllable::new();
                    }
                    
                    current_syllable.add_consonant(phoneme.clone());
                },
                PhonemeType::Vowel => {
                    // If we already have a vowel in this syllable, start a new one
                    if current_syllable.has_vowel() {
                        syllables.push(current_syllable);
                        current_syllable = Syllable::new();
                    }
                    
                    current_syllable.set_vowel(phoneme.clone());
                    
                    // Check if this vowel is standalone
                    current_syllable.set_standalone(!current_syllable.has_consonants());
                },
                PhonemeType::Modifier => {
                    // Add modifier to current syllable
                    current_syllable.add_modifier(phoneme.clone());
                },
                PhonemeType::Whitespace | PhonemeType::Punctuation | 
                PhonemeType::Number | PhonemeType::Other => {
                    // Push current syllable if not empty
                    if !current_syllable.is_empty() {
                        syllables.push(current_syllable);
                        current_syllable = Syllable::new();
                    }
                    
                    // Create a special syllable for this phoneme
                    let mut special_syllable = Syllable::new();
                    special_syllable.add_special(phoneme.clone());
                    syllables.push(special_syllable);
                },
            }
            
            i += 1;
        }
        
        // Add the last syllable if not empty
        if !current_syllable.is_empty() {
            syllables.push(current_syllable);
        }
        
        // Apply syllable-level rules
        self.apply_syllable_rules(&mut syllables);
        
        syllables
    }
    
    /// Apply syllable-level phonological rules
    fn apply_syllable_rules(&self, syllables: &mut [Syllable]) {
        // Handle inherent vowels (consonant with no vowel gets 'অ')
        if self.handle_inherent_vowels {
            for syllable in syllables.iter_mut() {
                if syllable.has_consonants() && !syllable.has_vowel() && !syllable.has_hasanta() {
                    // Add inherent vowel 'অ'
                    syllable.set_vowel(Phoneme {
                        roman: "a".to_string(),
                        bengali: "".to_string(),  // Empty for inherent vowel
                        phoneme_type: PhonemeType::Vowel,
                        position: None,
                    });
                }
            }
        }
        
        // Handle reph (র্)
        if self.handle_reph {
            for i in 0..syllables.len() {
                if i + 1 < syllables.len() {
                    let has_reph = syllables[i].has_consonant_sequence("র", "্");
                    
                    if has_reph {
                        // Mark this syllable as having reph
                        syllables[i].set_has_reph(true);
                        
                        // Remove the 'র্' from this syllable
                        syllables[i].remove_consonant_sequence("র", "্");
                        
                        // Add reph to next syllable
                        syllables[i + 1].set_has_preceding_reph(true);
                    }
                }
            }
        }
        
        // Handle ya-phala (্য)
        if self.handle_ya_phala {
            for syllable in syllables.iter_mut() {
                if syllable.has_consonant_sequence("্", "য") {
                    // Mark this syllable as having ya-phala
                    syllable.set_has_ya_phala(true);
                    
                    // Remove the '্য' sequence
                    syllable.remove_consonant_sequence("্", "য");
                }
            }
        }
    }
    
    /// Format syllables into proper Bengali text
    pub fn format_syllables(&self, syllables: &[Syllable]) -> String {
        let mut result = String::new();
        
        for syllable in syllables {
            result.push_str(&self.format_syllable(syllable));
        }
        
        result
    }
    
    /// Format a single syllable into Bengali text
    fn format_syllable(&self, syllable: &Syllable) -> String {
        if syllable.is_special() {
            // Special syllable (whitespace, punctuation, etc.)
            return syllable.get_special_text();
        }
        
        let mut result = String::new();
        
        // Handle consonants
        if syllable.has_consonants() {
            // Check for special conjuncts
            let consonant_text = syllable.get_consonant_text();
            
            if let Some(special_form) = SPECIAL_CONJUNCTS.get(consonant_text.as_str()) {
                result.push_str(special_form);
            } else {
                result.push_str(&consonant_text);
            }
        }
        
        // Handle vowel
        if syllable.has_vowel() {
            if syllable.is_standalone() || !syllable.has_consonants() {
                // Standalone vowel (use independent form)
                result.push_str(&syllable.get_vowel_text());
            } else if syllable.get_vowel_roman() != "a" || !self.handle_inherent_vowels {
                // Non-inherent vowel or inherent vowels disabled (use dependent form)
                result.push_str(&syllable.get_vowel_text());
            }
            // Inherent vowel ('অ') is implicit after consonant
        }
        
        // Handle modifiers
        if syllable.has_modifiers() {
            result.push_str(&syllable.get_modifier_text());
        }
        
        // Handle reph (র্)
        if syllable.has_preceding_reph() {
            // Insert reph at appropriate position
            // In Bengali, reph is rendered before the consonant
            result = format!("র্{}", result);
        }
        
        // Handle ya-phala (্য)
        if syllable.has_ya_phala() {
            // Append ya-phala after the consonant
            result.push_str("্য");
        }
        
        result
    }
}

impl Default for PhonologyEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::tokenizer::Tokenizer;
    
    #[test]
    fn test_convert_simple_word() {
        let tokenizer = Tokenizer::new();
        let phonology = PhonologyEngine::new();
        
        let tokens = tokenizer.tokenize("ami");
        let phonemes = phonology.tokens_to_phonemes(&tokens);
        let syllables = phonology.organize_into_syllables(&phonemes);
        let result = phonology.format_syllables(&syllables);
        
        assert_eq!(result, "আমি");
    }
    
    #[test]
    fn test_inherent_vowel() {
        let tokenizer = Tokenizer::new();
        let phonology = PhonologyEngine::new();
        
        let tokens = tokenizer.tokenize("km");
        let phonemes = phonology.tokens_to_phonemes(&tokens);
        let syllables = phonology.organize_into_syllables(&phonemes);
        let result = phonology.format_syllables(&syllables);
        
        assert_eq!(result, "কম");  // 'অ' is implicit after 'ক'
    }
    
    #[test]
    fn test_reph_formation() {
        let tokenizer = Tokenizer::new();
        let phonology = PhonologyEngine::new();
        
        let tokens = tokenizer.tokenize("karma");
        let phonemes = phonology.tokens_to_phonemes(&tokens);
        let syllables = phonology.organize_into_syllables(&phonemes);
        let result = phonology.format_syllables(&syllables);
        
        assert_eq!(result, "কর্ম");  // 'র্' appears before 'ম'
    }
}