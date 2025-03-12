use obadh_engine::ObadhEngine;

#[test]
fn test_simple_consonants() {
    let engine = ObadhEngine::new();
    
    // Test simple consonants
    assert_eq!(engine.transliterate("k"), "ক");
    assert_eq!(engine.transliterate("kh"), "খ");
    assert_eq!(engine.transliterate("g"), "গ");
    assert_eq!(engine.transliterate("gh"), "ঘ");
    assert_eq!(engine.transliterate("ng"), "ঙ");
    
    assert_eq!(engine.transliterate("ch"), "চ");
    assert_eq!(engine.transliterate("chh"), "ছ");
    assert_eq!(engine.transliterate("j"), "জ");
    assert_eq!(engine.transliterate("jh"), "ঝ");
    
    assert_eq!(engine.transliterate("T"), "ট");
    assert_eq!(engine.transliterate("Th"), "ঠ");
    assert_eq!(engine.transliterate("D"), "ড");
    assert_eq!(engine.transliterate("Dh"), "ঢ");
    assert_eq!(engine.transliterate("N"), "ণ");
    
    assert_eq!(engine.transliterate("t"), "ত");
    assert_eq!(engine.transliterate("th"), "থ");
    assert_eq!(engine.transliterate("d"), "দ");
    assert_eq!(engine.transliterate("dh"), "ধ");
    assert_eq!(engine.transliterate("n"), "ন");
    
    assert_eq!(engine.transliterate("p"), "প");
    assert_eq!(engine.transliterate("ph"), "ফ");
    assert_eq!(engine.transliterate("b"), "ব");
    assert_eq!(engine.transliterate("bh"), "ভ");
    assert_eq!(engine.transliterate("m"), "ম");
    
    assert_eq!(engine.transliterate("z"), "য");
    assert_eq!(engine.transliterate("r"), "র");
    assert_eq!(engine.transliterate("l"), "ল");
    
    assert_eq!(engine.transliterate("sh"), "শ");
    assert_eq!(engine.transliterate("S"), "ষ");
    assert_eq!(engine.transliterate("s"), "স");
    assert_eq!(engine.transliterate("h"), "হ");
}

#[test]
fn test_consonant_sequences() {
    let engine = ObadhEngine::new();
    
    // Test consonant sequences without conjuncts
    assert_eq!(engine.transliterate("krm"), "ক্রম");
    assert_eq!(engine.transliterate("prt"), "প্রত");
    assert_eq!(engine.transliterate("sthn"), "স্থন");
}

#[test]
fn test_consonant_with_inherent_vowels() {
    let engine = ObadhEngine::new();
    
    // Test words with consonants and inherent vowels
    assert_eq!(engine.transliterate("dh"), "ধ");
    assert_eq!(engine.transliterate("dhk"), "ধক");
    assert_eq!(engine.transliterate("jns"), "জনস");
}

#[test]
fn test_alternate_consonant_forms() {
    let engine = ObadhEngine::new();
    
    // Test alternate forms for consonants
    assert_eq!(engine.transliterate("f"), "ফ");     // Alternative for 'ph'
    assert_eq!(engine.transliterate("w"), "ও");     // For foreign words
    assert_eq!(engine.transliterate("v"), "ভ");     // Often pronounced as 'bh' in Bengali
}

#[test]
fn test_consonant_words() {
    let engine = ObadhEngine::new();
    
    // Test common words with consonant emphasis
    assert_eq!(engine.transliterate("bhasha"), "ভাষা");
    assert_eq!(engine.transliterate("likhte"), "লিখতে");
    assert_eq!(engine.transliterate("dhaka"), "ঢাকা");
    assert_eq!(engine.transliterate("ghori"), "ঘড়ি");
}