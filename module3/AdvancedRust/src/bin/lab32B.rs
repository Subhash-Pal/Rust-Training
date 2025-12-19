use std::time::Instant;

fn slow_concat() -> String {
    let mut s = String::new();
    for i in 0..10_000 {
        s = format!("{}{}", s, i);
    }
    s
}

fn fast_concat() -> String {
    let mut s = String::with_capacity(100_000);
    use std::fmt::Write;
    for i in 0..10_000 {
        write!(s, "{}", i).unwrap();
    }
    s
}

fn main() {
    let start = Instant::now();
    slow_concat();
    println!("Slow concat: {:?}", start.elapsed());

    let start = Instant::now();
    fast_concat();
    println!("Fast concat: {:?}", start.elapsed());
}

/*
🧠 Key Optimization Principles (VERY IMPORTANT)
🔹 1. Measure Before Optimizing

Guessing is slower than measuring.

🔹 2. Avoid Reallocation

Use with_capacity

Avoid format! in loops

🔹 3. Prefer Iterators (They Are Zero-Cost)
let sum: i32 = (0..1_000_000).filter(|x| x % 2 == 0).sum();


➡️ Compiles to tight loops
➡️ No runtime overhead
*/