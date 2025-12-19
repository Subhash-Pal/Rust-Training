use std::sync::mpsc;
use std::thread;

fn main() {
    // ONE channel created:
    let (sender_original, receiver) = mpsc::channel();

    // Clone the sender handle to give to Thread A
    let sender_a = sender_original.clone();
    thread::spawn(move || {
        sender_a.send("Message from Thread A").unwrap();
    });

    // Clone the sender handle to give to Thread B
    let sender_b = sender_original.clone();
    thread::spawn(move || {
        sender_b.send("Message from Thread B").unwrap();
    });

    // Drop the original sender handle in the main thread.
    // This ensures the channel closes when all cloned senders are dropped.
    drop(sender_original);

    // ONE thread consumes ALL messages using the 'receiver' handle
    for received_message in receiver.iter() {
        println!("Consumer thread received: {}", received_message);
    }

    // The loop automatically ends when all sender handles have been dropped.
    println!("All messages consumed.");
}