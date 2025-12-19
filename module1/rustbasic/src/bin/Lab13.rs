// ==============================================
// Hour 13: Iterators + Closures
// ==============================================
// Rust's iterators are zero-cost abstractions:
// • Lazy (computed on-demand)
// • Composable (chain filter → map → fold)
// • Zero runtime overhead (compiled to efficient loops)
//
// Closures in Rust vs C++ Lambdas:
// • Rust closures capture variables with precise ownership (`Fn`, `FnMut`, `FnOnce`)
// • No manual memory management — borrow checker enforces safety
// • Can be inlined completely at compile time (zero cost)

//use std::fmt::Debug;

// ============================================================================
// Part 1: Closures + Iterator Examples (filter, map, fold)
// ============================================================================
fn demo_closures_and_iterators() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    println!("=== Original numbers ===");
    println!("{:?}", numbers);

    // --------------------------------------------------------
    // 1. `filter`: keep only even numbers
    // Closure: |x| x % 2 == 0  → takes `x`, returns bool
    // --------------------------------------------------------
    let evens = numbers
        .iter()
        .filter(|&x| x % 2 == 0)  // `&x` because we're iterating by reference
        .copied()                 // convert &i32 → i32
        .collect::<Vec<i32>>();  // collect into Vec<i32>
    
    println!("\n=== Even numbers (filter) ===");
    println!("{:?}", evens);

    // --------------------------------------------------------
    // 2. `map`: square each number
    // Closure captures by value (but `x` is i32, so it's copied)
    // --------------------------------------------------------
    let squares: Vec<i32> = evens
        .iter()
        .map(|&x| x * x)   // destructure reference with `&x`
        .collect();

    println!("\n=== Squares (map) ===");
    println!("{:?}", squares);

    // --------------------------------------------------------
    // 3. `fold`: sum all squares
    // `fold` takes an initial value and an accumulator closure
    // --------------------------------------------------------
    let total = squares.iter().fold(0, |acc, &x| acc + x);

    println!("\n=== Sum of squares (fold) ===");
    println!("Total: {}", total);

    // --------------------------------------------------------
    // 4. Chain operations: one-liner pipeline
    // Shows the power of composition
    // --------------------------------------------------------
    let result: i32 = numbers
        .iter()
        .filter(|&x| x % 2 == 0)    // double & because iter() gives &&i32 in this context
        .map(|&x| x * x)
        .sum();  // sum() is shorthand for fold(0, |a, b| a + b)

    println!("\n=== Chained: even squares sum ===");
    println!("Result: {}", result);
}

// ============================================================================
// Part 2: Exercise — Custom Iterator: Counter
// ----------------------------------------------------------------------------
// Implements an iterator that counts from `start` to `end` (exclusive).
// Demonstrates:
//   - Implementing the `Iterator` trait
//   - Using `Option<T>` for termination
//   - Zero allocation, lazy evaluation
// ============================================================================
struct Counter {
    current: u32,
    end: u32,
}

impl Counter {
    /// Creates a new counter from `start` (inclusive) to `end` (exclusive).
    fn new(start: u32, end: u32) -> Self {
        Self { current: start, end }
    }
}

// Implement the `Iterator` trait
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let result = self.current;
            self.current += 1;
            Some(result)
        } else {
            None
        }
    }
}

// Add convenience methods via extension
impl Counter {
    /// Collects all values into a Vec<u32>.
    fn collect_vec(&mut self) -> Vec<u32> {
        self.collect()
    }

    /// Sums all values.
    fn sum_all(&mut self) -> u32 {
        self.sum()
    }
}

// ============================================================================
// Part 3: Demo Custom Iterator
// ============================================================================
fn demo_custom_iterator() {
    println!("\n=== Custom Iterator: Counter ===");

    let mut counter = Counter::new(5, 10);
    println!("Counter from 5 to 10 (exclusive):");

    // Use as iterator
    for num in &mut counter {
        print!("{} ", num);
    }
    println!();

    // Reuse? No — iterators are consumed. Create new one.
    let numbers: Vec<u32> = Counter::new(1, 6).collect_vec();
    println!("Collected: {:?}", numbers);

    let sum = Counter::new(1, 101).sum_all(); // sum 1 to 100
    println!("Sum 1 to 100: {}", sum);
}

// ============================================================================
// Bonus: Closure Ownership Demo (Fn, FnMut, FnOnce)
// ----------------------------------------------------------------------------
// Helps understand how Rust closures differ from C++ lambdas
// ============================================================================
fn closure_ownership_demo() {
    println!("\n=== Closure Ownership Demo ===");

    let x = 5;

    // `add_x` captures `x` by reference → implements `Fn`
    let add_x = |y| x + y;
    println!("add_x(10) = {}", add_x(10));
    println!("add_x(20) = {}", add_x(20)); // can call multiple times

    let mut vec = vec![1, 2, 3];

    // `push_to_vec` captures `vec` by mutable reference → `FnMut`
    let mut push_to_vec = || vec.push(4);
    push_to_vec();
    println!("After push: {:?}", vec);

    let s = String::from("hello");

    // `consume_s` takes ownership of `s` → `FnOnce` (can only call once)
    let consume_s = || println!("Consumed: {}", s);
    consume_s();
    // consume_s(); // ❌ Compile error: `s` moved
}

// ============================================================================
// Main Function
// ============================================================================
fn main() {
    demo_closures_and_iterators();
    demo_custom_iterator();
    closure_ownership_demo();
}