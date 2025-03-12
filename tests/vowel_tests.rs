use obadh_engine::ObadhEngine;

#[test]
fn test_standalone_vowels() {
    let engine = ObadhEngine::new();
    
    // Test standalone vowels
    assert_eq!(engine.transliterate("a"), "অ");
    assert_eq!(engine.transliterate("aa"), "আ");
    assert_eq!(engine.transliterate("i"), "ই");
    assert_eq!(engine.transliterate("ii"), "ঈ");
    assert_eq!(engine.transliterate("u"), "উ");
    assert_eq!(engine.transliterate("uu"), "ঊ");
    assert_eq!(engine.transliterate("e"), "এ");
    assert_eq!(engine.transliterate("oi"), "ঐ");
    assert_eq!(engine.transliterate("o"), "ও");
    assert_eq!(engine.transliterate("ou"), "ঔ");
}

#[test]
fn test_vowel_with_consonants() {
    let engine = ObadhEngine::new();
    
    // Test vowels with consonants
    assert_eq!(engine.transliterate("ka"), "কা");
    assert_eq!(engine.transliterate("ki"), "কি");
    assert_eq!(engine.transliterate("ku"), "কু");
    assert_eq!(engine.transliterate("ke"), "কে");
    assert_eq!(engine.transliterate("ko"), "কো");
    assert_eq!(engine.transliterate("koi"), "কৈ");
    assert_eq!(engine.transliterate("kou"), "কৌ");
}

#[test]
fn test_inherent_vowel() {
    let engine = ObadhEngine::new();
    
    // Test inherent vowel (implicit 'অ')
    assert_eq!(engine.transliterate("k"), "ক");
    assert_eq!(engine.transliterate("kml"), "কমল");  // Inherent 'অ' after each consonant
    assert_eq!(engine.transliterate("ghr"), "ঘর");   // Inherent 'অ' after 'ঘ'
}

#[test]
fn test_vowel_sequences() {
    let engine = ObadhEngine::new();
    
    // Test vowel sequences
    assert_eq!(engine.transliterate("aami"), "আমি");   // Sequence of vowels
    assert_eq!(engine.transliterate("aalo"), "আলো");   // Vowel sequence with consonant
    assert_eq!(engine.transliterate("koi"), "কৈ");    // Diphthong after consonant
}

#[test]
fn test_vowel_words() {
    let engine = ObadhEngine::new();
    
    // Test common words with vowels
    assert_eq!(engine.transliterate("aagun"), "আগুন");
    assert_eq!(engine.transliterate("ei"), "এই");
    assert_eq!(engine.transliterate("oi"), "ঐ");     // Standalone diphthong
    assert_eq!(engine.transliterate("aamra"), "আমরা");
    assert_eq!(engine.transliterate("upohar"), "উপহার");
}