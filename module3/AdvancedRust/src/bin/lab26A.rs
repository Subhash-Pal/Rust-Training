// ==============================================
// Hour 26: Concurrency Basics
// Threads + Channels (Producer–Consumer)
// ==============================================

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Create channel
    let (tx, rx) = mpsc::channel();

    // Producer thread
    let producer = thread::spawn(move || {
        for i in 1..=5 {
            println!("Produced: {}", i);
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    // Consumer (main thread)
    for received in rx {
        println!("Consumed: {}", received);
    }

    producer.join().unwrap();
}
