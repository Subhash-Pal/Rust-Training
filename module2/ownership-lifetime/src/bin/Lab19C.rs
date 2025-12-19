// src/bin/Lab19C.rs
// Lab 19C – Production Error Handling with anyhow & thiserror (CORRECT)

use anyhow::{Context, Result};
use std::{fs, path::PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
enum ConfigError {
    #[error("Invalid value: {key} = {value}")]
    InvalidValue { key: String, value: String },
}

fn main() -> Result<()> {
    println!("=== Lab 19C – Professional Error Handling ===\n");

    // ✅ Always resolve files from project root
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let config_path = root.join("config.toml");
    let user_data_path = root.join("user_data.txt");

    // ✅ Ensure demo files exist and are NON-empty
    prepare_demo_files(&config_path, &user_data_path)?;

    // -------- Example 1 --------
    println!("--- Example 1: Loading config.toml ---");
    match load_config(&config_path) {
        Ok(cfg) => {
            println!("Config loaded successfully:");
            println!("{}", cfg);
        }
        Err(e) => {
            println!("Failed to load config:");
            println!("{:#}", e);
        }
    }
    println!();

    // -------- Example 2 --------
    println!("--- Example 2: Processing user_data.txt ---");
    match process_user_data(&user_data_path) {
        Ok(lines) => println!("User data processed successfully ({} lines)", lines),
        Err(e) => {
            println!("Failed to process user data:");
            println!("{:#}", e);
        }
    }
    println!();

    // -------- Example 3 --------
    println!("--- Example 3: Intentional failure ---");
    match intentional_failure() {
        Ok(_) => unreachable!(),
        Err(e) => {
            println!("Expected error:");
            println!("{:#}", e);
        }
    }

    Ok(())
}

// ------------------------------------------------------

fn prepare_demo_files(config: &PathBuf, user_data: &PathBuf) -> Result<()> {
    if !config.exists() {
        fs::write(
            config,
            r#"
debug = true
app_name = "ownership-lab"
"#,
        )
        .context("Failed to create config.toml")?;
    }

    if !user_data.exists() || fs::read_to_string(user_data)?.trim().is_empty() {
        fs::write(
            user_data,
            "Alice\nBob\nCharlie\n",
        )
        .context("Failed to create user_data.txt")?;
    }

    Ok(())
}

// ------------------------------------------------------

fn load_config(path: &PathBuf) -> Result<String> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("File not found or cannot be read: {}", path.display()))?;

    validate_config(&content)
        .with_context(|| "Failed to validate config")?;

    Ok(content.trim().to_string())
}

fn validate_config(content: &str) -> Result<(), ConfigError> {
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("debug =") {
            let value = line.split('=').nth(1).unwrap().trim();
            if value == "false" {
                return Err(ConfigError::InvalidValue {
                    key: "debug".into(),
                    value: value.into(),
                });
            }
        }
    }
    Ok(())
}

// ------------------------------------------------------

fn process_user_data(path: &PathBuf) -> Result<usize> {
    let data = fs::read_to_string(path)
        .with_context(|| format!("User data file not found: {}", path.display()))?;

    let count = data.lines().filter(|l| !l.trim().is_empty()).count();

    if count == 0 {
        anyhow::bail!("User data file is empty");
    }

    Ok(count)
}

// ------------------------------------------------------

fn intentional_failure() -> Result<()> {
    fs::read_to_string("this_file_does_not_exist.txt")
        .with_context(|| "This file intentionally does not exist")?;
    Ok(())
}

```
Step-by-Step Learning Points








































StepConceptWhat You Saw1Option<T> basicsSome(value) or None2Safe parsingNo panic on invalid input3match on OptionExplicit handling4Combinators (unwrap_or, ok())Clean defaults5Chaining with if let and strip_prefixReal-world pattern6Early return with ? not used(saved for Result – Option has no ?)
Key Takeaways

Option<T> = safe way to represent "missing" or "invalid" values
Never use null in Rust — use Option
Always handle None explicitly or provide safe defaults
unwrap() panics — avoid in production (we didn't use it!)

You now handle missing values safely like a pro.
Ready for Lab 20B – Advanced Option Patterns (map, and_then, filter, or_else, collect)?
Just say next! 🚀9.9sok nextLab 20B (Intermediate)
Title: Advanced Option Patterns – Combinators & Chaining
Focus:

Functional style with map, and_then, or_else, filter, unwrap_or_else
Converting between Option and Result
Collecting Options safely
Real-world config parsing chain

Rust// src/bin/lab20b.rs
// Lab 20B – Advanced Option Combinators & Chaining

fn main() {
    println!("=== Lab 20B – Advanced Option Patterns ===\n");

    // Example 1: Chaining operations with map/and_then
    println!("--- Example 1: Functional chaining ---");
    let inputs = vec!["  123  ", "hello", "   456   ", "", "789"];

    for input in inputs {
        let result = input
            .trim()
            .parse::<u32>()
            .ok()                                          // Option<u32>
            .filter(|&n| n > 0)                            // remove 0 or negative
            .map(|n| n * 2)                                // double it
            .and_then(|n| if n < 1000 { Some(n) } else { None })  // upper limit
            .unwrap_or(100);                               // default fallback

        println!("Input: \"{:<8}\" → Final value: {}", input, result);
    }
    println!();

    // Example 2: or_else and unwrap_or_else (lazy defaults)
    println!("--- Example 2: Lazy defaults with or_else ---");
    let configs = vec!["port=8080", "server=prod", "invalid", "port=9000"];

    for line in configs {
        let port = extract_port(line)
            .or_else(|| {
                println!("   → No port in '{}', trying fallback", line);
                Some(3000)
            })
            .unwrap_or_else(|| {
                println!("   → Still no port – using hard default");
                8080
            });

        println!("Line: \"{}\" → Port: {}", line, port);
    }
    println!();

    // Example 3: Collecting only valid values
    println!("--- Example 3: Collect only Some values ---");
    let raw_data = vec!["42", "invalid", "100", "", "25", "abc", "5"];

    let valid_ages: Vec<u32> = raw_data
        .iter()
        .filter_map(|s| safe_parse_age(s))   // filter_map = map + filter None
        .collect();

    println!("Raw data: {:?}", raw_data);
    println!("Valid ages only: {:?}", valid_ages);
    println!("Total valid: {}", valid_ages.len());
    println!();

    // Example 4: Option ↔ Result conversion
    println!("--- Example 4: Option to Result and back ---");
    let username = get_username_from_config();
    let user_id = username
        .as_deref()
        .and_then(validate_username)
        .ok_or("Invalid or missing username")
        .and_then(fetch_user_id);

    match user_id {
        Ok(id) => println!("User ID fetched: {}", id),
        Err(e) => println!("Failed to get user ID: {}", e),
    }
}

// Safe parse with filter
fn safe_parse_age(s: &str) -> Option<u32> {
    s.trim()
        .parse::<u32>()
        .ok()
        .filter(|&age| age <= 150)
}

// Extract port from line
fn extract_port(line: &str) -> Option<u32> {
    line.split('=')
        .nth(1)?
        .trim()
        .parse::<u32>()
        .ok()
}

// Simulate config lookup
fn get_username_from_config() -> Option<String> {
    Some("   alice_42  ".to_string())
}

// Validate username
fn validate_username(name: &str) -> Option<String> {
    let cleaned = name.trim().to_lowercase();
    if cleaned.chars().all(|c| c.is_alphanumeric() || c == '_') && !cleaned.is_empty() {
        Some(cleaned)
    } else {
        None
    }
}

// Simulate database lookup
fn fetch_user_id(username: &str) -> Result<u64, String> {
    match username {
        "alice_42" => Ok(1001),
        "bob" => Ok(1002),
        _ => Err(format!("User not found: {}", username)),
    }
}