use obadh_engine::ObadhEngine;

#[test]
fn test_basic_vowel_transliteration() {
    let engine = ObadhEngine::new();
    
    // Independent vowels (at the beginning of a word)
    assert_eq!(engine.transliterate("o"), "অ");     // অ-কার (a-kar)
    assert_eq!(engine.transliterate("A"), "আ");     // আ-কার (aa-kar)
    assert_eq!(engine.transliterate("i"), "ই");     // ই-কার (i-kar)
    assert_eq!(engine.transliterate("I"), "ঈ");     // ঈ-কার (dirgho i-kar)
    assert_eq!(engine.transliterate("u"), "উ");     // উ-কার (u-kar)
    assert_eq!(engine.transliterate("U"), "ঊ");     // ঊ-কার (dirgho u-kar)
    assert_eq!(engine.transliterate("e"), "এ");     // এ-কার (e-kar)
    assert_eq!(engine.transliterate("OI"), "ঐ");    // ঐ-কার (oi-kar)
    assert_eq!(engine.transliterate("O"), "ও");     // ও-কার (o-kar)
    assert_eq!(engine.transliterate("OU"), "ঔ");    // ঔ-কার (ou-kar)
    assert_eq!(engine.transliterate("rri"), "ঋ");   // ঋ-কার (ri-kar)
}

#[test]
fn test_vowel_with_consonants() {
    let engine = ObadhEngine::new();
    
    // Consonant + vowel combinations (vowel symbols/kars)
    assert_eq!(engine.transliterate("ko"), "ক");     // Default/inherent vowel (no visible kar)
    assert_eq!(engine.transliterate("kA"), "কা");    // আ-কার (aa-kar)
    assert_eq!(engine.transliterate("ki"), "কি");    // ই-কার (i-kar)
    assert_eq!(engine.transliterate("kI"), "কী");    // ঈ-কার (dirgho i-kar)
    assert_eq!(engine.transliterate("ku"), "কু");    // উ-কার (u-kar)
    assert_eq!(engine.transliterate("kU"), "কূ");    // ঊ-কার (dirgho u-kar)
    assert_eq!(engine.transliterate("ke"), "কে");    // এ-কার (e-kar)
    assert_eq!(engine.transliterate("kOI"), "কৈ");   // ঐ-কার (oi-kar)
    assert_eq!(engine.transliterate("kO"), "কো");    // ও-কার (o-kar)
    assert_eq!(engine.transliterate("kOU"), "কৌ");   // ঔ-কার (ou-kar)
    assert_eq!(engine.transliterate("krri"), "কৃ");  // ঋ-কার (ri-kar)
}

#[test]
fn test_vowel_initial_words() {
    let engine = ObadhEngine::new();
    
    // Vowels at the beginning of words
    assert_eq!(engine.transliterate("amar"), "আমার");  // 'a' becomes আ at beginning
    assert_eq!(engine.transliterate("ele"), "এলে");    // 'e' becomes এ at beginning
}

#[test]
fn test_o_as_conjunct_breaker() {
    let engine = ObadhEngine::new();
    
    // 'o' as conjunct breaker
    assert_eq!(engine.transliterate("kk"), "ক্ক");     // Forms conjunct (k + hasant + k)
    assert_eq!(engine.transliterate("kok"), "কক");     // Prevents conjunct by inserting implicit 'o'
    assert_eq!(engine.transliterate("kOk"), "কোক");    // Inserts the full o-kar vowel
}

#[test]
fn test_vowel_vowel_combinations() {
    let engine = ObadhEngine::new();
    
    // Vowel + Vowel combinations
    assert_eq!(engine.transliterate("aa"), "আ");      // a + a -> আ
    assert_eq!(engine.transliterate("ai"), "আই");     // a + i -> আই
    assert_eq!(engine.transliterate("au"), "আউ");     // a + u -> আউ
    assert_eq!(engine.transliterate("ae"), "আএ");     // a + e -> আএ
    assert_eq!(engine.transliterate("ao"), "আও");     // a + o -> আও
    assert_eq!(engine.transliterate("ia"), "ইয়া");    // i + a -> ইয়া
    assert_eq!(engine.transliterate("io"), "ইও");     // i + o -> ইও 
    assert_eq!(engine.transliterate("eo"), "এও");     // e + o -> এও
}

#[test]
fn test_vowel_modifications() {
    let engine = ObadhEngine::new();
    
    // Vowel modifications
    assert_eq!(engine.transliterate("cha^d"), "চাঁদ");  // Nasalization (chandrabindu)
    assert_eq!(engine.transliterate("du:kh"), "দুঃখ");  // Visarga
} 