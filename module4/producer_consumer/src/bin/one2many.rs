//cargo run --bin one2many
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// Typed messages passed over channel
enum Message {
    Data { producer_id: u32, value: u32 },
    Quit,
}

fn main() {
    // Buffered channel with capacity = 2
    let (tx, rx) = mpsc::sync_channel(2);

    let mut producers = Vec::new();

    // -------- Multiple Producers --------
    for producer_id in 1..=3 {
        let tx_clone = tx.clone();

        let handle = thread::spawn(move || {
            for value in 1..=3 {
                println!(
                    "Producer {} sending value {}",
                    producer_id, value
                );

                tx_clone
                    .send(Message::Data {
                        producer_id,
                        value,
                    })
                    .unwrap();

                thread::sleep(Duration::from_millis(300));
            }

            println!("Producer {} done", producer_id);
        });

        producers.push(handle);
    }

    // Drop original sender so channel closes after producers finish
    drop(tx);

    // -------- Consumer --------
    let consumer = thread::spawn(move || {
        let mut message_count = 0;

        while let Ok(msg) = rx.recv() {
            match msg {
                Message::Data {
                    producer_id,
                    value,
                } => {
                    println!(
                        "Consumer received {} from producer {}",
                        value, producer_id
                    );
                    message_count += 1;
                }
                Message::Quit => break,
            }
        }

        println!("Consumer processed {} messages", message_count);
    });

    // Wait for all producers
    for p in producers {
        p.join().unwrap();
    }

    consumer.join().unwrap();

    println!("Main finished cleanly");
}
