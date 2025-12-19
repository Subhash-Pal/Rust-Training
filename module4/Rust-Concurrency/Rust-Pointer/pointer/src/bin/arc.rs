use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 1. Wrap the counter in a Mutex for safe mutation
    // 2. Wrap the Mutex in an Arc for safe shared ownership across threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    let num_threads = 10;

    for _ in 0..num_threads {
        // Create a clone of the Arc for each new thread
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // Lock the Mutex to get exclusive access to the inner data
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            // The lock is automatically released when 'num' goes out of scope
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Lock the final value and print it
    println!("Result: {}", *counter.lock().unwrap()); // Output: Result: 10
}
