use obadh_engine::ObadhEngine;

#[test]
fn test_complex_cases() {
    let engine = ObadhEngine::new();
    
    // Complex test cases
    assert_eq!(engine.transliterate("ShTho"), "ষ্ঠ");
    assert_eq!(engine.transliterate("ddho"), "দ্ধ");
    assert_eq!(engine.transliterate("tmo"), "ত্ম");
    assert_eq!(engine.transliterate("lko"), "ল্ক");
    assert_eq!(engine.transliterate("ngo"), "ঙ্গ");
    assert_eq!(engine.transliterate("ntro"), "ন্ত্র");
    assert_eq!(engine.transliterate("kkhmo"), "ক্ষ্ম");
    assert_eq!(engine.transliterate("ttwo"), "ত্ত্ব");
    assert_eq!(engine.transliterate("ddo"), "দ্ব");
    assert_eq!(engine.transliterate("Tro"), "ট্র");
    assert_eq!(engine.transliterate("Dro"), "ড্র");
}



