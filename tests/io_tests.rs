use obadh_engine::{ObadhEngine, OutputFormat};
use std::io::{BufReader, Cursor, Read};
use serde_json::Value;

/// Test vectors for known working cases (from vowel tests)
fn get_test_cases() -> Vec<(&'static str, &'static str)> {
    vec![
        ("a", "আ"),
        ("e", "এ"),
        ("i", "ই"),
        ("o", "অ"),
        ("u", "উ"),
        
        ("ama", "আমা"),
        ("ele", "এলে"),
        ("ila", "ইলা"),
        ("oke", "অকে"),
        ("ura", "উরা"),
        
        ("amar", "আমার"),
        ("tOmar", "তোমার")
    ]
}

#[test]
fn test_standard_transliteration() {
    let engine = ObadhEngine::new();
    
    for &(input, expected) in &get_test_cases() {
        let result = engine.transliterate(input);
        assert_eq!(result, expected, "Failed on input: {}", input);
    }
}

#[test]
fn test_json_output_format() {
    let engine = ObadhEngine::new()
        .with_output_format(OutputFormat::Json);
    
    for (input, expected) in get_test_cases().into_iter().take(3) {
        let json_result = engine.transliterate_as(input);
        
        // Parse JSON and verify expected fields
        let parsed: Value = serde_json::from_str(&json_result).expect("Valid JSON");
        
        assert_eq!(parsed["input"], input, "Input mismatch for: {}", input);
        assert_eq!(parsed["output"], expected, "Output mismatch for: {}", input);
    }
}

#[test]
fn test_xml_output_format() {
    let engine = ObadhEngine::new()
        .with_output_format(OutputFormat::Xml);
    
    for (input, expected) in get_test_cases().into_iter().take(2) {
        let xml_result = engine.transliterate_as(input);
        
        // Simple check for XML structure
        assert!(xml_result.starts_with("<result>"), "XML should start with <result>");
        assert!(xml_result.contains(&format!("<input>{}</input>", input)), "XML should contain input");
        assert!(xml_result.contains(&format!("<output>{}</output>", expected)), "XML should contain output");
        assert!(xml_result.ends_with("</result>"), "XML should end with </result>");
    }
}

#[test]
fn test_debug_with_performance_metrics() {
    let engine = ObadhEngine::new();
    
    for (input, expected) in get_test_cases().into_iter().take(2) {
        let perf_result = engine.transliterate_with_performance(input);
        
        // Parse JSON
        let parsed: Value = serde_json::from_str(&perf_result).expect("Valid JSON");
        
        // Check main fields
        assert_eq!(parsed["input"], input, "Input should match original text");
        assert_eq!(parsed["output"], expected, "Output should match expected Bengali");
        
        // Check performance data
        assert!(parsed["performance"].is_object(), "Should have performance object");
        assert!(parsed["performance"]["total_ms"].is_number(), "Should have total_ms timing");
        assert!(parsed["performance"]["analysis_ms"].is_number(), "Should have analysis_ms timing");
        
        // Check tokens array
        assert!(parsed["tokens"].is_array(), "Should have tokens array");
        assert!(!parsed["tokens"].as_array().unwrap().is_empty(), "Tokens should not be empty");
        
        // Check phonemes array
        assert!(parsed["phonemes"].is_array(), "Should have phonemes array");
        assert!(!parsed["phonemes"].as_array().unwrap().is_empty(), "Phonemes should not be empty");
        
        // Check syllables array
        assert!(parsed["syllables"].is_array(), "Should have syllables array");
        assert!(!parsed["syllables"].as_array().unwrap().is_empty(), "Syllables should not be empty");
    }
}

#[test]
fn test_batch_transliteration() {
    let engine = ObadhEngine::new();
    let inputs: Vec<&str> = get_test_cases().iter().map(|(input, _)| *input).collect();
    let expected: Vec<&str> = get_test_cases().iter().map(|(_, expected)| *expected).collect();
    
    // Test standard batch
    let results = engine.batch_transliterate(&inputs);
    assert_eq!(results, expected, "Batch results should match expected outputs");
    
    // Test efficient batch
    let efficient_results = engine.batch_transliterate_efficient(&inputs);
    assert_eq!(efficient_results, expected, "Efficient batch results should match expected outputs");
}

#[test]
fn test_batch_with_performance() {
    let engine = ObadhEngine::new();
    let inputs: Vec<&str> = get_test_cases().iter().take(3).map(|(input, _)| *input).collect();
    
    let perf_result = engine.batch_transliterate_with_performance(&inputs);
    
    // Parse JSON
    let parsed: Value = serde_json::from_str(&perf_result).expect("Valid JSON");
    
    // Check batch size
    assert_eq!(parsed["batch_size"], inputs.len(), "Batch size should match input count");
    
    // Check performance data
    assert!(parsed["performance"].is_object(), "Should have performance object");
    assert!(parsed["performance"]["total_ms"].is_number(), "Should have total_ms timing");
    assert!(parsed["performance"]["avg_per_text_ms"].is_number(), "Should have avg_per_text_ms timing");
    
    // Check results array
    assert!(parsed["results"].is_array(), "Should have results array");
    assert_eq!(parsed["results"].as_array().unwrap().len(), inputs.len(), "Results count should match input count");
    
    // Verify each result
    for (i, input) in inputs.iter().enumerate() {
        let result = &parsed["results"][i];
        assert_eq!(result["input"], *input, "Input should match for result {}", i);
        assert!(result["processing_ms"].is_number(), "Should have processing_ms timing");
    }
}

#[test]
fn test_stream_transliteration() {
    let engine = ObadhEngine::new();
    
    // Prepare test input
    let input_text = get_test_cases().iter()
        .map(|(input, _)| input.to_string())
        .collect::<Vec<_>>()
        .join("\n");
    
    let expected_output = get_test_cases().iter()
        .map(|(_, expected)| expected.to_string())
        .collect::<Vec<_>>()
        .join("\n") + "\n"; // Extra newline because writeln adds one
    
    // Set up readers and writers
    let mut reader = BufReader::new(Cursor::new(input_text));
    let mut writer = Cursor::new(Vec::new());
    
    // Process the stream
    engine.transliterate_stream(&mut reader, &mut writer).expect("Stream processing failed");
    
    // Check output
    let mut output = String::new();
    writer.set_position(0);
    writer.read_to_string(&mut output).expect("Failed to read output");
    
    assert_eq!(output, expected_output, "Stream output should match expected");
} 