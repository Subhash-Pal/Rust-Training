// src/bin/lab20a.rs
// Lab 20A – Option<T>: Safe Handling of Missing Values

fn main() {
    println!("=== Lab 20A – Option<T> Mastery (Basic) ===\n");

    // Example 1: Parsing user input safely
    println!("--- Example 1: Safe parsing from strings ---");
    let inputs = vec!["42", "hello", "123", "", "0", "-5", "999"];

    for input in inputs {
        let parsed = safe_parse_age(input);
        println!("Input: \"{:<6}\" → {:?}", input, parsed);

        // Step-by-step handling
        match parsed {
            Some(age) => println!("   → Valid age: {} years old", age),
            None => println!("   → Invalid or missing age"),
        }
        println!();
    }

    // Example 2: Chaining Option with combinators
    println!("--- Example 2: Option combinators ---");
    let config_values = vec!["debug=true", "port=8080", "timeout", "log=info"];

    for line in config_values {
        let port = extract_port(line);
        println!("Config line: \"{}\" → Port: {:?}", line, port);

        let effective_port = port.unwrap_or(3000);  // default fallback
        println!("   → Effective port: {}\n", effective_port);
    }

    // Example 3: Real-world find-first pattern
    println!("--- Example 3: Finding first valid config ---");
    let configs = vec!["invalid", "", "port=5000", "port=9000"];
    let first_valid = find_first_port(&configs);
    match first_valid {
        Some(p) => println!("First valid port found: {}", p),
        None => println!("No valid port found in config"),
    }
}

// Safe age parsing – returns Option<u32>
fn safe_parse_age(input: &str) -> Option<u32> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }

    match trimmed.parse::<u32>() {
        Ok(age) if age <= 150 => Some(age),  // reasonable upper limit
        Ok(_) => None,                       // too old to be true
        Err(_) => None,                      // not a number
    }
}

// Extract port from config line like "port=8080"
fn extract_port(line: &str) -> Option<u32> {
    let prefix = "port=";
    if let Some(port_str) = line.strip_prefix(prefix) {
        port_str.parse::<u32>().ok()
    } else {
        None
    }
}

// Find first valid port in a list of config lines
fn find_first_port(lines: &[&str]) -> Option<u32> {
    for &line in lines {
        if let Some(port) = extract_port(line) {
            return Some(port);
        }
    }
    None
}