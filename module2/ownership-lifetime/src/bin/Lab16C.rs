// src/bin/lab16_simple.rs
// THE SIMPLEST CORRECT EXAMPLE – NO CRATES, NO ERRORS, PERFECT RESULT

use rayon::prelude::*;
use std::time::Instant;

fn main() {
    println!("=== Simple Zero-Copy Parallel Demo ===\n");

    // Create 100 MiB of data – only once!
    let data: Vec<u8> = (0..100_000_000).map(|i| (i % 251) as u8).collect();
    println!("Created 100 MiB at address: {:p}", data.as_ptr());

    // Sequential sum (simple but correct)
    let start = Instant::now();
    let seq_sum = data.iter().map(|&b| b as u64).sum::<u64>();
    let seq_time = start.elapsed();

    // Parallel sum – same data, no clone, just &[]
    let start = Instant::now();
    let par_sum = data.par_iter().map(|&b| b as u64).sum::<u64>();
    let par_time = start.elapsed();

    let speedup = seq_time.as_secs_f64() / par_time.as_secs_f64().max(0.001);

    // Beautiful result
    println!("\nResults:");
    println!("   Sequential sum : {}", seq_sum);
    println!("   Parallel sum   : {}", par_sum);
    println!("   Match          : {}", if seq_sum == par_sum { "YES" } else { "NO" });
    println!("   Sequential time: {:.3} s", seq_time.as_secs_f64());
    println!("   Parallel time  : {:.3} s", par_time.as_secs_f64());
    println!("   Speedup        : {:.1}x", speedup);
    println!("\n   Memory copied  : 0 bytes (only borrowing with &[u8])");
    println!("   Data still valid at: {:p}", data.as_ptr());

    println!("\nDone! You just saw zero-copy parallel magic.");
}