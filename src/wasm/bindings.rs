//! WebAssembly bindings for the Obadh Engine.
//!
//! This module exports the engine functionality to JavaScript.

use wasm_bindgen::prelude::*;
use crate::ObadhEngine;
use crate::engine::transliterator::TransliterationAnalysis;
use std::fmt;

// Import JavaScript functions we need
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind to `console.log`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_obj(obj: &JsValue);
}

// Custom macro for logging
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

/// WebAssembly wrapper for the Obadh Engine
#[wasm_bindgen]
pub struct WasmEngine {
    engine: ObadhEngine,
}

/// WebAssembly representation of transliteration steps
#[wasm_bindgen]
pub struct WasmAnalysis {
    input: String,
    tokens: String,
    phonemes: String,
    syllables: String,
    output: String,
}

#[wasm_bindgen]
impl WasmAnalysis {
    /// Get the original input text
    #[wasm_bindgen(getter)]
    pub fn input(&self) -> String {
        self.input.clone()
    }
    
    /// Get the tokens as a JSON string
    #[wasm_bindgen(getter)]
    pub fn tokens(&self) -> String {
        self.tokens.clone()
    }
    
    /// Get the phonemes as a JSON string
    #[wasm_bindgen(getter)]
    pub fn phonemes(&self) -> String {
        self.phonemes.clone()
    }
    
    /// Get the syllables as a JSON string
    #[wasm_bindgen(getter)]
    pub fn syllables(&self) -> String {
        self.syllables.clone()
    }
    
    /// Get the final output text
    #[wasm_bindgen(getter)]
    pub fn output(&self) -> String {
        self.output.clone()
    }
}

#[wasm_bindgen]
impl WasmEngine {
    /// Create a new instance of the WebAssembly engine
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // Set up better panic handling for WASM
        crate::wasm::set_panic_hook();
        
        WasmEngine {
            engine: ObadhEngine::new(),
        }
    }
    
    /// Transliterate Roman text to Bengali
    #[wasm_bindgen]
    pub fn transliterate(&self, text: &str) -> String {
        self.engine.transliterate(text)
    }
    
    /// Get a detailed analysis of the transliteration process
    #[wasm_bindgen]
    pub fn analyze(&self, text: &str) -> WasmAnalysis {
        let analysis = self.engine.analyze(text);
        
        // Convert to a WASM-friendly format
        WasmAnalysis {
            // input: analysis.input,
            // tokens: Self::tokens_to_json(&analysis),
            input: analysis.input.clone(),
            tokens: Self::tokens_to_json(&analysis),
            phonemes: Self::phonemes_to_json(&analysis),
            syllables: Self::syllables_to_json(&analysis),
            output: analysis.output,
        }
    }
    
    /// Convert tokens to a JSON string for JavaScript
    fn tokens_to_json(analysis: &TransliterationAnalysis) -> String {
        let tokens = analysis.tokens.iter().map(|token| {
            let token_type = format!("{:?}", token.token_type);
            let position = match &token.position {
                Some(pos) => format!("{:?}", pos),
                None => "null".to_string(),
            };
            
            format!(
                r#"{{"text":"{}", "type":"{}", "position":{}}}"#,
                token.text, token_type, 
                if position == "null" { position } else { format!(r#""{}""#, position) }
            )
        }).collect::<Vec<String>>().join(",");
        
        format!("[{}]", tokens)
    }
    
    /// Convert phonemes to a JSON string for JavaScript
    fn phonemes_to_json(analysis: &TransliterationAnalysis) -> String {
        let phonemes = analysis.phonemes.iter().map(|phoneme| {
            let phoneme_type = format!("{:?}", phoneme.phoneme_type);
            let position = match &phoneme.position {
                Some(pos) => format!("{:?}", pos),
                None => "null".to_string(),
            };
            
            format!(
                r#"{{"roman":"{}", "bengali":"{}", "type":"{}", "position":{}}}"#,
                phoneme.roman, phoneme.bengali, phoneme_type,
                if position == "null" { position } else { format!(r#""{}""#, position) }
            )
        }).collect::<Vec<String>>().join(",");
        
        format!("[{}]", phonemes)
    }
    
    /// Convert syllables to a JSON string for JavaScript
    fn syllables_to_json(analysis: &TransliterationAnalysis) -> String {
        let syllables = analysis.syllables.iter().map(|syllable| {
            let consonant_text = syllable.get_consonant_text();
            let vowel_text = syllable.get_vowel_text();
            let modifier_text = syllable.get_modifier_text();
            let is_standalone = syllable.is_standalone();
            let has_reph = syllable.has_reph();
            let has_ya_phala = syllable.has_ya_phala();
            
            format!(
                r#"{{"consonants":"{}", "vowel":"{}", "modifiers":"{}", "standalone":{}, "reph":{}, "yaPhala":{}}}"#,
                consonant_text, vowel_text, modifier_text, is_standalone, has_reph, has_ya_phala
            )
        }).collect::<Vec<String>>().join(",");
        
        format!("[{}]", syllables)
    }
}

// Default implementation
impl Default for WasmEngine {
    fn default() -> Self {
        Self::new()
    }
}

// Initialize the WebAssembly module
#[wasm_bindgen(start)]
pub fn start() {
    crate::wasm::set_panic_hook();
    console_log!("Obadh Engine WASM module initialized");
}