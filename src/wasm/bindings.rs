//! WASM bindings for Obadh Engine

use wasm_bindgen::prelude::*;
use crate::ObadhEngine;

/// WebAssembly wrapper for the Obadh Engine
#[wasm_bindgen]
pub struct WasmObadhEngine {
    engine: ObadhEngine,
}

#[wasm_bindgen]
impl WasmObadhEngine {
    /// Create a new instance of the engine
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            engine: ObadhEngine::new(),
        }
    }
    
    /// Transliterate Roman text to Bengali
    #[wasm_bindgen]
    pub fn transliterate(&self, text: &str) -> String {
        self.engine.transliterate(text)
    }
}