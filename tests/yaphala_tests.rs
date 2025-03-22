use obadh_engine::ObadhEngine;

#[test]
fn test_yaphala_cases() {
    let engine = ObadhEngine::new().with_debug_mode(true);
    
    // Yaphala test cases with debug output
    println!("Testing 'bidyaloy':");
    let result = engine.transliterate("bidyaloy");
    assert_eq!(result, "বিদ্যালয়");
    
    println!("\nTesting 'modhyam':");
    let result = engine.transliterate("modhyam");
    assert_eq!(result, "মধ্যম");
    
    println!("\nTesting 'byakti':");
    let result = engine.transliterate("byakti");
    assert_eq!(result, "ব্যক্তি");
    
    println!("\nTesting 'sompadyo':");
    let result = engine.transliterate("sompadyo");
    assert_eq!(result, "সম্পাদ্য");
    
    println!("\nTesting 'nyay':");
    let result = engine.transliterate("nyay");
    assert_eq!(result, "ন্যায়");
    
    println!("\nTesting 'tyag':");
    let result = engine.transliterate("tyag");
    assert_eq!(result, "ত্যাগ");
    
    println!("\nTesting 'dhyaan':");
    let result = engine.transliterate("dhyaan");
    assert_eq!(result, "ধ্যান");
    
    println!("\nTesting 'shyamol':");
    let result = engine.transliterate("shyamol");
    assert_eq!(result, "শ্যামল");
    
    println!("\nTesting 'kyampo':");
    let result = engine.transliterate("kyampo");
    assert_eq!(result, "ক্যাম্প");
    
    println!("\nTesting 'byabohar':");
    let result = engine.transliterate("byabohar");
    assert_eq!(result, "ব্যবহার");
}
