use std::time::Instant;

fn main() {
    let mut data: Vec<i32> = (0..1_000_000)
        .rev()
        .collect();

    let start = Instant::now();
    data.sort();
    let duration = start.elapsed();

    println!("Vec sorting time: {:?}", duration);
}
