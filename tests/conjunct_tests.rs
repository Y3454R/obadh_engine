use obadh_engine::{Tokenizer, PhoneticUnitType};

#[test]
fn test_basic_conjunct_formation() {
    let tokenizer = Tokenizer::new();
    
    // Test automatic conjunct formation from consecutive consonants
    let units = tokenizer.tokenize_word("kk");
    
    println!("Tokenization of 'kk':");
    for unit in &units {
        println!("Unit '{}' type: {:?}", unit.text, unit.unit_type);
    }
    
    // Verify we got a conjunct
    assert_eq!(units.len(), 1);
    assert_eq!(units[0].unit_type, PhoneticUnitType::Conjunct);
    assert_eq!(units[0].text, "k,,k");
    
    // Test another common consonant pair
    let units = tokenizer.tokenize_word("ks");
    
    println!("Tokenization of 'ks':");
    for unit in &units {
        println!("Unit '{}' type: {:?}", unit.text, unit.unit_type);
    }
    
    // Verify we got a conjunct
    assert_eq!(units.len(), 1);
    assert_eq!(units[0].unit_type, PhoneticUnitType::Conjunct);
    assert_eq!(units[0].text, "k,,s");
}

#[test]
fn test_explicit_conjunct_formation() {
    let tokenizer = Tokenizer::new();
    
    // Test explicit conjunct formation with hasant
    let units = tokenizer.tokenize_word("k,,k");
    
    println!("Tokenization of 'k,,k':");
    for unit in &units {
        println!("Unit '{}' type: {:?}", unit.text, unit.unit_type);
    }
    
    // Verify we got a conjunct
    assert_eq!(units.len(), 1);
    assert_eq!(units[0].unit_type, PhoneticUnitType::Conjunct);
    assert_eq!(units[0].text, "k,,k");
    
    // Test with different consonants
    let units = tokenizer.tokenize_word("g,,gh");
    
    println!("Tokenization of 'g,,gh':");
    for unit in &units {
        println!("Unit '{}' type: {:?}", unit.text, unit.unit_type);
    }
    
    // Verify we got a conjunct
    assert_eq!(units.len(), 1);
    assert_eq!(units[0].unit_type, PhoneticUnitType::Conjunct);
}

#[test]
fn test_conjunct_with_vowel() {
    let tokenizer = Tokenizer::new();
    
    // Test conjunct with regular vowel
    let units = tokenizer.tokenize_word("kkA");
    
    println!("Tokenization of 'kkA':");
    for unit in &units {
        println!("Unit '{}' type: {:?}", unit.text, unit.unit_type);
    }
    
    // Verify we got a conjunct with vowel
    assert_eq!(units.len(), 1);
    assert_eq!(units[0].unit_type, PhoneticUnitType::ConjunctWithVowel);
    
    // Test with uppercase O (full vowel)
    let units = tokenizer.tokenize_word("kkO");
    
    println!("Tokenization of 'kkO':");
    for unit in &units {
        println!("Unit '{}' type: {:?}", unit.text, unit.unit_type);
    }
    
    // Verify we got a conjunct with vowel
    assert_eq!(units.len(), 1);
    assert_eq!(units[0].unit_type, PhoneticUnitType::ConjunctWithVowel);
}

#[test]
fn test_conjunct_with_terminator() {
    let tokenizer = Tokenizer::new();
    
    // Test conjunct with terminator vowel (lowercase o)
    let units = tokenizer.tokenize_word("kko");
    
    println!("Tokenization of 'kko':");
    for unit in &units {
        println!("Unit '{}' type: {:?}", unit.text, unit.unit_type);
    }
    
    // Verify we got a conjunct with terminator
    assert_eq!(units.len(), 1);
    assert_eq!(units[0].unit_type, PhoneticUnitType::ConjunctWithTerminator);
}

#[test]
fn test_complex_conjunct_sequences() {
    let tokenizer = Tokenizer::new();
    
    // Test multiple consecutive consonants (should form conjuncts in pairs)
    let units = tokenizer.tokenize_word("kkk");
    
    println!("Tokenization of 'kkk':");
    for unit in &units {
        println!("Unit '{}' type: {:?}", unit.text, unit.unit_type);
    }
    
    // Verify we got a conjunct followed by a consonant
    assert_eq!(units.len(), 2);
    assert_eq!(units[0].unit_type, PhoneticUnitType::Conjunct);
    assert_eq!(units[1].unit_type, PhoneticUnitType::Consonant);
    
    // Test consonant + consonant + consonantWithVowel
    let units = tokenizer.tokenize_word("nkkO");
    
    println!("Tokenization of 'nkkO':");
    for unit in &units {
        println!("Unit '{}' type: {:?}", unit.text, unit.unit_type);
    }
    
    // The tokenizer forms conjuncts from the first pair it encounters
    // So we get n,,k + kO instead of n + k,,kO
    assert_eq!(units.len(), 2);
    assert_eq!(units[0].unit_type, PhoneticUnitType::Conjunct);
    assert_eq!(units[1].unit_type, PhoneticUnitType::ConsonantWithVowel);
}

#[test]
fn test_comparison_auto_and_explicit_conjuncts() {
    let tokenizer = Tokenizer::new();
    
    // Compare automatic versus explicit conjuncts
    let auto_units = tokenizer.tokenize_word("kk");
    let explicit_units = tokenizer.tokenize_word("k,,k");
    
    println!("Comparing 'kk' and 'k,,k':");
    println!("'kk': {:?}", auto_units);
    println!("'k,,k': {:?}", explicit_units);
    
    // Both should produce the same result
    assert_eq!(auto_units.len(), 1);
    assert_eq!(explicit_units.len(), 1);
    assert_eq!(auto_units[0].unit_type, PhoneticUnitType::Conjunct);
    assert_eq!(explicit_units[0].unit_type, PhoneticUnitType::Conjunct);
    assert_eq!(auto_units[0].text, "k,,k");
    assert_eq!(explicit_units[0].text, "k,,k");
}

#[test]
fn test_consonant_with_conjunct_sequences() {
    let tokenizer = Tokenizer::new();
    
    // Test consonant followed by a consonant with vowel
    let units = tokenizer.tokenize_word("kkO");
    
    println!("Tokenization of 'kkO':");
    for unit in &units {
        println!("Unit '{}' type: {:?}", unit.text, unit.unit_type);
    }
    
    // Verify we got a conjunct with vowel
    assert_eq!(units.len(), 1);
    assert_eq!(units[0].unit_type, PhoneticUnitType::ConjunctWithVowel);
    assert_eq!(units[0].text, "k,,kO");
    
    // Test similar case with terminator vowel
    let units = tokenizer.tokenize_word("kko");
    
    println!("Tokenization of 'kko':");
    for unit in &units {
        println!("Unit '{}' type: {:?}", unit.text, unit.unit_type);
    }
    
    // Verify we got a conjunct with terminator
    assert_eq!(units.len(), 1);
    assert_eq!(units[0].unit_type, PhoneticUnitType::ConjunctWithTerminator);
    assert_eq!(units[0].text, "k,,ko");
} 