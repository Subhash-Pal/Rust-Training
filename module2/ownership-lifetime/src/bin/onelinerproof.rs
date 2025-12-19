use rayon::prelude::*; // REQUIRED for par_iter()

fn main() {
    let data: Vec<u8> = (0..100_000_000).map(|i| i as u8).collect();

    println!("Address: {:p}", data.as_ptr());

    let sequential_sum: u64 =
        data.iter().map(|&x| x as u64).sum();

    let parallel_sum: u64 =
        data.par_iter().map(|&x| x as u64).sum();

    assert_eq!(sequential_sum, parallel_sum);

    println!(
        "Zero-copy parallel sum works perfectly at {:p}",
        data.as_ptr()
    );
}
