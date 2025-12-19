use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut sum: i64 = 0;

    for i in 0..1_000_000 {
        if i % 2 == 0 {
            sum += i as i64;
        }
    }

    println!("Manual loop: {:?}, sum={}", start.elapsed(), sum);

    let start = Instant::now();
    let sum: i64 = (0..1_000_000)
        .filter(|x| x % 2 == 0)
        .map(|x| x as i64)
        .sum();

    println!("Iterator: {:?}, sum={}", start.elapsed(), sum);
}

/*
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut sum: u64 = 0;

    for i in 0u64..1_000_000 {
        if i % 2 == 0 {
            sum += i;
        }
    }

    println!("Manual loop: {:?}, sum={}", start.elapsed(), sum);

    let start = Instant::now();
    let sum: u64 = (0u64..1_000_000)
        .filter(|x| x % 2 == 0)
        .sum();

    println!("Iterator: {:?}, sum={}", start.elapsed(), sum);
}

*/