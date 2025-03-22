use std::env;
use std::io::{self, Read};
use serde_json::json;
use obadh_engine::{Sanitizer, Tokenizer, TokenType, PhoneticUnitType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    
    // Check for help flag
    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        print_help();
        return Ok(());
    }
    
    // Check for debug flag to print vowel patterns
    if args.len() > 1 && args[1] == "--debug-vowels" {
        let tokenizer = Tokenizer::new();
        debug_vowel_patterns(&tokenizer);
        return Ok(());
    }

    // Debug direct word tokenization
    if args.len() > 2 && args[1] == "--debug-word" {
        let word = &args[2];
        let tokenizer = Tokenizer::new();
        debug_word_tokenization(&tokenizer, word);
        return Ok(());
    }
    
    // Special tokenization that preserves ",," sequences
    if args.len() > 2 && args[1] == "--special-tokenize" {
        let text = &args[2];
        let tokenizer = Tokenizer::new();
        debug_special_tokenization(&tokenizer, text);
        return Ok(());
    }
    
    // Read input from stdin or command line argument
    let input = if args.len() > 1 {
        // Use the command line argument as input
        args[1].clone()
    } else {
        // Check if stdin has data
        let mut buffer = String::new();
        let bytes_read = io::stdin().read_to_string(&mut buffer)?;
        
        if bytes_read == 0 {
            // No input provided, show help
            print_help();
            return Ok(());
        }
        
        buffer
    };
    
    // Create sanitizer and tokenizer
    let sanitizer = Sanitizer::new();
    let tokenizer = Tokenizer::new();
    
    // Sanitize the input using clean which preserves valid characters
    // and removes invalid ones, rather than failing on invalid characters
    let sanitized = sanitizer.clean(&input);
    
    // Tokenize the sanitized text
    let tokens = tokenizer.tokenize_text(&sanitized);
    
    // Convert tokens to a JSON-friendly format
    let json_tokens = tokens.iter().map(|token| {
        json!({
            "content": token.content,
            "type": format!("{:?}", token.token_type),
            "position": token.position
        })
    }).collect::<Vec<_>>();
    
    // For each word token, also include phonetic units
    let detailed_tokens = tokens.iter().map(|token| {
        let mut token_json = json!({
            "content": token.content,
            "type": format!("{:?}", token.token_type),
            "position": token.position
        });
        
        // If it's a word, include phonetic analysis
        if token.token_type == TokenType::Word {
            let phonetic_units = tokenizer.tokenize_word(&token.content);
            let units_json = phonetic_units.iter().map(|unit| {
                json!({
                    "text": unit.text,
                    "type": format!("{:?}", unit.unit_type),
                    "position": unit.position
                })
            }).collect::<Vec<_>>();
            
            if let serde_json::Value::Object(ref mut map) = token_json {
                map.insert("phonetic_units".to_string(), json!(units_json));
            }
        }
        
        token_json
    }).collect::<Vec<_>>();
    
    // Create the final output JSON
    let output = json!({
        "original": input,
        "sanitized": sanitized,
        "tokens": json_tokens,
        "detailed_tokens": detailed_tokens
    });
    
    // Pretty print the JSON
    println!("{}", serde_json::to_string_pretty(&output)?);
    
    Ok(())
}

// Debug function to directly test word tokenization
fn debug_word_tokenization(tokenizer: &Tokenizer, word: &str) {
    println!("Direct word tokenization for '{}':", word);
    let units = tokenizer.tokenize_word(word);
    
    println!("Phonetic units:");
    for (i, unit) in units.iter().enumerate() {
        println!("  {}: {:?} - '{}'", i, unit.unit_type, unit.text);
    }
    
    // Check if it's a conjunct without using to_string()
    if units.len() == 1 {
        match units[0].unit_type {
            PhoneticUnitType::Conjunct => {
                println!("\nDetected as a conjunct: '{}'", units[0].text);
            },
            _ => {}
        }
    }
}

// Debug function to print vowel patterns loaded by the tokenizer
fn debug_vowel_patterns(tokenizer: &Tokenizer) {
    // This requires exposing the vowel_patterns field in the Tokenizer struct
    println!("Vowel patterns loaded from definitions:");
    // Either modify the Tokenizer to expose its patterns or access them via a method
    
    // For now, use the public API to detect if specific patterns are recognized
    let test_vowels = ["a", "A", "i", "I", "u", "U", "e", "E", "o", "O"];
    for vowel in test_vowels {
        let units = tokenizer.tokenize_word(vowel);
        if !units.is_empty() {
            println!("  '{}' - detected as: {:?}", vowel, units[0].unit_type);
        } else {
            println!("  '{}' - not recognized", vowel);
        }
    }
}

// For testing purposes, add a special function that preserves commas in k,,k pattern
fn debug_special_tokenization(tokenizer: &Tokenizer, text: &str) {
    println!("Special tokenization for '{}':", text);
    
    // First perform a basic split by whitespace
    let words: Vec<&str> = text.split_whitespace().collect();
    
    println!("Words after splitting by whitespace:");
    for (i, word) in words.iter().enumerate() {
        println!("  {}: '{}'", i, word);
        
        // Now tokenize each word phonetically
        let units = tokenizer.tokenize_word(word);
        println!("  Phonetic units:");
        for (j, unit) in units.iter().enumerate() {
            println!("    {}: {:?} - '{}'", j, unit.unit_type, unit.text);
        }
    }
}

fn print_help() {
    println!("Obadh Engine Tokenizer");
    println!("======================");
    println!("A tool to tokenize text for Bengali transliteration.");
    println!();
    println!("Usage:");
    println!("  tokenizer [TEXT]         Tokenize the provided text");
    println!("  cat file.txt | tokenizer Tokenize text from stdin");
    println!("  tokenizer --help         Show this help message");
    println!("  tokenizer --debug-vowels Print vowel patterns loaded from definitions");
    println!("  tokenizer --debug-word WORD Directly tokenize a single word");
    println!("  tokenizer --special-tokenize TEXT Special tokenize a text");
    println!();
    println!("Output:");
    println!("  JSON data containing:");
    println!("  - Original input text");
    println!("  - Sanitized text");
    println!("  - Basic tokens (words, whitespace, punctuation, etc.)");
    println!("  - Detailed tokens with phonetic unit analysis for words");
} 