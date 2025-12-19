
/*
```
String slices (&str)
Array and vector slices (&[T])
A function to extract words from a string
An interactive slice-viewer for arrays (as an exercise)
```
*/
// Function to extract words from a string slice
fn extract_words(sentence: &str) -> Vec<&str> {
    // Split by whitespace and filter out empty strings
    sentence.split_whitespace().collect()
}

// Generic slice viewer: safely prints a subslice [start..end)
fn slice_viewer<T: std::fmt::Debug>(arr: &[T], start: isize, end: isize) {
    println!("Original array: {:?}", arr);

    let len = arr.len() as isize;

    // Clamp start and end to valid bounds
    let start = if start < 0 { 0 } else { start }.min(len);
    let end = if end > len { len } else { end }.max(start); // ensure end >= start

    if start >= len || end <= 0 {
        println!("Requested slice is out of bounds → []");
        return;
    }

    // Safe conversion back to usize (we clamped above)
    let start_usize = start as usize;
    let end_usize = end as usize;

    let sub_slice = &arr[start_usize..end_usize];
    println!("Slice[{}..{}] = {:?}", start_usize, end_usize, sub_slice);
}

fn main() {
    // ===== Part 1: String Slices =====
    println!("=== String Slices ===");
    let text = "  The   quick brown fox  ";
    println!("Original string: {:?}", text);

    // Extract words
    let words = extract_words(text);
    println!("Extracted words: {:?}", words);

    // Manual string slicing (byte-based! beware with non-ASCII)
    // For this ASCII string, it's safe
    println!("First 5 bytes as str: {:?}", &text[0..5]);
    println!("Substring 'quick': {:?}", &text[6..11]);

    // ===== Part 2: Array/Vector Slice Viewer =====
    println!("\n=== Array Slice Viewer ===");

    let numbers = [10, 20, 30, 40, 50, 60, 70];
    let test_cases = [
        (0, 3),
        (2, 5),
        (-1, 4),
        (3, 20),
        (10, 15),
        (4, 4),
    ];

    for (start, end) in test_cases {
        slice_viewer(&numbers, start, end);
        println!("---");
    }

    // Example with vector of strings
    println!("String array example:");
    let fruits = vec!["apple", "banana", "cherry", "date", "elderberry"];
    slice_viewer(&fruits, 1, 4);
}

/*
```
Key Rust Concepts Covered
Concept
Explanation
String slices (&str)   |  Immutable view into string data. &text[start..end] accesses a substring by byte index (safe only for ASCII or when aligned to UTF-8 boundaries).
Array slices (&[T])    |  View into an array or Vec<T>. Created with &arr[start..end].
split_whitespace()     |  Rust’s built-in method to split a &str on any Unicode whitespace and skip empty entries — perfect for word extraction.
Bounds safety          |  Rust panics on out-of-bounds slicing. This code clamps indices to prevent panics, mimicking a "safe viewer".
Generics               |  slice_viewer<T> works with any type that implements Debug (so it can be printed).
```
*/