use obadh_engine::ObadhEngine;

#[test]
fn test_special_symbols() {
    let engine = ObadhEngine::new();
    
    // Test special symbols
    assert_eq!(engine.transliterate("kon^"), "কোঁ");    // Chandrabindu
    assert_eq!(engine.transliterate("du:kho"), "দুঃখ");  // Visarga
    assert_eq!(engine.transliterate("ban."), "বাং");     // Anusvar
    assert_eq!(engine.transliterate("tat."), "তৎ");      // Khanda Ta
}

#[test]
fn test_numbers() {
    let engine = ObadhEngine::new();
    
    // Test Bengali numerals
    assert_eq!(engine.transliterate("0123456789"), "০১২৩৪৫৬৭৮৯");
}

#[test]
fn test_punctuation() {
    let engine = ObadhEngine::new();
    
    // Test punctuation conversion
    assert_eq!(engine.transliterate("ami."), "আমি।");
    assert_eq!(engine.transliterate("tumi?"), "তুমি?");
    assert_eq!(engine.transliterate("bhai!"), "ভাই!");
    assert_eq!(engine.transliterate("ami, tumi"), "আমি, তুমি");
}

#[test]
fn test_mixed_content() {
    let engine = ObadhEngine::new();
    
    // Test mixed content
    assert_eq!(engine.transliterate("amar 2ti boi"), "আমার ২টি বই");
    assert_eq!(engine.transliterate("Se 100% Thik"), "সে ১০০% ঠিক");
}

#[test]
fn test_complex_sentences() {
    let engine = ObadhEngine::new();
    
    // Test complex sentences
    assert_eq!(
        engine.transliterate("amar sonar bangla, ami tomay bhalobashi."),
        "আমার সোনার বাংলা, আমি তোমায় ভালোবাসি।"
    );
    
    assert_eq!(
        engine.transliterate("aj amar mon bhalo nei. ki kori?"),
        "আজ আমার মন ভালো নেই। কি করি?"
    );
}

#[test]
fn test_complex_words() {
    let engine = ObadhEngine::new();
    
    // Test challenging words
    assert_eq!(engine.transliterate("driShTibhonggi"), "দৃষ্টিভঙ্গি");
    assert_eq!(engine.transliterate("srIShTi"), "সৃষ্টি");
    assert_eq!(engine.transliterate("pratisruti"), "প্রতিশ্রুতি");
    assert_eq!(engine.transliterate("jijnasa"), "জিজ্ঞাসা");
    assert_eq!(engine.transliterate("uttaraDhikar"), "উত্তরাধিকার");
}

#[test]
fn test_corner_cases() {
    let engine = ObadhEngine::new();
    
    // Test corner cases and edge cases
    assert_eq!(engine.transliterate(""), "");     // Empty string
    assert_eq!(engine.transliterate(" "), " ");   // Just space
    assert_eq!(engine.transliterate("r.r"), "র্র");  // reph + ra
    assert_eq!(engine.transliterate("zkzk"), "যকযক");  // Sequence of 'z' and 'k'
}

#[test]
fn test_phonologically_complex() {
    let engine = ObadhEngine::new();
    
    // Test phonologically complex words
    assert_eq!(engine.transliterate("mrityunjoy"), "মৃত্যুঞ্জয়");
    assert_eq!(engine.transliterate("arghya"), "অর্ঘ্য");
    assert_eq!(engine.transliterate("aishbarya"), "ঐশ্বর্য");
    assert_eq!(engine.transliterate("svarga"), "স্বর্গ");
}