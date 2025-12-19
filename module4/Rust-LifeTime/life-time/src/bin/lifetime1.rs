// Basic lifetime example: Comparing string lengths
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    println!("=== Basic Lifetime Example ===\n");
    
    // Example 1: Both strings in same scope
    let string1 = "This is a long string";
    let string2 = "Short";
    let result = longest(string1, string2);
    println!("Longest string: {}", result);
    
    // Example 2: Nested scopes demonstration
    let outer_string = String::from("I live in outer scope");
    let result2;
    
    {
        let inner_string = String::from("Inner scope string");
        result2 = longest(&outer_string, &inner_string);
        println!("Inside inner scope, longest is: {}", result2);
    }
    // result2 can't be used here because inner_string is out of scope
    // But outer_string is still valid
    println!("Outer string is still valid: {}", outer_string);
    
    // Example 3: Using string slices (which are references)
    let text = String::from("Hello, Rust lifetimes!");
    let first_word = &text[0..5];
    let second_word = &text[7..11];
    
    let longest_word = longest(first_word, second_word);
    println!("Longest word between '{}' and '{}' is: '{}'", 
             first_word, second_word, longest_word);
}