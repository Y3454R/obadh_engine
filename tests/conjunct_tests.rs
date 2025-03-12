use obadh_engine::ObadhEngine;

#[test]
fn test_simple_conjuncts() {
    let engine = ObadhEngine::new();
    
    // Test simple two-consonant conjuncts
    assert_eq!(engine.transliterate("kt"), "ক্ত");
    assert_eq!(engine.transliterate("kl"), "ক্ল");
    assert_eq!(engine.transliterate("ksh"), "ক্ষ");  // Special conjunct
    assert_eq!(engine.transliterate("gn"), "গ্ন");
    assert_eq!(engine.transliterate("ng"), "ঙ");    // Not a conjunct, but a single character
    
    assert_eq!(engine.transliterate("jn"), "জ্ঞ");   // Special conjunct (gya)
    assert_eq!(engine.transliterate("jj"), "জ্জ");
    assert_eq!(engine.transliterate("nch"), "ঞ্চ");
    
    assert_eq!(engine.transliterate("tt"), "ত্ত");
    assert_eq!(engine.transliterate("tv"), "ত্ব");
    assert_eq!(engine.transliterate("sth"), "স্থ");
    assert_eq!(engine.transliterate("shch"), "শ্চ");
}

#[test]
fn test_complex_conjuncts() {
    let engine = ObadhEngine::new();
    
    // Test three-consonant conjuncts
    assert_eq!(engine.transliterate("ntr"), "ন্ত্র");
    assert_eq!(engine.transliterate("str"), "স্ত্র");
    assert_eq!(engine.transliterate("ndr"), "ন্দ্র");
    assert_eq!(engine.transliterate("ktr"), "ক্ত্র");
    
    // Complex sequences
    assert_eq!(engine.transliterate("kShN"), "ক্ষ্ণ");   // kSha + Na
    assert_eq!(engine.transliterate("spr"), "স্প্র");   // sa + pa + ra
}

#[test]
fn test_conjunct_words() {
    let engine = ObadhEngine::new();
    
    // Test common words with conjuncts
    assert_eq!(engine.transliterate("bidya"), "বিদ্যা");
    assert_eq!(engine.transliterate("bidyut"), "বিদ্যুৎ");
    assert_eq!(engine.transliterate("bastobtaa"), "বাস্তবতা");
    assert_eq!(engine.transliterate("baktrita"), "বক্তৃতা");
    assert_eq!(engine.transliterate("sthir"), "স্থির");
    assert_eq!(engine.transliterate("bigyapti"), "বিজ্ঞপ্তি");
}

#[test]
fn test_conjuncts_with_ya_phala() {
    let engine = ObadhEngine::new();
    
    // Test conjuncts with y-phola (্য)
    assert_eq!(engine.transliterate("baky"), "বাক্য");
    assert_eq!(engine.transliterate("svastya"), "স্বাস্থ্য");
    assert_eq!(engine.transliterate("adya"), "আদ্য");
    assert_eq!(engine.transliterate("madyahna"), "মধ্যাহ্ন");
}

#[test]
fn test_conjuncts_with_ra_phala() {
    let engine = ObadhEngine::new();
    
    // Test conjuncts with r-phola (্র)
    assert_eq!(engine.transliterate("kr"), "ক্র");
    assert_eq!(engine.transliterate("pr"), "প্র");
    assert_eq!(engine.transliterate("mrittika"), "মৃত্তিকা");
    assert_eq!(engine.transliterate("srot"), "স্রোত");
}

#[test]
fn test_conjuncts_with_la_phala() {
    let engine = ObadhEngine::new();
    
    // Test conjuncts with l-phola (্ল)
    assert_eq!(engine.transliterate("kl"), "ক্ল");
    assert_eq!(engine.transliterate("pl"), "প্ল");
    assert_eq!(engine.transliterate("pluto"), "প্লুতো");
    assert_eq!(engine.transliterate("plabito"), "প্লাবিত");
}