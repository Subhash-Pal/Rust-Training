use std::sync::mpsc;
use std::thread;

fn main() {
    // Create unbuffered channels for Ping and Pong communication
    let (ping_tx, ping_rx) = mpsc::channel();
    let (pong_tx, pong_rx) = mpsc::channel();

    // Spawn the first thread: Initiator (sends Ping, waits for Pong)
    let pong_sender = pong_tx.clone(); // Clone the transmitter for the initiator
    let handle1 = thread::spawn(move || {
        for i in 0..26 {
            let letter = (b'A' + i) as char; // Generate letters from 'A' to 'Z'

            // Send the Ping message
            pong_sender.send(letter.to_string()).unwrap();
            println!("Ping: {}", i);

            // Wait for the Pong reply before proceeding
            ping_rx.recv().unwrap();
        }
    });

    // Spawn the second thread: Responder (receives Ping, sends Pong)
    let handle2 = thread::spawn(move || {
        for _ in 0..26 {
            // Receive the Ping message
            let received_letter = pong_rx.recv().unwrap();
            println!("Pong: {}", received_letter);

            // Send an acknowledgment back on the Ping channel
            ping_tx.send("ack".to_string()).unwrap();
        }
    });

    // Wait for both threads to finish
    handle1.join().unwrap();
    handle2.join().unwrap();

    // Channels are automatically closed when all senders/receivers are dropped
}