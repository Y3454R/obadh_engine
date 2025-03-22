use obadh_engine::ObadhEngine;
use std::collections::HashMap;
use std::env;

// Define test cases by category
fn get_test_cases() -> HashMap<&'static str, Vec<(&'static str, &'static str)>> {
    let mut test_cases = HashMap::new();
    
    // Basic test cases
    test_cases.insert("basic", vec![
        ("bhalo", "ভালো"),
        ("bhalO", "ভালো"),
        ("kok", "কক"),
        ("kOk", "কোক"),
        ("v", "ভ"),
        ("boi", "বই"),
        ("kokko", "কক্ক"),
        ("kokkO", "কক্কো"),
        ("kk", "ক্ক"),
        ("t``", "ৎ"),
        ("kOnnO", "কোন্নো"),
    ]);
    
    // Ya-phala test cases
    test_cases.insert("yaphala", vec![
        ("biddaloy", "বিদ্দালয়"),
        ("bidyaloy", "বিদ্যালয়"),
        ("madhyam", "মাধ্যম"),
        ("adhyapon", "অধ্যাপন"),
        ("byakti", "ব্যক্তি"),
        ("shOmpadyo", "শোম্পাদ্য"),
        ("madhyOm", "মাধ্যোম"),
        ("vidyut``", "বিদ্যুৎ"),
    ]);
    
    // Complex conjuncts test cases
    test_cases.insert("complex", vec![
        ("ShTho", "ষ্ঠ"),
        ("ddho", "দ্ধ"),
        ("tmo", "ত্ম"),
        ("lko", "ল্ক"),
        ("ngo", "ঙ্গ"),
        ("ntro", "ন্ত্র"),
        ("kkhmo", "ক্ষ্ম"),
        ("ttwo", "ত্ত্ব"),
        ("ddo", "দ্ব"),
        ("Tro", "ট্র"),
        ("Dro", "ড্র"),
    ]);
    
    // Bo-fola test cases
    test_cases.insert("bofola", vec![
        ("bishwo", "বিশ্ব"),
        ("dwip", "দ্বীপ"),
        ("atmbiSwas", "আত্মবিশ্বাস"),
        ("dwIp", "দ্বীপ"),
        ("dhwoni", "ধ্বনি"),
        ("anubadwok", "অনুবাদ্বক"),
    ]);
    
    test_cases
}

fn main() {
    let engine = ObadhEngine::new();
    let test_cases = get_test_cases();
    
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let json_output = args.len() > 1 && args[1] == "--json";
    
    // Set specific_category using if-else instead of ternary
    let specific_category = if args.len() > 1 && !args[1].starts_with("--") {
        Some(&args[1])
    } else {
        None
    };
    
    // Set specific_case using if-else instead of ternary
    let specific_case = if args.len() > 2 {
        Some(&args[2])
    } else {
        None
    };
    
    // Run tests and record results
    let mut total_pass = 0;
    let mut total_fail = 0;
    
    for (category, cases) in &test_cases {
        // Skip if a specific category was requested and this isn't it
        if let Some(requested_category) = specific_category {
            if requested_category != category {
                continue;
            }
        }
        
        let mut category_pass = 0;
        let mut category_fail = 0;
        
        if !json_output {
            println!("\n===== {} TEST CASES =====", category.to_uppercase());
        }
        
        for (input, expected) in cases {
            // Skip if a specific case was requested and this isn't it
            if let Some(requested_case) = specific_case {
                if requested_case != input {
                    continue;
                }
            }
            
            let actual = engine.transliterate(input);
            let passed = &actual == expected;
            
            if passed {
                category_pass += 1;
                total_pass += 1;
            } else {
                category_fail += 1;
                total_fail += 1;
            }
            
            if json_output {
                println!("{}", engine.transliterate_json(input));
            } else {
                let status = if passed { "✅" } else { "❌" };
                println!("{} {} => {}", status, input, actual);
                if !passed {
                    println!("   Expected: {}", expected);
                }
            }
        }
        
        if !json_output && (category_pass > 0 || category_fail > 0) {
            println!("Category results: {} passed, {} failed", category_pass, category_fail);
        }
    }
    
    if !json_output {
        println!("\n===== OVERALL RESULTS =====");
        println!("Total passed: {}", total_pass);
        println!("Total failed: {}", total_fail);
        
        if total_fail > 0 {
            std::process::exit(1);
        }
    }
} 