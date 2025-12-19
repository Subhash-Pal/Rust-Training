/*
🔐 3. Send and Sync Traits
🔹 Concept

Send → value can be transferred to another thread

Sync → value can be safely shared between threads

Most Rust types are Send + Sync by default.

✅ Example: Shared immutable data (Sync)
*/

use std::thread;
use std::sync::Arc;

fn main() {
    let data = Arc::new(100);

    let handles: Vec<_> = (0..3).map(|i| {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            println!("Thread {} sees {}", i, data);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
/*

📌 Key points

Arc<T> = Atomic Reference Counted

Arc<T> is Send + Sync

*/