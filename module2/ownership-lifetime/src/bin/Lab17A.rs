// src/bin/lab17a.rs
// Lab 17A – Lifetimes Made Crystal Clear (ZERO WARNINGS)

fn main() {
    println!("=== Lab 17A – Lifetimes Made Crystal Clear ===\n");

    // Example 1: longest() – no lifetime annotations needed in call site!
    let string1 = String::from("short");
    let string2 = String::from("this one is much longer");

    let result = longest(&string1, &string2);
    println!("The longest string is → {}", result);
    println!("Both originals still alive:");
    println!("  string1 = {}", string1);
    println!("  string2 = {}\n", string2);

    // Example 2: Compiler prevents dangling references
    {
        let temporary = String::from("I only exist in this block");
        // let bad = longest(&string2, &temporary); // ← Try uncommenting → ERROR!
        // Rust says: `temporary` does not live long enough
        drop(temporary); // just to show the scope
    }

    // Example 3: Struct holding a borrowed value
    let novel = String::from("The Call of the Wild. A story of a dog named Buck...");
    let first_sentence = novel.split('.').next().expect("No sentence");

    let excerpt = ImportantExcerpt { part: first_sentence };
    excerpt.announce("Breaking News");
    {
        //let s=String::from("Breaking News");
        let s="Breaking News";
        excerpt.announce(s);
        let _result = longest(&string1,&s);
    }
    println!("\nKey Takeaways:");
    println!("   • Lifetimes = how long a reference is valid");
    println!("   • Rust guarantees no dangling references at compile time");
    println!("   • You rarely write 'a yourself – elision rules handle it");
    println!("   • When needed → just copy the pattern from longest<'a>()");
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}
/*
struct ImportantExcerpt<'a,'b> {
    part: &'a str,
    part: &'b str
}
    */

    struct ImportantExcerpt<'a> {
    part: &'a str,
    
}
impl<'a> ImportantExcerpt<'a> {
    fn announce(&self, msg: &str) {
        println!("{msg}!");
        println!("Excerpt: {}", self.part);
    }
}