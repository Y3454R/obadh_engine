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

/// The main entry point for the Obadh transliteration engine.
/// 
/// This struct provides a convenient API for the transliteration process.
pub struct ObadhEngine {
    transliterator: engine::transliterator::Transliterator,
}

impl ObadhEngine {
    /// Create a new instance of the Obadh Engine
    pub fn new() -> Self {
        ObadhEngine {
            transliterator: engine::transliterator::Transliterator::new(),
        }
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
        self.transliterator.transliterate(text)
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
}

impl Default for ObadhEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_transliteration_works() {
        let engine = ObadhEngine::new();
        assert_eq!(engine.transliterate("ami"), "আমি");
        assert_eq!(engine.transliterate("bangla"), "বাংলা");
    }
    
    #[test]
    fn maintains_whitespace_and_punctuation() {
        let engine = ObadhEngine::new();
        assert_eq!(
            engine.transliterate("ami bangla boli."),
            "আমি বাংলা বলি।"
        );
    }
}