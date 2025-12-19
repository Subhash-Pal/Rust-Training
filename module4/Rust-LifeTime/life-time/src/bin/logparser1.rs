/*
RUST LIFETIME ELISION DEMONSTRATION - UPDATED VERSION

This code demonstrates Rust's lifetime elision rules and nested lifetime relationships.
Updated to fix the "hiding a lifetime that's elided elsewhere" warning.

Key Concepts:
1. Lifetime annotations in structs and functions
2. Nested lifetimes (Parser has two independent lifetimes)
3. Lifetime elision in function signatures
4. Borrowing chains and memory safety
*/

// ============================================
// STRUCT DEFINITIONS
// ============================================

/// Context struct holds a borrowed string with lifetime 's
/// This means Context cannot outlive the string it references
struct Context<'s>(&'s str);

/// Parser struct demonstrates nested lifetimes:
/// - 'c: lifetime for the reference to Context
/// - 's: lifetime for the string inside Context (passed through)
/// The Parser borrows a Context which in turn borrows a string
struct Parser<'c, 's> {
    context: &'c Context<'s>,
}

// ============================================
// PARSER IMPLEMENTATION
// ============================================

impl<'c, 's> Parser<'c, 's> {
    /// Parse method returns a Result with a string slice
    /// The returned slice has lifetime 's, matching the original string
    /// This ensures the slice doesn't outlive the data it references
    fn parse(&self) -> Result<(), &'s str> {
        // Return an error with a substring starting from index 1
        // This borrows from the original string with lifetime 's
        Err(&self.context.0[1..])
    }
}

// ============================================
// FUNCTIONS WITH LIFETIME ELISION - UPDATED
// ============================================

/// parse_context function demonstrates lifetime elision
/// Using `'_` to indicate an elided lifetime makes it consistent
/// 
/// Options (all are valid and equivalent):
/// 1. fn parse_context<'s>(context: Context<'s>) -> Result<(), &'s str>
/// 2. fn parse_context(context: Context<'_>) -> Result<(), &'_ str>
/// 3. fn parse_context(context: Context<'_>) -> Result<(), &str> (elided)
/// 
/// We're using option 3 which is the most common and readable
fn parse_context(context: Context<'_>) -> Result<(), &str> {
    // Create a Parser that borrows the context
    // The Parser's lifetime 'c is the scope of this function
    // The string lifetime 's comes from the Context
    Parser { context: &context }.parse()
}

// Alternative explicit version (also valid, no warnings):
/*
fn parse_context_explicit<'s>(context: Context<'s>) -> Result<(), &'s str> {
    Parser { context: &context }.parse()
}
*/

// ============================================
// MAIN FUNCTION - DEMONSTRATION
// ============================================

fn main() {
    println!("=== RUST LIFETIME ELISION DEMONSTRATION ===\n");
    
    // Create an owned String
    let data = String::from("Hello, World!");
    println!("1. Created String: '{}'", data);
    
    // Create Context borrowing from the String
    let context = Context(&data);
    println!("2. Created Context borrowing the string");
    
    // Call parse_context, moving the Context into it
    println!("3. Calling parse_context (Context is moved)");
    let result = parse_context(context);
    
    // Note: 'context' is now moved and cannot be used
    // But 'data' is still valid because Context only borrowed it
    
    // Process the result
    match result {
        Ok(_) => println!("4. Result: Parsing succeeded"),
        Err(slice) => println!("4. Result: Error with sliced string '{}'", slice),
    }
    
    // Demonstrate that the original data is still accessible
    println!("5. Original string still valid: '{}'", data);
    
    // ============================================
    // ADDITIONAL DEMONSTRATIONS
    // ============================================
    
    println!("\n=== ADDITIONAL LIFETIME TESTS ===");
    
    // Test 1: Nested scopes
    println!("\nTest 1: Nested scope demonstration");
    let outer_data = String::from("Rust Programming");
    let result;
    
    {
        // Create context in inner scope
        let inner_context = Context(&outer_data);
        result = parse_context(inner_context);
        // inner_context is dropped here, but the string slice in result
        // still references outer_data which lives longer
    } // inner_context dropped
    
    match result {
        Err(s) => println!("   Result from inner scope: '{}'", s),
        _ => println!("   Unexpected result"),
    }
    
    // outer_data still valid
    println!("   Outer data still exists: '{}'", outer_data);
    


    /*

    // Test 2: Direct Parser usage
    println!("\nTest 2: Direct Parser usage");
    let test_string = String::from("Lifetime Test");
    let test_context = Context(&test_string);
    let parser = Parser { context: &test_context };
    
    match parser.parse() {
        Err(s) => println!("   Direct parse result: '{}'", s),
        _ => println!("   Unexpected"),
    }
    
    // Test 3: Multiple calls
    println!("\nTest 3: Multiple parse calls");
    let multi_data = String::from("ABCDEF");
    let multi_context = Context(&multi_data);
    let parser1 = Parser { context: &multi_context };
    let parser2 = Parser { context: &multi_context };
    
    println!("   First parse: {:?}", parser1.parse());
    println!("   Second parse: {:?}", parser2.parse());
    
    // ============================================
    // EXPLANATION OUTPUT
    // ============================================
    
    println!("\n=== LIFETIME EXPLANATION ===");
    println!("Lifetime relationships in this code:");
    println!("1. String ('data') has its own lifetime");
    println!("2. Context<'s> borrows from String with lifetime 's");
    println!("3. Parser<'c, 's> has two lifetimes:");
    println!("   - 'c: how long Parser borrows Context");
    println!("   - 's: how long Context borrows String (passed through)");
    println!("4. parse_context() uses lifetime elision with '_':");
    println!("   - Input: Context<'_> - anonymous lifetime");
    println!("   - Output: Result<(), &str> (inferred by compiler)");
    println!("5. The '_' tells Rust: 'infer the lifetime here'");
    println!("6. Memory safety: All references valid for their lifetime");
    println!("7. The returned string slice cannot outlive the original String");
    
    // ============================================
    // DEMONSTRATING THE WARNING
    // ============================================
    
    println!("\n=== THE WARNING EXPLAINED ===");
    println!("The original code had:");
    println!("  fn parse_context(context: Context) -> Result<(), &str>");
    println!("This is confusing because:");
    println!("1. 'Context' hides its lifetime parameter");
    println!("2. '&str' elides its lifetime parameter");
    println!("3. They refer to the same lifetime, but written differently");
    println!("\nThe fix is:");
    println!("  fn parse_context(context: Context<'_>) -> Result<(), &str>");
    println!("Now both use consistent elision syntax!");

    */

}

// ============================================
// DIFFERENT WAYS TO WRITE THE SAME FUNCTION
// ============================================

/*
// Version 1: Full explicit (no elision)
fn parse_context_explicit<'s>(context: Context<'s>) -> Result<(), &'s str> {
    Parser { context: &context }.parse()
}

// Version 2: Mixed (what caused the warning)
// fn parse_context_mixed(context: Context) -> Result<(), &str> {
//     Parser { context: &context }.parse()
// }

// Version 3: Consistent elision (recommended)
fn parse_context_elided(context: Context<'_>) -> Result<(), &str> {
    Parser { context: &context }.parse()
}

// Version 4: Both with '_'
fn parse_context_both_underscore(context: Context<'_>) -> Result<(), &'_ str> {
    Parser { context: &context }.parse()
}
*/

