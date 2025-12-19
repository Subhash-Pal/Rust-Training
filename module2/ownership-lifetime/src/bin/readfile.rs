//  Concept

// Rust's ownership system controls how data moves, borrows, and gets freed.
// Real-world ownership patterns appear when:

// Returning large objects without copying

// Passing references safely

// Using borrowed vs owned data in APIs

// Why it matters

// C++ developers often rely on raw pointers, smart pointers, or move constructors.
// Rust enforces ownership through the compiler:

// One owner at a time

// Data moved by default

// Explicit references &T / &mut T

// 📘 Example: File Reader That Safely Returns Data
// read_file() returns owned String (safe + efficient)
 


use std::fs;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let contents = fs::read_to_string(path)?; // ownership of String
    Ok(contents) // moved to caller
}

fn main() {
    match read_file("./sample.txt") {
        Ok(text) => println!("File content:\n{}", text),
        Err(err) => println!("Error: {}", err),
    }
}


// Why this is good ownership

// The function owns the string

// Returns it to the caller without cloning

// Caller now becomes the owner