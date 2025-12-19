// src/bin/lab19b.rs
// Lab 19B – Advanced Result: ? operator, combinators, custom errors

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

// Step 1: Define a custom error type (real-world style)
#[derive(Debug)]
enum AppError {
    Io(io::Error),
    Parse(String),
    InvalidValue(String),
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::Io(err)
    }
}

fn main() {
    println!("=== Lab 19B – Advanced Result Patterns ===\n");

    // Example 1: Clean error propagation with ?
    println!("--- Example 1: Clean file reading with ? ---");
    let config_path = "config.txt";
    match read_config(config_path) {
        Ok(config) => println!("Config loaded successfully:\n{}", config),
        Err(e) => println!("Failed to load config: {:?}", e),
    }
    println!();

    // Example 2: Using combinators (map, and_then, unwrap_or_else)
    println!("--- Example 2: Result combinators ---");
    let values = [Some(10), None, Some(30), None, Some(50)];

    for &opt in &values {
        let result = opt
            .ok_or("Missing value!")
            .and_then(|v| safe_sqrt(v as f64))
            .map(|sqrt| sqrt * 2.0)
            .or_else(|e| {
                println!("   → Error: {}", e);
                Ok(0.0)  // fallback value
            });
        println!("Processing {:?} → {:?}", opt, result);
    }
    println!();

    // Example 3: Chaining operations with ?
    println!("--- Example 3: Chaining with ? ---");
    let numbers = vec!["10", "abc", "25", "not_a_number", "100"];
    match parse_and_sum(numbers) {
        Ok(sum) => println!("Successfully parsed and summed: {}", sum),
        Err(e) => println!("Parsing failed: {:?}", e),
    }
}

// Step 1: Function using ? operator – super clean!
fn read_config(path: &str) -> Result<String, AppError> {
    let mut file = File::open(path)?;  // ? propagates io::Error as AppError
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;  // another ?
    Ok(contents)
}

// Step 2: Safe square root that returns Result
fn safe_sqrt(x: f64) -> Result<f64, String> {
    if x < 0.0 {
        Err(format!("Cannot take sqrt of negative number: {}", x))
    } else {
        Ok(x.sqrt())
    }
}

// Step 3: Full chain with ? and custom error handling
fn parse_and_sum(strings: Vec<&str>) -> Result<i32, AppError> {
    let mut sum = 0;
    for s in strings {
        let num: i32 = s.parse().map_err(|_| AppError::Parse(s.to_string()))?;
        if num < 0 {
            return Err(AppError::InvalidValue(format!("Negative numbers not allowed: {}", num)));
        }
        sum += num;
    }
    Ok(sum)
}