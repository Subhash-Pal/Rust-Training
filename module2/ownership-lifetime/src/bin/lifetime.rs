//1. Basic Lifetime Syntax & Why It's Needed
//rust
// Without explicit lifetimes - This won't compile!
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }

// With explicit lifetimes
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("short");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    // Can't use `result` here because `string2` is out of scope
}
