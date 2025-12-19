// src/bin/lab24a.rs
// Lab 24A – Custom Iterator Trait (Zero Warnings)

#![allow(dead_code)]  // ← This silences the warning while keeping the field

fn main() {
    println!("=== Lab 24A – Custom Iterator Trait ===\n");

    println!("--- Example 1: Range iterator (1 to 10) ---");
    let range = Range::new(1, 10);
    for num in range {
        print!("{} ", num);
    }
    println!("\n");

    println!("--- Example 2: First 10 Fibonacci numbers ---");
    let fib = Fibonacci::new();
    for (i, num) in fib.take(10).enumerate() {
        println!("Fib {}: {}", i, num);
    }
    println!();

    println!("--- Example 3: Chaining with map & filter ---");
    let squares = Range::new(1, 15)
        .map(|x| x * x)
        .filter(|&x| x % 2 == 0)
        .take(5);

    println!("First 5 even squares: {:?}", squares.collect::<Vec<_>>());






}

#[derive(Debug)]
struct Range {
    start: u32,     // ← Kept for clarity (real iterators often keep original bounds)
    end: u32,
    current: u32,
}

impl Range {
    fn new(start: u32, end: u32) -> Self {
        Range {
            start,
            end,
            current: start,
        }
    }
}

impl Iterator for Range {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let value = self.current;
            self.current += 1;
            Some(value)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.a;
        let temp = self.a + self.b;
        self.a = self.b;
        self.b = temp;
        Some(next)
    }
}