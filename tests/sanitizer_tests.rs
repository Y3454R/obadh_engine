use obadh_engine::Sanitizer;

#[test]
fn test_valid_input() {
    let sanitizer = Sanitizer::new();
    
    // Valid inputs
    assert!(sanitizer.sanitize("Hello World").is_ok());
    assert!(sanitizer.sanitize("abc123").is_ok());
    assert!(sanitizer.sanitize("k,, kaaj").is_ok()); // Avro with hasant
    assert!(sanitizer.sanitize("123").is_ok()); // Numerals
    assert!(sanitizer.sanitize("!@#$%^&*()").is_ok()); // Special characters
}

#[test]
fn test_invalid_input() {
    let sanitizer = Sanitizer::new();
    
    // Invalid inputs containing Bengali characters
    assert!(sanitizer.sanitize("অআই").is_err());
    assert!(sanitizer.sanitize("Hello অ World").is_err());
    
    // Invalid inputs containing other non-Latin characters
    assert!(sanitizer.sanitize("こんにちは").is_err()); // Japanese
    assert!(sanitizer.sanitize("Привет").is_err()); // Russian
}

#[test]
fn test_clean_input() {
    let sanitizer = Sanitizer::new();
    
    // Clean should remove invalid characters
    assert_eq!(sanitizer.clean("Hello অ World"), "Hello  World");
    assert_eq!(sanitizer.clean("abc123こんにちは"), "abc123");
    assert_eq!(sanitizer.clean("!@#$%^&*()Привет"), "!@#$%^&*()");
}

#[test]
fn test_is_valid() {
    let sanitizer = Sanitizer::new();
    
    // Test valid and invalid inputs
    assert!(sanitizer.is_valid("Hello World"));
    assert!(sanitizer.is_valid("abc123"));
    assert!(!sanitizer.is_valid("অআই"));
    assert!(!sanitizer.is_valid("Hello অ World"));
} 