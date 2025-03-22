use obadh_engine::ObadhEngine;

#[test]
fn test_common_words() {
    let engine = ObadhEngine::new();
    
    // Common Bengali words with expected Avro transliterations
    assert_eq!(engine.transliterate("bangla"), "বাংলা");
    assert_eq!(engine.transliterate("bishwo"), "বিশ্ব");
    assert_eq!(engine.transliterate("porIkkha"), "পরীক্ষা");
    assert_eq!(engine.transliterate("akash"), "আকাশ");
    assert_eq!(engine.transliterate("mrrityu"), "মৃত্যু");
    assert_eq!(engine.transliterate("odhyapok"), "অধ্যাপক");
    assert_eq!(engine.transliterate("biddaloy"), "বিদ্যালয়");
    assert_eq!(engine.transliterate("sUrrjo"), "সূর্য");
    assert_eq!(engine.transliterate("byakoron"), "ব্যাকরণ");
    assert_eq!(engine.transliterate("swadhInota"), "স্বাধীনতা");
    assert_eq!(engine.transliterate("OUShodh"), "ঔষধ");
    assert_eq!(engine.transliterate("rritu"), "ঋতু");
    assert_eq!(engine.transliterate("sneho"), "স্নেহ");
    assert_eq!(engine.transliterate("ggan"), "জ্ঞান");
    assert_eq!(engine.transliterate("shikkha"), "শিক্ষা");
}

#[test]
fn test_more_common_words() {
    let engine = ObadhEngine::new();
    
    // More common Bengali words with expected Avro transliterations
    assert_eq!(engine.transliterate("kkhomota"), "ক্ষমতা");
    assert_eq!(engine.transliterate("prrithibI"), "পৃথিবী");
    assert_eq!(engine.transliterate("chiThi"), "চিঠি");
    assert_eq!(engine.transliterate("Dhaka"), "ঢাকা");
    assert_eq!(engine.transliterate("Thakur"), "ঠাকুর");
    assert_eq!(engine.transliterate("jhor"), "ঝড়");
    assert_eq!(engine.transliterate("dwIp"), "দ্বীপ");
    assert_eq!(engine.transliterate("aSharh"), "আষাঢ়");
    assert_eq!(engine.transliterate("biggaan"), "বিজ্ঞান");
    assert_eq!(engine.transliterate("ghuri"), "ঘুড়ি");
    assert_eq!(engine.transliterate("nko"), "ঙ্ক");
    assert_eq!(engine.transliterate("nirbhoy"), "নির্ভয়");
    assert_eq!(engine.transliterate("dukkho"), "দুঃখ");
    assert_eq!(engine.transliterate("ittadi"), "ইত্যাদি");
    assert_eq!(engine.transliterate("onchol"), "অঞ্চল");
} 