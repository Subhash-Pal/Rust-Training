// ==============================================
// Hour 14: String Handling Deep Dive
// ==============================================
// Rust strings are UTF-8 encoded by default:
// • `String` = owned, growable UTF-8 string
// • `&str` = borrowed string slice (guaranteed valid UTF-8)
//
// ⚠️ Critical: Never slice by byte index unless you know it's a UTF-8 boundary!
//   - Use `.chars()`, `.split_whitespace()`, `.lines()` for safe processing.

// ============================================================================
// Example: Reverse Words Safely
// Reverses word order in a sentence without breaking Unicode.
// ============================================================================
fn reverse_words(sentence: &str) -> String {
    let words: Vec<&str> = sentence.split_whitespace().collect();
    let reversed: Vec<&str> = words.into_iter().rev().collect();
    reversed.join(" ")
}

// ============================================================================
// Exercise: Basic Text Processor
// ============================================================================
#[derive(Debug)]
pub struct TextStats {
    line_count: usize,
    word_count: usize,
    char_count: usize,
    palindromes: Vec<String>,
}

impl TextStats {
    /// Analyze text and return statistics.
    pub fn from_text(text: &str) -> Self {
        let line_count = text.lines().count();

        let words: Vec<&str> = text.split_whitespace().collect();
        let word_count = words.len();

        let char_count = text.chars().count();

        let palindromes: Vec<String> = words
            .into_iter()
            .filter(|word| is_palindrome(word))
            .map(|s| s.to_lowercase())
            .collect();

        Self {
            line_count,
            word_count,
            char_count,
            palindromes,
        }
    }
}

/// Check if a word is a palindrome (case-insensitive, letters only).
fn is_palindrome(word: &str) -> bool {
    let clean: String = word
        .chars()
        .filter(|c| c.is_alphabetic())
        .flat_map(|c| c.to_lowercase())
        .collect();

    if clean.is_empty() {
        return false;
    }

    clean.chars().eq(clean.chars().rev())
}

// ============================================================================
// Demonstration of UTF-8 Safety
// ============================================================================
fn demo_utf8_slicing() {
    let s = "café 🦀";

    println!("=== UTF-8 Slicing Safety Demo ===");
    println!("Text: {:?}", s);
    println!("Byte length: {} bytes", s.len());
    println!("Character count: {} chars", s.chars().count());

    println!("\nCharacters with indices:");
    for (i, c) in s.chars().enumerate() {
        println!("  Index {}: '{}'", i, c);
    }

    // Safe slicing using .get()
    println!("\nTrying to slice [0..4]:");
    match s.get(0..4) {
        Some(slice) => println!("  Result: {:?}", slice),
        None => println!("  ❌ Invalid UTF-8 boundary!"),
    }

    println!("Trying to slice [0..5]:");
    match s.get(0..5) {
        Some(slice) => println!("  Result: {:?}", slice),
        None => println!("  ❌ Invalid UTF-8 boundary!"),
    }
}

// ============================================================================
// Main Function
// ============================================================================
fn main() {
    // -----------------------------------------------------------------------
    // Part 1: Reverse Words Example
    // -----------------------------------------------------------------------
    println!("=== Reverse Words Safely ===");
    let sentence = "Hello Rust 🦀 world!";
    println!("Original: {}", sentence);
    println!("Reversed: {}", reverse_words(sentence));

    let unicode_sentence = "café naïve résumé";
    println!("\nUnicode test:");
    println!("Original: {}", unicode_sentence);
    println!("Reversed: {}", reverse_words(unicode_sentence));

    // -----------------------------------------------------------------------
    // Part 2: UTF-8 Safety Demo
    // -----------------------------------------------------------------------
    demo_utf8_slicing();

    // -----------------------------------------------------------------------
    // Part 3: Text Processor Exercise
    // -----------------------------------------------------------------------
    println!("\n=== Basic Text Processor ===");

    let sample_text =
        "Was it a car or a cat I saw?\n\
         Madam, I'm Adam.\n\
         Rust is awesome!";

    let stats = TextStats::from_text(sample_text);

    println!("Input text:\n{}\n", sample_text);
    println!("Statistics:");
    println!("  Lines: {}", stats.line_count);
    println!("  Words: {}", stats.word_count);
    println!("  Characters (Unicode): {}", stats.char_count);
    println!("  Palindromic words: {:?}", stats.palindromes);
}
