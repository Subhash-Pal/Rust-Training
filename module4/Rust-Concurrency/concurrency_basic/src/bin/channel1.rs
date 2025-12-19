/*
Channels (Message Passing)
🔹 Concept

Channels allow safe communication between threads using messages.

✅ Example: Sending data between threads

*/

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send("Hello from child thread").unwrap();
    });

    let message = rx.recv().unwrap();
    println!("Received: at Main {}", message);
}
