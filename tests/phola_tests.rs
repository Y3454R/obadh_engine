use obadh_engine::ObadhEngine;

#[test]
fn test_ya_phala_simple() {
    let engine = ObadhEngine::new();
    
    // Test simple ya-phala formations
    assert_eq!(engine.transliterate("by"), "ব্য");   // ba + ya-phala
    assert_eq!(engine.transliterate("ty"), "ত্য");   // ta + ya-phala
    assert_eq!(engine.transliterate("dy"), "দ্য");   // da + ya-phala
    assert_eq!(engine.transliterate("ny"), "ন্য");   // na + ya-phala
}

#[test]
fn test_ya_phala_with_vowels() {
    let engine = ObadhEngine::new();
    
    // Test ya-phala with vowels
    assert_eq!(engine.transliterate("bya"), "ব্যা");   // ba + ya-phala + aa
    assert_eq!(engine.transliterate("tyi"), "ত্যি");   // ta + ya-phala + i
    assert_eq!(engine.transliterate("dyu"), "দ্যু");   // da + ya-phala + u
    assert_eq!(engine.transliterate("nye"), "ন্যে");   // na + ya-phala + e
}

#[test]
fn test_ya_phala_words() {
    let engine = ObadhEngine::new();
    
    // Test common words with ya-phala
    assert_eq!(engine.transliterate("byatha"), "ব্যথা");
    assert_eq!(engine.transliterate("byabostha"), "ব্যবস্থা");
    assert_eq!(engine.transliterate("adhyapon"), "অধ্যাপন");
    assert_eq!(engine.transliterate("maddhyahno"), "মধ্যাহ্ন");
    assert_eq!(engine.transliterate("nyay"), "ন্যায়");
}

#[test]
fn test_ra_phala_simple() {
    let engine = ObadhEngine::new();
    
    // Test simple ra-phala formations
    assert_eq!(engine.transliterate("kr"), "ক্র");   // ka + ra-phala
    assert_eq!(engine.transliterate("pr"), "প্র");   // pa + ra-phala
    assert_eq!(engine.transliterate("dr"), "দ্র");   // da + ra-phala
    assert_eq!(engine.transliterate("gr"), "গ্র");   // ga + ra-phala
}

#[test]
fn test_ra_phala_with_vowels() {
    let engine = ObadhEngine::new();
    
    // Test ra-phala with vowels
    assert_eq!(engine.transliterate("kra"), "ক্রা");   // ka + ra-phala + aa
    assert_eq!(engine.transliterate("pri"), "প্রি");   // pa + ra-phala + i
    assert_eq!(engine.transliterate("dru"), "দ্রু");   // da + ra-phala + u
    assert_eq!(engine.transliterate("gre"), "গ্রে");   // ga + ra-phala + e
}

#[test]
fn test_ra_phala_words() {
    let engine = ObadhEngine::new();
    
    // Test common words with ra-phala
    assert_eq!(engine.transliterate("pran"), "প্রাণ");
    assert_eq!(engine.transliterate("kromo"), "ক্রম");
    assert_eq!(engine.transliterate("drishti"), "দৃষ্টি");
    assert_eq!(engine.transliterate("drishtanto"), "দৃষ্টান্ত");
    assert_eq!(engine.transliterate("grahoN"), "গ্রহণ");
}

#[test]
fn test_ba_phala() {
    let engine = ObadhEngine::new();
    
    // Test ba-phala formations
    assert_eq!(engine.transliterate("dbip"), "দ্বীপ");   // da + ba-phala
    assert_eq!(engine.transliterate("dbi"), "দ্বি");     // da + ba-phala
    assert_eq!(engine.transliterate("tba"), "ত্বা");     // ta + ba-phala
    assert_eq!(engine.transliterate("sbajon"), "স্বজন"); // sa + ba-phala
}

#[test]
fn test_ma_phala() {
    let engine = ObadhEngine::new();
    
    // Test ma-phala formations
    assert_eq!(engine.transliterate("smaroN"), "স্মরণ");   // sa + ma-phala
    assert_eq!(engine.transliterate("padmo"), "পদ্ম");     // pa + da + ma-phala
    assert_eq!(engine.transliterate("ksma"), "ক্ষ্মা");    // kSa + ma-phala
}

#[test]
fn test_multiple_phala() {
    let engine = ObadhEngine::new();
    
    // Test words with multiple phala forms
    assert_eq!(engine.transliterate("briddhy"), "বৃদ্ধ্য");   // ba + ra-phala + dha + ya-phala
    assert_eq!(engine.transliterate("srastaa"), "স্রষ্টা");    // sa + ra-phala + Sha
    assert_eq!(engine.transliterate("smrity"), "স্মৃত্য");     // sa + ma-phala + ra + ta + ya-phala
}