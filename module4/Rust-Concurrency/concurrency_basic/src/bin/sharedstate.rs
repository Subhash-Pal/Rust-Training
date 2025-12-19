use std::sync::{mpsc, Arc, Mutex};
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let shared_data = Arc::new(Mutex::new(0)); // Shared mutable state

    // Spawn a thread to modify shared data
    let shared_data_clone = Arc::clone(&shared_data);
    thread::spawn(move || {
        let mut data = shared_data_clone.lock().unwrap();
        *data += 42;
        tx.send(*data).unwrap(); // Send updated value through the channel
    });

    // Receive the updated value from the channel
    let result = rx.recv().unwrap();
    println!("Shared data updated to: {}", result);
}