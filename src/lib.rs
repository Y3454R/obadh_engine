//! # Obadh Engine
//! 
//! A high-performance, linguistically accurate Roman to Bengali transliteration engine.
//! 
//! The engine uses a phonological approach based on Bengali linguistic principles
//! rather than exhaustive mappings of character combinations.

// Define modules
pub mod engine;
pub mod linguistic;
pub mod wasm;

// Re-export main structs for easier usage
pub use engine::transliterator::Transliterator;

use serde_json::json;
use std::io::{BufRead, Write, Result as IoResult};

/// Controls the amount of detail provided in the output
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerbosityLevel {
    /// Only show the final output
    Quiet,
    /// Show basic info (default)
    Normal,
    /// Show detailed analysis
    Detailed,
    /// Show full debugging information
    Debug
}

/// Output format for the transliteration results
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// Plain text output (default)
    Text,
    /// JSON formatted output
    Json,
    /// XML formatted output
    Xml,
    /// HTML formatted output
    Html,
    /// Markdown formatted output
    Markdown
}

/// The main entry point for the Obadh transliteration engine.
/// 
/// This struct provides a convenient API for the transliteration process.
pub struct ObadhEngine {
    transliterator: engine::transliterator::Transliterator,
    debug_mode: bool,
    verbosity: VerbosityLevel,
    output_format: OutputFormat,
}

impl ObadhEngine {
    /// Create a new instance of the Obadh Engine
    pub fn new() -> Self {
        ObadhEngine {
            transliterator: engine::transliterator::Transliterator::new(),
            debug_mode: false,
            verbosity: VerbosityLevel::Normal,
            output_format: OutputFormat::Text,
        }
    }
    
    /// Enable or disable debug mode
    pub fn with_debug_mode(mut self, enable: bool) -> Self {
        self.debug_mode = enable;
        self
    }
    
    /// Set the verbosity level
    pub fn with_verbosity(mut self, level: VerbosityLevel) -> Self {
        self.verbosity = level;
        self
    }
    
    /// Set the output format
    pub fn with_output_format(mut self, format: OutputFormat) -> Self {
        self.output_format = format;
        self
    }
    
    /// Check if debug mode is enabled
    pub fn is_debug_mode(&self) -> bool {
        self.debug_mode
    }
    
    /// Transliterate Roman text to Bengali
    /// 
    /// # Arguments
    /// 
    /// * `text` - The Roman text to transliterate
    /// 
    /// # Returns
    /// 
    /// The transliterated Bengali text
    pub fn transliterate(&self, text: &str) -> String {
        let result = self.transliterator.transliterate(text);
        
        // Output debug information if debug mode is enabled
        if self.debug_mode {
            self.debug_tokenization(text);
        }
        
        result
    }
    
    /// Transliterate input text and return it in the configured format
    pub fn transliterate_as(&self, text: &str) -> String {
        match self.output_format {
            OutputFormat::Text => self.transliterate(text),
            OutputFormat::Json => self.transliterate_json(text),
            OutputFormat::Xml => {
                let result = self.transliterate(text);
                format!("<result><input>{}</input><output>{}</output></result>", text, result)
            },
            OutputFormat::Html => {
                let result = self.transliterate(text);
                format!("<div class=\"transliteration\"><div class=\"input\">{}</div><div class=\"output\">{}</div></div>", text, result)
            },
            OutputFormat::Markdown => {
                let result = self.transliterate(text);
                format!("**Input:** {}\n\n**Output:** {}", text, result)
            },
        }
    }
    
    /// Process a batch of texts for transliteration
    pub fn batch_transliterate(&self, texts: &[&str]) -> Vec<String> {
        texts.iter()
             .map(|&text| self.transliterate(text))
             .collect()
    }
    
    /// Process a batch of texts for transliteration with the configured format
    pub fn batch_transliterate_as(&self, texts: &[&str]) -> Vec<String> {
        texts.iter()
             .map(|&text| self.transliterate_as(text))
             .collect()
    }
    
    /// Process text from a reader and write results to a writer
    pub fn transliterate_stream<R, W>(&self, reader: &mut R, writer: &mut W) -> IoResult<()> 
    where 
        R: BufRead,
        W: Write
    {
        for line in reader.lines() {
            let line = line?;
            let result = self.transliterate(&line);
            writeln!(writer, "{}", result)?;
        }
        Ok(())
    }
    
    /// Print debug information about the tokenization of text
    /// 
    /// This is useful for debugging tokenization issues
    /// 
    /// # Arguments
    /// 
    /// * `text` - The Roman text to analyze
    pub fn debug_tokenization(&self, text: &str) {
        let analysis = self.analyze(text);
        
        println!("Input: {}", text);
        
        println!("Tokens:");
        for (i, token) in analysis.tokens.iter().enumerate() {
            println!("  {} - {:?} - {:?}", i, token.text, token.token_type);
        }
        
        println!("Phonemes:");
        for (i, phoneme) in analysis.phonemes.iter().enumerate() {
            println!("  {} - {} - {:?} - has_ya_phala: {}", 
                     i, phoneme.bengali, phoneme.phoneme_type, phoneme.has_ya_phala);
        }
        
        println!("Syllables:");
        for (i, syllable) in analysis.syllables.iter().enumerate() {
            println!("  {} - Consonants: {} - Vowel: {} - Has Ya-phala: {}", 
                     i, syllable.get_consonant_text(), 
                     syllable.get_vowel_text(), syllable.has_ya_phala());
        }
        
        println!("Output: {}", analysis.output);
    }
    
    /// Transliterate Roman text to Bengali and return JSON output
    /// 
    /// This is useful for testing, debugging, and integration with other systems
    /// 
    /// # Arguments
    /// 
    /// * `text` - The Roman text to transliterate
    /// 
    /// # Returns
    /// 
    /// A JSON string with the input, output, and detailed analysis information
    pub fn transliterate_json(&self, text: &str) -> String {
        let analysis = self.analyze(text);
        
        // Create a detailed JSON structure with all available information
        let json_result = match self.verbosity {
            VerbosityLevel::Quiet => {
                json!({
                    "output": analysis.output,
                })
            },
            VerbosityLevel::Normal => {
                json!({
                    "input": text,
                    "output": analysis.output,
                })
            },
            VerbosityLevel::Detailed => {
                json!({
                    "input": text,
                    "output": analysis.output,
                    "tokens": analysis.tokens.iter().map(|t| {
                        json!({
                            "text": t.text,
                            "type": format!("{:?}", t.token_type),
                            "position": t.position.as_ref().map(|p| format!("{:?}", p))
                        })
                    }).collect::<Vec<_>>()
                })
            },
            VerbosityLevel::Debug => {
                json!({
                    "input": text,
                    "output": analysis.output,
                    "tokens": analysis.tokens.iter().map(|t| {
                        json!({
                            "text": t.text,
                            "type": format!("{:?}", t.token_type),
                            "position": t.position.as_ref().map(|p| format!("{:?}", p))
                        })
                    }).collect::<Vec<_>>(),
                    "phonemes": analysis.phonemes.iter().map(|p| {
                        json!({
                            "bengali": p.bengali,
                            "type": format!("{:?}", p.phoneme_type),
                            "is_reph": p.is_reph,
                            "has_ya_phala": p.has_ya_phala,
                            "vowel": p.vowel
                        })
                    }).collect::<Vec<_>>(),
                    "syllables": analysis.syllables.iter().map(|s| {
                        json!({
                            "consonants": s.get_consonant_text(),
                            "vowel": s.get_vowel_text(),
                            "has_ya_phala": s.has_ya_phala(),
                            "has_reph": s.has_reph()
                        })
                    }).collect::<Vec<_>>()
                })
            }
        };
        
        serde_json::to_string_pretty(&json_result).unwrap_or_else(|_| 
            format!("{{\"error\":\"Failed to serialize JSON output\",\"input\":\"{}\",\"output\":\"{}\"}}", 
                    text, analysis.output)
        )
    }
    
    /// Get detailed analysis of the transliteration process
    /// 
    /// This is useful for debugging or understanding the steps
    /// in the transliteration process.
    /// 
    /// # Arguments
    /// 
    /// * `text` - The Roman text to analyze
    /// 
    /// # Returns
    /// 
    /// A detailed breakdown of the transliteration process
    pub fn analyze(&self, text: &str) -> engine::transliterator::TransliterationAnalysis {
        self.transliterator.analyze(text)
    }
    
    /// Get analysis with formatted diagnostic information based on verbosity level
    pub fn analyze_with_details(&self, text: &str) -> (String, String) {
        let analysis = self.analyze(text);
        let result = self.transliterate(text);
        
        let debug_info = match self.verbosity {
            VerbosityLevel::Quiet => String::new(),
            VerbosityLevel::Normal => format!("Input: {}, Output: {}", text, result),
            VerbosityLevel::Detailed => {
                let mut info = format!("Input: {}\nOutput: {}\n\n", text, result);
                
                info.push_str("Tokens:\n");
                for (i, token) in analysis.tokens.iter().enumerate() {
                    info.push_str(&format!("  {} - {:?} - {:?}\n", 
                                         i, token.text, token.token_type));
                }
                
                info
            },
            VerbosityLevel::Debug => {
                let mut info = format!("Input: {}\nOutput: {}\n\n", text, result);
                
                info.push_str("Tokens:\n");
                for (i, token) in analysis.tokens.iter().enumerate() {
                    info.push_str(&format!("  {} - {:?} - {:?} - {:?}\n", 
                                         i, token.text, token.token_type, token.position));
                }
                
                info.push_str("\nPhonemes:\n");
                for (i, phoneme) in analysis.phonemes.iter().enumerate() {
                    info.push_str(&format!("  {} - {} - {:?} - has_ya_phala: {} - vowel: {:?}\n", 
                                         i, phoneme.bengali, phoneme.phoneme_type, 
                                         phoneme.has_ya_phala, phoneme.vowel));
                }
                
                info.push_str("\nSyllables:\n");
                for (i, syllable) in analysis.syllables.iter().enumerate() {
                    info.push_str(&format!("  {} - Consonants: {} - Vowel: {} - Has Ya-phala: {}\n", 
                                         i, syllable.get_consonant_text(), 
                                         syllable.get_vowel_text(), syllable.has_ya_phala()));
                }
                
                info
            }
        };
        
        (result, debug_info)
    }
}

impl Default for ObadhEngine {
    fn default() -> Self {
        Self::new()
    }
}