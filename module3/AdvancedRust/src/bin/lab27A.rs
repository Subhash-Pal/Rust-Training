// ==============================================
// Hour 27: Shared State Concurrency
// Mutex + Arc (Safe Shared Counter)
// ==============================================

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Shared counter wrapped in Arc + Mutex
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    // Spawn 5 threads
    for _ in 0..5 {
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            // Lock the mutex
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            println!("Counter updated to {}", *num);
            // Mutex is automatically unlocked here
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Final value
    println!("Final Counter Value: {}", *counter.lock().unwrap());
}
