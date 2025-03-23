use obadh_engine::{Tokenizer, PhoneticUnitType};

#[test]
fn test_explicit_hasant_notation() {
    let tokenizer = Tokenizer::new();
    
    // Test explicit hasant notation (n,,d,,r)
    let units = tokenizer.tokenize_word("n,,d,,r");
    
    println!("Tokenization of 'n,,d,,r':");
    for unit in &units {
        println!("Unit '{}' type: {:?}", unit.text, unit.unit_type);
    }
    
    // Verify we get a conjunct
    assert!(!units.is_empty());
    assert_eq!(units.len(), 1);
    assert_eq!(units[0].unit_type, PhoneticUnitType::Conjunct);
    
    // Test with trailing hasant (n,,d,,r,,)
    let units = tokenizer.tokenize_word("n,,d,,r,,");
    
    println!("Tokenization of 'n,,d,,r,,':");
    for unit in &units {
        println!("Unit '{}' type: {:?}", unit.text, unit.unit_type);
    }
    
    // Verify we get a conjunct
    assert!(!units.is_empty());
    assert_eq!(units.len(), 1);
    assert_eq!(units[0].unit_type, PhoneticUnitType::Conjunct);
    
    // Compare both formats
    let standard = tokenizer.tokenize_word("n,,d,,r");
    let with_trailing = tokenizer.tokenize_word("n,,d,,r,,");
    
    println!("Comparing 'n,,d,,r' and 'n,,d,,r,,':");
    println!("'n,,d,,r': {:?}", standard[0].text);
    println!("'n,,d,,r,,': {:?}", with_trailing[0].text);
}

#[test]
fn test_explicit_hasant_with_vowel() {
    let tokenizer = Tokenizer::new();
    
    // Test explicit hasant with vowel (n,,d,,rA)
    let units = tokenizer.tokenize_word("n,,d,,rA");
    
    println!("Tokenization of 'n,,d,,rA':");
    for unit in &units {
        println!("Unit '{}' type: {:?}", unit.text, unit.unit_type);
    }
    
    // Verify we get a conjunct with vowel
    assert!(!units.is_empty());
    assert_eq!(units.len(), 1);
    assert_eq!(units[0].unit_type, PhoneticUnitType::ConjunctWithVowel);
    
    // Test with terminator vowel (n,,d,,ro)
    let units = tokenizer.tokenize_word("n,,d,,ro");
    
    println!("Tokenization of 'n,,d,,ro':");
    for unit in &units {
        println!("Unit '{}' type: {:?}", unit.text, unit.unit_type);
    }
    
    // Verify we get a conjunct with terminator
    assert!(!units.is_empty());
    assert_eq!(units.len(), 1);
    assert_eq!(units[0].unit_type, PhoneticUnitType::ConjunctWithTerminator);
}

#[test]
fn test_mixed_hasant_notation() {
    let tokenizer = Tokenizer::new();
    
    // Test mixing auto and explicit hasant (k,,lr)
    let units = tokenizer.tokenize_word("k,,lr");
    
    println!("Tokenization of 'k,,lr':");
    for unit in &units {
        println!("Unit '{}' type: {:?}", unit.text, unit.unit_type);
    }
    
    // Verify the output structure
    assert!(!units.is_empty());
    
    // Test more complex mixed notation (n,,dr)
    let units = tokenizer.tokenize_word("n,,dr");
    
    println!("Tokenization of 'n,,dr':");
    for unit in &units {
        println!("Unit '{}' type: {:?}", unit.text, unit.unit_type);
    }
    
    // Verify the output structure
    assert!(!units.is_empty());
} 