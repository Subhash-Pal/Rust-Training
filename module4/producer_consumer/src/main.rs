use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Create channel
    let (tx, rx) = mpsc::channel();

    // ---------- Producer Thread ----------
    let producer = thread::spawn(move || {
        for i in 1..=5 {
            println!("Producer: sending {}", i);
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
        println!("Producer: finished");
    });

    // ---------- Consumer Thread ----------
    let consumer = thread::spawn(move || {
        for received in rx {
            println!("Consumer: received {}", received);
        }
        println!("Consumer: channel closed");
    });

    producer.join().unwrap();
    consumer.join().unwrap();

    println!("Main thread: done");
}
