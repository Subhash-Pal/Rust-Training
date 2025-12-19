// ==============================================
// Hour 24: Traits Deep Dive
// Supertraits + Associated Types
// ==============================================

use std::fmt;

// ---------- Supertrait Example ----------
// Supertrait requires Display
trait Printable: fmt::Display {
    fn print(&self) {
        println!("{}", self);
    }
}

// ---------- Custom Iterator Trait ----------
// Uses Associated Type
trait MyIterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

// ---------- Struct Implementing Traits ----------
struct Counter {
    current: u32,
    max: u32,
}

// Implement Custom Iterator
impl MyIterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            self.current += 1;
            Some(self.current)
        } else {
            None
        }
    }
}

// Implement Display (required by Printable supertrait)
impl fmt::Display for Counter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Counter at {}", self.current)
    }
}

// Implement Supertrait
impl Printable for Counter {}

// ---------- Main ----------
fn main() {
    let mut counter = Counter { current: 0, max: 3 };

    // Supertrait method
    counter.print();

    // Custom iterator usage
    while let Some(value) = counter.next() {
        println!("Next value: {}", value);
    }
}
