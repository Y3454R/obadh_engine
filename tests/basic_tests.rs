use obadh_engine::ObadhEngine;

#[test]
fn test_basic_cases() {
    let engine = ObadhEngine::new();
    
    // Basic test cases
    assert_eq!(engine.transliterate("bhalo"), "ভাল");
    assert_eq!(engine.transliterate("bhalO"), "ভালো");
    assert_eq!(engine.transliterate("kok"), "কক");
    assert_eq!(engine.transliterate("kOk"), "কোক");
    assert_eq!(engine.transliterate("v"), "ভ");
    assert_eq!(engine.transliterate("boi"), "বই");
    assert_eq!(engine.transliterate("kokko"), "কক্ক");
    assert_eq!(engine.transliterate("kokkO"), "কক্কো");
    assert_eq!(engine.transliterate("kk"), "কক");
    assert_eq!(engine.transliterate("t``"), "ৎ");
    assert_eq!(engine.transliterate("kOnnO"), "কোন্নো");
}
