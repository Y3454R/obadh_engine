use std::io::{self, Read};
use std::env;
use std::time::{Instant, Duration};
use serde_json::{json, Value};
use clap::{Command, Arg, ArgAction};

use obadh_engine::engine::{Transliterator, TokenType};

// Single source of version - using the crate version from Cargo.toml
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create CLI with clap
    let matches = Command::new("obadh")
        .version(VERSION)
        .about("A Bengali transliteration engine using Avro Phonetic rules")
        .arg(
            Arg::new("INPUT")
                .help("Input text to transliterate")
                .index(1)
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Output detailed information in JSON format")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Output more detailed information in JSON format")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("benchmark")
                .short('b')
                .long("benchmark")
                .help("Run benchmark with N iterations")
                .num_args(0..=1)
                .default_missing_value("1")
                .value_parser(clap::value_parser!(usize))
        )
        .arg(
            Arg::new("pretty")
                .short('p')
                .long("pretty")
                .help("Pretty-print the JSON output (only used with --debug or --verbose)")
                .action(ArgAction::SetTrue)
        )
        .get_matches();

    // Get command line flags
    let debug_mode = matches.get_flag("debug");
    let verbose_mode = matches.get_flag("verbose");
    let pretty_print = matches.get_flag("pretty");
    let benchmark_iterations = matches.get_one::<usize>("benchmark").copied();

    // Get the input text from arguments or stdin
    let input = if let Some(text) = matches.get_one::<String>("INPUT") {
        text.clone()
    } else {
        // Try to read from stdin
        let mut buffer = String::new();
        let bytes_read = io::stdin().read_to_string(&mut buffer)?;
        
        if bytes_read == 0 {
            // No input provided, show usage
            let _ = Command::new("obadh")
                .version(VERSION)
                .about("A Bengali transliteration engine using Avro Phonetic rules")
                .print_help();
            println!();
            return Ok(());
        }
        
        buffer
    };

    // Initialize the transliterator
    let transliterator = Transliterator::new();
    
    // Process based on the flags
    if let Some(iterations) = benchmark_iterations {
        // Benchmark mode
        benchmark(&transliterator, &input, iterations, debug_mode || verbose_mode, pretty_print)
    } else if debug_mode || verbose_mode {
        // Debug/verbose mode with JSON output
        process_json_output(&transliterator, &input, verbose_mode, pretty_print)
    } else {
        // Default mode: Simple output with just the transliterated text
        let result = transliterator.transliterate(&input);
        println!("{}", result);
        Ok(())
    }
}

/// Process text with JSON output for debug/verbose mode
fn process_json_output(
    transliterator: &Transliterator, 
    input: &str, 
    verbose: bool, 
    pretty_print: bool
) -> Result<(), Box<dyn std::error::Error>> {
    // Measure sanitization performance
    let sanitize_start = Instant::now();
    let _sanitized = transliterator.sanitize(input).unwrap_or_else(|_| input.to_string());
    let sanitize_duration = sanitize_start.elapsed();
    
    // Measure tokenization performance
    let tokenize_start = Instant::now();
    let tokens = transliterator.tokenize(input);
    let tokenize_duration = tokenize_start.elapsed();
    
    // Measure transliteration performance
    let transliterate_start = Instant::now();
    let transliterated = transliterator.transliterate(input);
    let transliterate_duration = transliterate_start.elapsed();
    
    // Create JSON output
    let mut output_json = json!({
        "input": input,
        "output": transliterated,
        "performance": {
            "total_ms": format_duration(sanitize_duration + tokenize_duration + transliterate_duration),
            "sanitize_ms": format_duration(sanitize_duration),
            "tokenize_ms": format_duration(tokenize_duration),
            "transliterate_ms": format_duration(transliterate_duration),
        }
    });
    
    // Add token analysis for verbose mode
    if verbose {
        if let Value::Object(ref mut map) = output_json {
            // Convert tokens to JSON structure with detailed analysis
            let token_analysis = tokens.iter().map(|token| {
                let mut token_json = json!({
                    "content": token.content,
                    "type": format!("{:?}", token.token_type),
                    "position": token.position
                });
                
                // If it's a word, include phonetic analysis
                if token.token_type == TokenType::Word {
                    let phonetic_units = transliterator.tokenize_phonetic(&token.content);
                    let units_json = phonetic_units.iter().map(|unit| {
                        json!({
                            "text": unit.text,
                            "type": format!("{:?}", unit.unit_type),
                            "position": unit.position
                        })
                    }).collect::<Vec<_>>();
                    
                    if let Value::Object(ref mut token_map) = token_json {
                        token_map.insert("phonetic_units".to_string(), json!(units_json));
                        
                        // Add the transliterated form of this word
                        let word_transliterated = transliterator.transliterate(&token.content);
                        token_map.insert("transliterated".to_string(), json!(word_transliterated));
                    }
                }
                
                token_json
            }).collect::<Vec<_>>();
            
            map.insert("token_analysis".to_string(), json!(token_analysis));
        }
    }
    
    // Output the result
    if pretty_print {
        println!("{}", serde_json::to_string_pretty(&output_json)?);
    } else {
        println!("{}", serde_json::to_string(&output_json)?);
    }
    
    Ok(())
}

/// Format Duration to milliseconds with decimal precision
fn format_duration(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1000.0
}

/// Run benchmark with multiple iterations
fn benchmark(
    transliterator: &Transliterator, 
    input: &str, 
    iterations: usize, 
    json_output: bool,
    pretty_print: bool
) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize timing variables
    let mut total_duration = Duration::new(0, 0);
    let mut sanitize_duration = Duration::new(0, 0);
    let mut tokenize_duration = Duration::new(0, 0);
    let mut transliterate_duration = Duration::new(0, 0);
    
    // Run the benchmark
    for _ in 0..iterations {
        // Measure sanitization
        let start = Instant::now();
        let _ = transliterator.sanitize(input);
        sanitize_duration += start.elapsed();
        
        // Measure tokenization
        let start = Instant::now();
        let _ = transliterator.tokenize(input);
        tokenize_duration += start.elapsed();
        
        // Measure transliteration
        let start = Instant::now();
        let _ = transliterator.transliterate(input);
        transliterate_duration += start.elapsed();
        
        // Total time
        total_duration += sanitize_duration + tokenize_duration + transliterate_duration;
    }
    
    // Calculate averages
    let avg_total = total_duration / iterations as u32;
    let avg_sanitize = sanitize_duration / iterations as u32;
    let avg_tokenize = tokenize_duration / iterations as u32;
    let avg_transliterate = transliterate_duration / iterations as u32;
    
    // Output benchmark results
    let transliterated = transliterator.transliterate(input);
    
    if json_output {
        // JSON output for benchmark results
        let benchmark_json = json!({
            "input": input,
            "output": transliterated,
            "benchmark": {
                "iterations": iterations,
                "avg_total_ms": format_duration(avg_total),
                "avg_sanitize_ms": format_duration(avg_sanitize),
                "avg_tokenize_ms": format_duration(avg_tokenize),
                "avg_transliterate_ms": format_duration(avg_transliterate),
                "total_run_time_ms": format_duration(total_duration),
            }
        });
        
        if pretty_print {
            println!("{}", serde_json::to_string_pretty(&benchmark_json)?);
        } else {
            println!("{}", serde_json::to_string(&benchmark_json)?);
        }
    } else {
        // Simple text output for benchmark results
        println!("Translation: {}", transliterated);
        println!("Benchmark results ({} iterations):", iterations);
        println!("  Average total time: {:.4} ms", format_duration(avg_total));
        println!("  Average sanitize time: {:.4} ms", format_duration(avg_sanitize));
        println!("  Average tokenize time: {:.4} ms", format_duration(avg_tokenize));
        println!("  Average transliterate time: {:.4} ms", format_duration(avg_transliterate));
        println!("  Total run time: {:.4} ms", format_duration(total_duration));
    }
    
    Ok(())
} 