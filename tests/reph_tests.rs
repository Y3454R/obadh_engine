use obadh_engine::ObadhEngine;

#[test]
fn test_reph_simple() {
    let engine = ObadhEngine::new();
    
    // Test simple reph formations
    assert_eq!(engine.transliterate("rm"), "র্ম");   // ra + hasanta + ma
    assert_eq!(engine.transliterate("rk"), "র্ক");   // ra + hasanta + ka
    assert_eq!(engine.transliterate("rg"), "র্গ");   // ra + hasanta + ga
    assert_eq!(engine.transliterate("rp"), "র্প");   // ra + hasanta + pa
}

#[test]
fn test_reph_with_vowels() {
    let engine = ObadhEngine::new();
    
    // Test reph with vowels
    assert_eq!(engine.transliterate("rma"), "র্মা");   // reph + ma + aa
    assert_eq!(engine.transliterate("rke"), "র্কে");   // reph + ka + e
    assert_eq!(engine.transliterate("rgi"), "র্গি");   // reph + ga + i
    assert_eq!(engine.transliterate("rpu"), "র্পু");   // reph + pa + u
}

#[test]
fn test_reph_words() {
    let engine = ObadhEngine::new();
    
    // Test common words with reph
    assert_eq!(engine.transliterate("karma"), "কর্ম");
    assert_eq!(engine.transliterate("nirbachon"), "নির্বাচন");
    assert_eq!(engine.transliterate("sarbik"), "সার্বিক");
    assert_eq!(engine.transliterate("barsha"), "বর্ষ");
    assert_eq!(engine.transliterate("artha"), "অর্থ");
}

#[test]
fn test_reph_with_conjuncts() {
    let engine = ObadhEngine::new();
    
    // Test reph with complex conjuncts
    assert_eq!(engine.transliterate("karmakarta"), "কর্মকর্তা");   // Double reph
    assert_eq!(engine.transliterate("kartr"), "কর্ত্র");           // reph + ta + ra
    assert_eq!(engine.transliterate("nirjhar"), "নির্ঝর");         // reph + jha
    assert_eq!(engine.transliterate("birmohona"), "বীর্মোহনা");     // reph + ma with long ii
}

#[test]
fn test_reph_special_cases() {
    let engine = ObadhEngine::new();
    
    // Test special cases with reph
    assert_eq!(engine.transliterate("darshan"), "দর্শন");      // reph + sha
    assert_eq!(engine.transliterate("nirmata"), "নির্মাতা");    // reph + ma + long aa
    assert_eq!(engine.transliterate("purba"), "পূর্ব");        // reph + ba with long u
    assert_eq!(engine.transliterate("nirlajja"), "নির্লজ্জ");   // reph + la + ja
}