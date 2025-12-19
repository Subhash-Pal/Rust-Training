// src/bin/lab17b.rs
// Lab 17B – The 3 Lifetime Elision Rules (WORKS EVERYWHERE)

fn main() {
    println!("=== Lab 17B – The 3 Lifetime Elision Rules (Real Life) ===\n");

    let hello = String::from("hello");
    let world = String::from("world, this is longer!");

    // These work without a single explicit lifetime
    println!("1. First word          : {}", first_word(&hello));
    println!("2. Identity function   : {}", identity(&hello));
    
    {
     let world=String::from("example");
     println!("3. Method with &self   : {}", Parser::new(&world).get());
    }
    
    // This case NEEDS an explicit lifetime
    let result = longest_explicit(&hello, &world);
    println!("4. Explicit 'a needed  : {}", result);
}

////////////////////////////////////////////////////////////////////////////////
// THE 3 RULES THAT LET RUST INFER LIFETIMES AUTOMATICALLY
////////////////////////////////////////////////////////////////////////////////

// Rule 1 + Rule 2 → exactly one input lifetime → output gets it
fn first_word(s: &str) -> &str {
    s.split(' ').next().unwrap_or(s)
}

// Rule 1 + Rule 2 → same single-input rule
fn identity(s: &str) -> &str {
    s
}

// Rule 1 + Rule 3 → &self present → output gets &self's lifetime
struct Parser<'a> {
    text: &'a str,
}
impl<'a> Parser<'a> {
    fn new(text: &'a str) -> Self {
        Parser { text }
    }

    fn get(&self) -> &str {
        self.text
    }
}

// ❌ Lifetime elision CANNOT work here (two inputs, no &self)
// ✅ Explicit lifetime required
fn longest_explicit<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
