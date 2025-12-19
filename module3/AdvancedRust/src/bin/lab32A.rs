// ==============================================
// Hour 32: Performance Optimization
// Example: Measure Vec sorting time
// ==============================================

use std::time::Instant;

fn main() {
    // Create a large vector
    let mut data: Vec<i32> = (0..1_000_000).rev().collect();

    let start = Instant::now();

    // Operation to benchmark
    data.sort();

    let duration = start.elapsed();

    println!("Sorting took: {:?}", duration);
}
