// ==============================================
// Hour 12: Collections — Vec and HashMap
// ==============================================
// Rust provides rich collection types in the standard library:
// • Vec<T>       → dynamic array (growable list)
// • HashMap<K, V> → hash table (key-value store)
//
// This lab covers:
// • Using `Vec` to store dynamic lists
// • Using `HashMap` to count words and map IDs to names
// • Safe insertion, lookup, and iteration

use std::collections::HashMap;

// ============================================================================
// Example: Word Counter Application
// ----------------------------------------------------------------------------
// Counts how many times each word appears in a given text.
// Uses:
//   - `split_whitespace()` to tokenize
//   - `HashMap<String, u32>` to store counts
// ============================================================================
fn word_counter(text: &str) -> HashMap<String, u32> {
    let mut counts = HashMap::new();

    // Split text into words (handles spaces, tabs, newlines)
    for word in text.split_whitespace() {
        // Normalize to lowercase for case-insensitive counting
        let word = word.to_lowercase();

        // Entry API: efficient way to update or insert
        *counts.entry(word).or_insert(0) += 1;
    }

    counts
}

// ============================================================================
// Exercise: ID-to-Name Map
// ----------------------------------------------------------------------------
// Build a mapping from numeric user IDs to names.
// Supports:
//   - Adding users
//   - Looking up names by ID
//   - Listing all users
// ============================================================================
#[derive(Debug)]
pub struct UserManager {
    // Map user ID (u32) to name (String)
     users:HashMap<u32, String>,  // ← Field name `users` added!
}

impl UserManager {
    /// Creates a new, empty user manager.
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    /// Adds a user with the given ID and name.
    /// Returns `true` if the ID was new, `false` if it replaced an existing user.
    pub fn add_user(&mut self, id: u32, name: String) -> bool {
        self.users.insert(id, name).is_none()
    }

    /// Looks up a user by ID.
    /// Returns `Some(name)` if found, `None` otherwise.
    pub fn get_name(&self, id: u32) -> Option<&String> {
        self.users.get(&id)
    }

    /// Returns a list of all user IDs (as a Vec).
    pub fn all_ids(&self) -> Vec<u32> {
        self.users.keys().copied().collect()
    }

    /// Returns a list of all names (as a Vec of references).
    pub fn all_names(&self) -> Vec<&String> {
        self.users.values().collect()
    }

    /// Returns the total number of users.
    pub fn count(&self) -> usize {
        self.users.len()
    }

    /// Removes a user by ID.
    /// Returns `Some(name)` if removed, `None` if ID didn't exist.
    pub fn remove_user(&mut self, id: u32) -> Option<String> {
        self.users.remove(&id)
    }
}

// ============================================================================
// Main Function — Demo
// ============================================================================
fn main() {
    // -----------------------------------------------------------------------
    // Part 1: Word Counter Example
    // -----------------------------------------------------------------------
    println!("=== Word Counter ===");
    let text = "Rust is great! Rust is fast. Fast and safe.";
    let counts = word_counter(text);

    // Print word counts
    for (word, count) in &counts {
        println!("'{}': {}", word, count);
    }

    // Example: get count of "rust"
    if let Some(&count) = counts.get("rust") {
        println!("The word 'rust' appears {} time(s).", count);
    }

    // -----------------------------------------------------------------------
    // Part 2: ID-to-Name Map Exercise
    // -----------------------------------------------------------------------
    println!("\n=== User ID-to-Name Map ===");
    let mut users = UserManager::new();

    // Add users
    users.add_user(101, "Alice".to_string());
    users.add_user(102, "Bob".to_string());
    users.add_user(103, "Charlie".to_string());

    // Try to add duplicate ID (replaces)
    let is_new = users.add_user(102, "Robert".to_string());
    println!("User 102 was new? {}", is_new); // false — replaced Bob

    // Look up users
    println!("User 101: {:?}", users.get_name(101));   // Some("Alice")
    println!("User 999: {:?}", users.get_name(999));   // None

    // List all data
    println!("Total users: {}", users.count());
    println!("All IDs: {:?}", users.all_ids());
    println!("All names: {:?}", users.all_names());

    // Remove a user
    if let Some(name) = users.remove_user(103) {
        println!("Removed user: {}", name);
    }

    println!("After removal, total users: {}", users.count());
}