use std::sync::mpsc;
use std::thread;

// Function to create a sender thread
fn create_sender_thread(sender: mpsc::Sender<String>, thread_id: usize) {
    thread::spawn(move || {
        let message = format!("Message from Thread {}", thread_id);
        // Clone the message before sending it
        sender.send(message.clone()).unwrap();
        println!("Thread {} sent: {}", thread_id, message);
    });
}

// Function to listen for messages
fn listen_for_messages(receiver: mpsc::Receiver<String>) {
    for received_message in receiver.iter() {
        println!("Listener received: {}", received_message);
    }
    println!("All messages consumed.");
}

fn main() {
    // Create a multi-producer, single-consumer channel
    let (sender_original, receiver) = mpsc::channel();

    // Number of sender threads
    let num_senders = 5;

    // Spawn multiple sender threads
    for i in 0..num_senders {
        let sender = sender_original.clone();
        create_sender_thread(sender, i);
    }

    // Drop the original sender handle to close the channel when all senders are done
    drop(sender_original);

    // Start listening for messages
    listen_for_messages(receiver);
}