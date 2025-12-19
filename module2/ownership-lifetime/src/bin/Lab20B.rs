// src/bin/Lab20B.rs
// Lab 20B – Advanced Option Patterns (FINAL – CORRECT & IDIOMATIC)

fn main() {
    println!("=== Lab 20B – Advanced Option Patterns ===\n");

    // ---------------------------------------------------------
    // Example 1: Functional chaining
    // ---------------------------------------------------------
    println!("--- Example 1: Functional chaining ---");
    let inputs = vec!["  123  ", "hello", "   456   ", "", "789"];

    for input in inputs {
        let result = input
            .trim()
            .parse::<u32>()
            .ok()
            .filter(|&n| n > 0)
            .map(|n| n * 2)
            .and_then(|n| if n < 1000 { Some(n) } else { None })
            .unwrap_or(100);

        println!("Input: \"{:<8}\" → Final value: {}", input, result);
    }
    println!();

    // ---------------------------------------------------------
    // Example 2: or_else and unwrap_or_else
    // ---------------------------------------------------------
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

    // ---------------------------------------------------------
    // Example 3: Collect only Some values (FIXED)
    // ---------------------------------------------------------
    println!("--- Example 3: Collect only Some values ---");
    let raw_data = vec!["42", "invalid", "100", "", "25", "abc", "5"];

    let valid_ages: Vec<u32> = raw_data
        .iter()
        .filter_map(|s| safe_parse_age(*s)) // FIX: &&str → &str
        .collect();

    println!("Raw data: {:?}", raw_data);
    println!("Valid ages only: {:?}", valid_ages);
    println!("Total valid: {}", valid_ages.len());
    println!();

    // ---------------------------------------------------------
    // Example 4: Option → Result → Option (NO LEAKS)
    // ---------------------------------------------------------
    println!("--- Example 4: Option to Result and back ---");

    let user_id = get_username_from_config()
        .map(|s| s.trim().to_lowercase())
        .filter(|s| is_valid_username(s))
        .ok_or_else(|| "Invalid or missing username".to_string())
        .and_then(fetch_user_id);

    match user_id {
        Ok(id) => println!("User ID fetched: {}", id),
        Err(e) => println!("Failed to get user ID: {}", e),
    }
}

// ---------------------------------------------------------
// Helpers
// ---------------------------------------------------------

// Safe parse age
fn safe_parse_age(s: &str) -> Option<u32> {
    s.trim()
        .parse::<u32>()
        .ok()
        .filter(|&age| age <= 150)
}

// Extract port from "port=XXXX"
fn extract_port(line: &str) -> Option<u32> {
    let (key, value) = line.split_once('=')?;
    if key == "port" {
        value.trim().parse::<u32>().ok()
    } else {
        None
    }
}

// Simulate config lookup
fn get_username_from_config() -> Option<String> {
    Some("   alice_42  ".to_string())
}

// Validate username (NO allocations leaked)
fn is_valid_username(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_')
}

// Simulate database lookup
fn fetch_user_id(username: String) -> Result<u64, String> {
    match username.as_str() {
        "alice_42" => Ok(1001),
        "bob" => Ok(1002),
        _ => Err(format!("User not found: {}", username)),
    }
}
