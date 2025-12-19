

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// This is the "User One" function, running in Thread 1
fn fn_user_one(
    tx_to_two: mpsc::Sender<&'static str>, // Sender to User Two
    rx_from_two: mpsc::Receiver<&'static str>, // Receiver from User Two
) {
    println!("[User 1]: Started. Sending initial greeting to User 2.");
    
    // 1. Send first message
    tx_to_two.send("Hello User 2! Ready to chat?").unwrap();
    
    // 2. Wait for a reply
    let reply_one = rx_from_two.recv().unwrap();
    println!("[User 1]: Got reply: '{}'. Sending confirmation.", reply_one);

    // 3. Send final message
    tx_to_two.send("Confirmed. Chat complete.").unwrap();
}

// This is the "User Two" function, running in Thread 2
fn fn_user_two(
    tx_to_one: mpsc::Sender<&'static str>, // Sender to User One
    rx_from_one: mpsc::Receiver<&'static str>, // Receiver from User One
) {
    println!("[User 2]: Started. Waiting for User 1 to initiate.");

    // 1. Wait for User 1's message
    let initial_message = rx_from_one.recv().unwrap();
    println!("[User 2]: Got message: '{}'. Replying now.", initial_message);

    // 2. Send a reply
    thread::sleep(Duration::from_millis(50)); // Simulate some work
    tx_to_one.send("Yes! I am ready to chat.").unwrap();

    // 3. Wait for the final confirmation message
    let final_message = rx_from_one.recv().unwrap();
    println!("[User 2]: Got final message: '{}'.", final_message);
}


fn main() {
    // --- Setup the TWO bidirectional channels in main() ---
    
    // Channel A: Main/User 1 -> User 2
    let (tx_a_to_b, rx_a_from_b) = mpsc::channel();
    
    // Channel B: User 2 -> Main/User 1 (we swap the send/receive ends when spawning)
    let (tx_b_to_a, rx_b_from_a) = mpsc::channel();

    // --- Spawn Thread 1 for fn_user_one ---
    thread::spawn(move || {
        // We pass the send-to-B handle and the receive-from-B handle
        fn_user_one(tx_a_to_b, rx_b_from_a);
    });
    
    // --- Spawn Thread 2 for fn_user_two ---
    thread::spawn(move || {
        // We pass the send-to-A handle and the receive-from-A handle
        fn_user_two(tx_b_to_a, rx_a_from_b);
    });

    // Main thread just waits for the other two threads to complete their communication
    // Note: To cleanly ensure main waits, you would typically collect the JoinHandles 
    // and call .join() on them, as shown in the previous examples.
    println!("[Main]: Waiting for User 1 and User 2 to finish their conversation...");
    thread::sleep(Duration::from_secs(1)); 
    println!("[Main]: Conversation likely finished.");
}
