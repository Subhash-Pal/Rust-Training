use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    // Thread 1 sends a message after 1 second
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        tx1.send("Message from Channel 1").unwrap();
    });

    // Thread 2 sends a message after 2 seconds
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        tx2.send("Message from Channel 2").unwrap();
    });

    // Poll both channels until both messages are received
    let mut msg1_received = false;
    let mut msg2_received = false;

    while !msg1_received || !msg2_received {
        if !msg1_received {
            if let Ok(msg) = rx1.try_recv() {
                println!("Received: {}", msg);
                msg1_received = true;
            }
        }

        if !msg2_received {
            if let Ok(msg) = rx2.try_recv() {
                println!("Received: {}", msg);
                msg2_received = true;
            }
        }

        // Sleep briefly to avoid busy-waiting
        thread::sleep(Duration::from_millis(100));
    }
}