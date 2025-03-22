use obadh_engine::ObadhEngine;

#[test]
fn test_bofola_cases() {
    let engine = ObadhEngine::new();
    
    // Bo-fola test cases (w stands for ব-ফলা)
    assert_eq!(engine.transliterate("bishwo"), "বিশ্ব");
    assert_eq!(engine.transliterate("dwip"), "দ্বীপ");
    assert_eq!(engine.transliterate("atmbiSwas "), "আত্মবিশ্বাস");
    assert_eq!(engine.transliterate("dwIp"), "দ্বীপ");
    assert_eq!(engine.transliterate("dhwoni"), "ধ্বনি");
    assert_eq!(engine.transliterate("anubadwok"), "অনুবাদ্বক");
} 