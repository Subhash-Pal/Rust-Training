// src/bin/lab19b.rs
// Lab 19B – Advanced Result Patterns (FULL CLEAN CODE)

use std::fs::File;
use std::io::{self, Read};

// Custom error type with #[derive(Debug)]
#[allow(dead_code)]  // ← This silences the harmless "field never read" warnings
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

    // Example 1: Clean file reading with ?
    println!("--- Example 1: File reading with ? ---");
    match read_config("config.txt") {
        Ok(config) => println!("Config loaded successfully:\n{}", config),
        Err(e) => println!("Failed to load config: {:?}", e),
    }
    println!();

    // Example 2: Result combinators
    println!("--- Example 2: Result combinators ---");
    let inputs = [Some(10), None, Some(30), None, Some(50)];

    for &opt in &inputs {
        let result: Result<f64, String> = opt
            .ok_or("Missing value!".to_string())
            .and_then(|v| safe_sqrt(v as f64))
            .map(|sqrt_val| sqrt_val * 2.0)
            .or_else(|e| {
                println!("   → Handled error: {}", e);
                Ok(0.0)
            });

        println!("Input {:?} → {:?}", opt, result);
    }
    println!();

    // Example 3: Parsing and summing with ?
    println!("--- Example 3: Parsing and summing with ? ---");
    let numbers = vec!["10", "abc", "25", "invalid", "100"];
    match parse_and_sum(&numbers) {
        Ok(sum) => println!("Successfully parsed and summed: {}", sum),
        Err(e) => println!("Parsing failed: {:?}", e),
    }
}

// Clean error propagation with ?
fn read_config(path: &str) -> Result<String, AppError> {
    let mut file = File::open(path)?;  // ? automatically converts io::Error → AppError
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Safe square root
fn safe_sqrt(x: f64) -> Result<f64, String> {
    if x < 0.0 {
        Err(format!("Cannot take sqrt of negative number: {}", x))
    } else {
        Ok(x.sqrt())
    }
}

// Full parsing chain with custom errors
fn parse_and_sum(strings: &[&str]) -> Result<i32, AppError> {
    let mut sum = 0;
    for &s in strings {
        let num: i32 = s
            .parse()
            .map_err(|_| AppError::Parse(s.to_string()))?;
        if num < 0 {
            return Err(AppError::InvalidValue(format!("Negative numbers not allowed: {}", num)));
        }
        sum += num;
    }
    Ok(sum)
}