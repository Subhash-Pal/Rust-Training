/*
Hour 10: Traits, covering:

How Rust traits compare to C++ interfaces
A Printable trait with multiple implementors
An exercise: Implement a custom trait for your own struct
*/
// ===== Printable Trait (Example) =====
// ===== Printable Trait (Fixed: renamed method) =====
trait Printable {
    fn print(&self);
    fn as_printable_string(&self) -> String; // ← Renamed to avoid conflict
}

// Implement Printable for &str
impl Printable for &str {
    fn print(&self) {
        println!("{}", self);
    }

   fn as_printable_string(&self) -> String {
        self.to_string() // Now safe: we're not in a "to_string" method
    }
    
}
//&str -> Printable ->as_printable_string() -> String

// Implement Printable for String
impl Printable for String {
    fn print(&self) {
        println!("{}", self);
    }

    fn as_printable_string(&self) -> String {
        self.clone()
    }
}
//////////////////////
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    pages: u32,
}

/*
impl Printable for &str {
    fn to_string(&self) -> String {
        ToString::to_string(self) // ✅ calls std's to_string
        // OR
        // <&str as ToString>::to_string(self)
    }
}

*/
impl Printable for Book {
    fn print(&self) {
        println!("📖 '{}' by {} ({} pages)", self.title, self.author, self.pages);
    }

    fn as_printable_string(&self) -> String {
        format!("'{}' by {} ({} pages)", self.title, self.author, self.pages)
    }
}

// ===== Measurable Trait (unchanged, no conflict) =====
trait Measurable {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Measurable for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

impl Measurable for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}
//describe_shape(obj: &impl Measurable)
fn describe_shape<T: Measurable + std::fmt::Debug>(shape: &T) {
    println!(
        "Shape: {:?}\n  Area: {:.2}\n  Perimeter: {:.2}\n",
        shape,
        shape.area(),
        shape.perimeter()
    );
}

// ===== Main =====
fn main() {
    println!("=== Printable Trait Demo ===");
    
    let message = "Hello from &str!";
    message.print();
    
    let text = "Hello from String!".to_string();
    text.print();
    
    let book = Book {
        title: "The Rust Programming Language".to_string(),
        author: "Steve Klabnik & Carol Nichols".to_string(),
        pages: 576,
    };
    book.print();
    println!("Book as printable string: {}", book.as_printable_string());

    println!("\n=== Exercise: Measurable Trait ===");
    ////////////////////////////////////
     
    let circle = Circle { radius: 5.0 };
    let rect = Rectangle { width: 4.0, height: 6.0 };
    
    describe_shape(&circle);
    describe_shape(&rect);
}