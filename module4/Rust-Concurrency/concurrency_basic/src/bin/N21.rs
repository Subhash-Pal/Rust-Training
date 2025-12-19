use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    // Create multiple producer threads
    for i in 0..5 {
        let tx_clone = tx.clone(); // Clone the transmitter for each thread
        thread::spawn(move || {
            let message = format!("Message from thread {}", i);
            tx_clone.send(message).unwrap();
        });
    }

    // Drop the original transmitter to signal no more messages will be sent
    drop(tx);

    // Receive all messages from the channel
    for received in rx {
        println!("Received: {}", received);
    }
}