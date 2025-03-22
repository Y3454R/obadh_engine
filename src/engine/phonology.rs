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
        map.insert("Y", "য");  // Ya-phala - changed to proper character (য) from য্
        map.insert("R", "ড়");  // Bengali ড়
        map.insert("Rh", "ঢ়"); // Bengali ঢ়
        
        map
    };
    
    /// Mapping of Roman vowels to Bengali phonemes (independent form)
    static ref VOWEL_MAP_INDEPENDENT: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        
        // Basic vowels as per the documentation
        map.insert("o", "অ");     // অ-কার (a-kar)
        map.insert("A", "আ");     // আ-কার (aa-kar)
        map.insert("i", "ই");     // ই-কার (i-kar)
        map.insert("I", "ঈ");     // ঈ-কার (dirgho i-kar)
        map.insert("u", "উ");     // উ-কার (u-kar)
        map.insert("U", "ঊ");     // ঊ-কার (dirgho u-kar)
        map.insert("e", "এ");     // এ-কার (e-kar)
        map.insert("OI", "ঐ");    // ঐ-কার (oi-kar)
        map.insert("O", "ও");     // ও-কার (o-kar)
        map.insert("OU", "ঔ");    // ঔ-কার (ou-kar)
        map.insert("rri", "ঋ");   // ঋ-কার (ri-kar)
        
        // Common alternative spellings for backward compatibility
        map.insert("a", "আ");     // Equivalent to 'A'
        map.insert("aa", "আ");    // Equivalent to 'A'
        map.insert("oi", "ঐ");    // Equivalent to 'OI'
        map.insert("ou", "ঔ");    // Equivalent to 'OU'
        
        // Special vowel combinations
        map.insert("ai", "আই");   // a + i combination
        map.insert("au", "আউ");   // a + u combination
        map.insert("ae", "আএ");   // a + e combination
        map.insert("ao", "আও");   // a + o combination
        map.insert("ia", "ইয়া");  // i + a combination
        map.insert("io", "ইও");   // i + o combination
        map.insert("eo", "এও");   // e + o combination
        
        map
    };
    
    /// Mapping of Roman vowels to Bengali phonemes (dependent form)
    static ref VOWEL_MAP_DEPENDENT: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        
        // Basic vowels as per the documentation
        map.insert("o", "");      // Inherent vowel (no visible kar)
        map.insert("A", "া");     // আ-কার (aa-kar)
        map.insert("i", "ি");     // ই-কার (i-kar)
        map.insert("I", "ী");     // ঈ-কার (dirgho i-kar)
        map.insert("u", "ু");     // উ-কার (u-kar)
        map.insert("U", "ূ");     // ঊ-কার (dirgho u-kar)
        map.insert("e", "ে");     // এ-কার (e-kar)
        map.insert("OI", "ৈ");    // ঐ-কার (oi-kar)
        map.insert("O", "ো");     // ও-কার (o-kar)
        map.insert("OU", "ৌ");    // ঔ-কার (ou-kar)
        map.insert("rri", "ৃ");   // ঋ-কার (ri-kar)
        
        // Common alternative spellings for backward compatibility
        map.insert("a", "া");     // Equivalent to 'A'
        map.insert("aa", "া");    // Equivalent to 'A'
        map.insert("oi", "ৈ");    // Equivalent to 'OI'
        map.insert("ou", "ৌ");    // Equivalent to 'OU'
        
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
    
    /// Mapping of numbers to Bengali numerals
    static ref NUMBER_MAP: HashMap<char, &'static str> = {
        let mut map = HashMap::new();
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
    
    /// Mapping of punctuation to Bengali equivalents
    static ref PUNCTUATION_MAP: HashMap<char, &'static str> = {
        let mut map = HashMap::new();
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
        map.insert('-', "-");
        map.insert('_', "_");
        map.insert('"', "\"");
        map.insert('\'', "'");
        map
    };
}

/// Engine for Bengali phonology rules
pub struct PhonologyEngine {
    /// Whether to handle inherent vowels (default true)
    handle_inherent_vowels: bool,
    /// Whether to handle reph formation (default true)
    handle_reph: bool,
    /// Whether to handle ya-phala (default true)
    handle_ya_phala: bool,
    /// Whether to handle bo-fola (default true)
    handle_bo_fola: bool,
}

impl PhonologyEngine {
    /// Create a new phonology engine with default settings
    pub fn new() -> Self {
        PhonologyEngine {
            handle_inherent_vowels: true,
            handle_reph: true,
            handle_ya_phala: true,
            handle_bo_fola: true,
        }
    }
    
    /// Configure inherent vowel handling
    pub fn with_inherent_vowels(mut self, enable: bool) -> Self {
        self.handle_inherent_vowels = enable;
        self
    }
    
    /// Configure reph handling
    pub fn with_reph(mut self, enable: bool) -> Self {
        self.handle_reph = enable;
        self
    }
    
    /// Configure ya-phala handling
    pub fn with_ya_phala(mut self, enable: bool) -> Self {
        self.handle_ya_phala = enable;
        self
    }
    
    /// Configure bo-fola handling
    pub fn with_bo_fola(mut self, enable: bool) -> Self {
        self.handle_bo_fola = enable;
        self
    }
    
    /// Convert tokens to phonemes
    pub fn tokens_to_phonemes(&self, tokens: &[Token]) -> Vec<Phoneme> {
        let mut phonemes: Vec<Phoneme> = Vec::new();
        let mut i = 0;
        
        while i < tokens.len() {
            let token = &tokens[i];
            
            // Check for special cases - like 'r' followed by a conjunct
            let is_reph = token.text == "r" && 
                          i + 2 < tokens.len() && 
                          tokens[i+1].token_type == TokenType::Consonant && 
                          (i + 2 >= tokens.len() || tokens[i+2].token_type != TokenType::Vowel);
            
            // Check if this is ya-phala - 'y' after a consonant
            let _is_ya_phala = token.text == "Y";  // Special token for ya-phala
            
            match token.token_type {
                TokenType::Consonant => {
                    // Handle reph formation - র্ - (r + hasanta)
                    if is_reph {
                        // Create the reph phoneme with hasanta
                        let mut phoneme = Phoneme::new(
                            "র".to_string(),
                            PhonemeType::Consonant,
                            token.position.clone()
                        );
                        phoneme.vowel = Some("্".to_string());
                        phoneme.is_reph = true; // Mark this as a reph for correct rendering
                        phonemes.push(phoneme);
                        
                        i += 1; // Move past the 'r'
                        continue;
                    }
                    
                    // Detect ya-phala: When 'y' follows a consonant and is not at word-initial position
                    let is_ya_phala = token.text == "y" && 
                                      i > 0 && 
                                      tokens[i-1].token_type == TokenType::Consonant &&
                                      token.position != Some(TokenPosition::Initial);
                    
                    // Handle ya-phala
                    if is_ya_phala {
                        if let Some(last) = phonemes.last_mut() {
                            if last.phoneme_type == PhonemeType::Consonant {
                                last.has_ya_phala = true;
                                i += 1;
                                continue;
                            }
                        }
                    }
                    
                    // Check if next token is a vowel
                    let next_vowel = if i + 1 < tokens.len() && tokens[i + 1].token_type == TokenType::Vowel {
                        Some(&tokens[i + 1])
                    } else {
                        None
                    };
                    
                    // Regular consonant handling
                    if let Some(bengali) = CONSONANT_MAP.get(token.text.as_str()) {
                        let vowel_str = next_vowel.and_then(|v| VOWEL_MAP_DEPENDENT.get(v.text.as_str()).map(|s| s.to_string()));
                        
                        // Create consonant phoneme
                        let mut phoneme = Phoneme::new(
                            bengali.to_string(),
                            PhonemeType::Consonant,
                            token.position.clone()
                        );
                        
                        // Handle special case for 'o' following a consonant (compound stopper)
                        let is_compound_stopper = next_vowel.map_or(false, |v| v.text == "o");
                        
                        // Add vowel if present
                        phoneme.vowel = vowel_str;
                        
                        // Avro rule: If this is followed by lowercase 'o', it acts as a compound stopper
                        // We don't add a visible vowel mark, but we ensure it doesn't form a conjunct
                        if is_compound_stopper {
                            phoneme.is_conjunct_former = false;
                        }
                        
                        // For final position consonants without explicit vowels,
                        // add appropriate handling based on phonological rules
                        if phoneme.vowel.is_none() && !is_compound_stopper && token.position == Some(TokenPosition::Final) {
                            // In Bengali, a final consonant without a vowel mark gets an implicit hasanta (্)
                            phoneme.vowel = Some("্".to_string());
                        }
                        
                        phonemes.push(phoneme);
                        
                        // Skip the vowel if it was used with this consonant
                        if next_vowel.is_some() {
                            i += 1;
                        }
                    }
                },
                TokenType::Vowel => {
                    // Check position to handle vowels correctly
                    let is_independent = i == 0 || 
                                         i > 0 && tokens[i-1].token_type != TokenType::Consonant ||
                                         token.position == Some(TokenPosition::Initial);
                    
                    if is_independent {
                        // Independent vowel form
                        if let Some(bengali) = VOWEL_MAP_INDEPENDENT.get(token.text.as_str()) {
                            let phoneme = Phoneme::new(
                                bengali.to_string(),
                                PhonemeType::Vowel,
                                token.position.clone()
                            );
                            phonemes.push(phoneme);
                        }
                    } else {
                        // Special case for 'o' as a compound stopper or when followed by another vowel
                        if token.text == "o" && i > 0 && tokens[i-1].token_type == TokenType::Consonant {
                            // Check if 'o' is followed by another vowel (like in "boi")
                            let has_following_vowel = i + 1 < tokens.len() && tokens[i + 1].token_type == TokenType::Vowel;
                            
                            if has_following_vowel {
                                // For "boi" case, we need to ensure we add both:
                                // 1. The vowel "ই" as an independent form
                                // 2. Also need to ensure "অ" is present to create the correct 'ই' form
                                
                                // First add the inherent vowel "অ"
                                let inherent_phoneme = Phoneme::new(
                                    "অ".to_string(),
                                    PhonemeType::Vowel,
                                    Some(TokenPosition::Medial)
                                );
                                phonemes.push(inherent_phoneme);
                            }
                            // Note: we don't need an else case here since 'o' as a compound stopper
                            // has already been handled in the consonant case
                        }
                    }
                },
                TokenType::Whitespace => {
                    let phoneme = Phoneme::new(
                        token.text.clone(),
                        PhonemeType::Whitespace,
                        None
                    );
                    phonemes.push(phoneme);
                },
                TokenType::Punctuation => {
                    if let Some(bengali) = PUNCTUATION_MAP.get(&token.text.chars().next().unwrap_or(' ')) {
                        let phoneme = Phoneme::new(
                            bengali.to_string(),
                            PhonemeType::Punctuation,
                            None
                        );
                        phonemes.push(phoneme);
                    } else {
                        let phoneme = Phoneme::new(
                            token.text.clone(),
                            PhonemeType::Punctuation,
                            None
                        );
                        phonemes.push(phoneme);
                    }
                },
                TokenType::Number => {
                    // Convert to Bengali numerals
                    let bengali_number: String = token.text.chars()
                        .map(|c| NUMBER_MAP.get(&c).map_or(c.to_string(), |s| s.to_string()))
                        .collect();
                    
                    let phoneme = Phoneme::new(
                        bengali_number,
                        PhonemeType::Number,
                        None
                    );
                    phonemes.push(phoneme);
                },
                _ => {
                    // Other token types
                    let phoneme = Phoneme::new(
                        token.text.clone(),
                        PhonemeType::Other,
                        None
                    );
                    phonemes.push(phoneme);
                }
            }
            
            i += 1;
        }
        
        phonemes
    }
    
    /// Organize phonemes into syllables
    pub fn organize_into_syllables(&self, phonemes: &[Phoneme]) -> Vec<Syllable> {
        let mut syllables: Vec<Syllable> = Vec::new();
        let mut current_syllable = Syllable::new();
        let mut i = 0;
        
        // First pass - identify reph and save it for later placement
        let mut reph_phoneme: Option<Phoneme> = None;
        
        while i < phonemes.len() {
            let phoneme = &phonemes[i];
            
            // If this is a reph (র + ্), save it for placement at the end of syllable
            if phoneme.phoneme_type == PhonemeType::Consonant && 
               phoneme.bengali == "র" && 
               phoneme.vowel.as_ref().map_or(false, |v| v == "্") &&
               phoneme.is_reph {
                reph_phoneme = Some(phoneme.clone());
                i += 1;
                continue;
            }
            
            match phoneme.phoneme_type {
                PhonemeType::Consonant => {
                    if current_syllable.has_consonants() && !current_syllable.has_vowel() {
                        // Start a new syllable
                        syllables.push(current_syllable);
                        current_syllable = Syllable::new();
                    }
                    
                    // Add the consonant
                    let consonant_phoneme = phoneme.clone();
                    
                    // If we have a saved reph and this is a new syllable with no consonants yet,
                    // mark this syllable as having a preceding reph
                    if reph_phoneme.is_some() && !current_syllable.has_consonants() {
                        current_syllable.set_has_preceding_reph(true);
                        reph_phoneme = None;
                    }
                    
                    current_syllable.add_consonant(consonant_phoneme);
                },
                PhonemeType::Vowel => {
                    if current_syllable.has_consonants() {
                        current_syllable.set_vowel(phoneme.clone());
                    } else {
                        // Standalone vowel
                        let mut vowel_syllable = Syllable::new();
                        vowel_syllable.set_vowel(phoneme.clone());
                        syllables.push(vowel_syllable);
                        current_syllable = Syllable::new();
                    }
                },
                PhonemeType::Modifier => {
                    current_syllable.add_modifier(phoneme.clone());
                },
                _ => {
                    // Create a special syllable for this phoneme
                    let mut special_syllable = Syllable::new();
                    special_syllable.add_special(phoneme.clone());
                    
                    syllables.push(special_syllable);
                    
                    // Start a new syllable
                    current_syllable = Syllable::new();
                }
            }
            
            i += 1;
        }
        
        // Add the final syllable if not empty
        if !current_syllable.is_empty() {
            syllables.push(current_syllable);
        }
        
        // Apply additional rules
        if self.handle_reph || self.handle_ya_phala || self.handle_bo_fola {
            let mut processed_syllables = syllables.clone();
            self.apply_syllable_rules(&mut processed_syllables);
            processed_syllables
        } else {
            syllables
        }
    }
    
    /// Apply syllable-level phonological rules
    fn apply_syllable_rules(&self, syllables: &mut Vec<Syllable>) {
        for syllable in syllables {
            // Handle inherent vowels only if there's no explicit vowel
            // and the consonants don't already have a vowel marker
            if syllable.has_consonants() && !syllable.has_vowel() && !syllable.has_conjunct_only() {
                // Check if any of the consonants in the syllable have an explicit "o" vowel
                let has_o_vowel = syllable.get_consonant_sequence().iter()
                    .any(|c| c.vowel.as_ref().map_or(false, |v| v == ""));
                
                // Only add the inherent vowel if there's no 'o' vowel
                if !has_o_vowel {
                    let vowel_phoneme = Phoneme::new(
                        "অ".to_string(),
                        PhonemeType::Vowel,
                        None
                    );
                    syllable.set_vowel(vowel_phoneme);
                }
            }
            
            // Handle reph
            if syllable.has_consonant_with_text("র") {
                let has_hasanta = syllable.has_consonant_with_vowel("্");
                if has_hasanta {
                    syllable.set_has_reph(true);
                }
            }
            
            // Handle ya-phala (jophola)
            let mut has_ya_phala = false;
            for consonant in &syllable.get_consonant_sequence() {
                if consonant.has_ya_phala {
                    has_ya_phala = true;
                    break;
                }
            }
            if has_ya_phala {
                syllable.set_has_ya_phala(true);
            }
            
            // Handle bo-fola
            let mut has_bo_fola = false;
            for consonant in &syllable.get_consonant_sequence() {
                if consonant.has_bo_fola {
                    has_bo_fola = true;
                    break;
                }
            }
            if has_bo_fola {
                syllable.set_has_bo_fola(true);
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
        let mut result = String::new();
        
        // Format consonants with conjuncts
        if syllable.has_consonants() {
            if syllable.has_conjunct_only() {
                result.push_str(&syllable.get_conjunct_text());
            } else {
                // Add consonants
                result.push_str(&syllable.get_consonant_text());
                
                // Add ya-phala if needed (apply before any other fola)
                if syllable.has_ya_phala() {
                    result.push_str("্য");
                }
                
                // Add bo-fola if needed
                if syllable.has_bo_fola() {
                    result.push_str("্ব");
                }
                
                // Check for vowels directly attached to consonants
                let consonants = syllable.get_consonant_sequence();
                if !consonants.is_empty() {
                    // Get the last consonant to apply vowel
                    if let Some(last_consonant) = consonants.last() {
                        if let Some(vowel_str) = &last_consonant.vowel {
                            // If the vowel is already in dependent form (kar)
                            if !vowel_str.is_empty() && vowel_str != "্" {
                                result.push_str(vowel_str);
                            }
                        }
                    }
                }
                
                // Add vowel diacritic from the syllable if present and not already handled
                if syllable.has_vowel() && 
                   !consonants.last().map_or(false, |c| c.vowel.is_some() && c.vowel.as_ref().unwrap() != "্" && !c.vowel.as_ref().unwrap().is_empty()) {
                    let vowel_text = syllable.get_vowel_text();
                    
                    // Convert vowel to diacritic form if needed
                    let diacritic = match vowel_text.as_str() {
                        "অ" => "", // Inherent vowel, no visible mark
                        "আ" => "া",
                        "ই" => "ি",
                        "ঈ" => "ী",
                        "উ" => "ু",
                        "ঊ" => "ূ",
                        "এ" => "ে",
                        "ঐ" => "ৈ",
                        "ও" => "ো",
                        "ঔ" => "ৌ",
                        "ঋ" => "ৃ",
                        _ => "",
                    };
                    
                    // Only add if not empty
                    if !diacritic.is_empty() {
                        result.push_str(diacritic);
                    }
                }
            }
        } else if syllable.has_vowel() {
            // Just a vowel, no consonants
            result.push_str(&syllable.get_vowel_text());
        }
        
        // Add reph at the end
        if syllable.has_reph() {
            result.push_str("র্");
        }
        
        result
    }
}

impl Default for PhonologyEngine {
    fn default() -> Self {
        Self::new()
    }
}