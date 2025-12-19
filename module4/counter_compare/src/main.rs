use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

// ------------------ MUTEX VERSION ------------------
fn mutex_counter() {
    println!("\n=== Mutex Counter ===");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for id in 1..=4 {
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            for _ in 0..5 {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
                println!("Thread {} incremented counter to {}", id, *num);
                drop(num);
                thread::sleep(Duration::from_millis(100));
            }
        });

        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!(
        "Final counter value (Mutex) = {}",
        *counter.lock().unwrap()
    );
}

// ------------------ CHANNEL VERSION ------------------
fn channel_counter() {
    println!("\n=== Channel Counter ===");

    let (tx, rx) = mpsc::channel();
    let mut producers = vec![];

    for id in 1..=4 {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            for _ in 0..5 {
                tx_clone.send(id).unwrap();
                thread::sleep(Duration::from_millis(100));
            }
        });
        producers.push(handle);
    }

    drop(tx);

    let consumer = thread::spawn(move || {
        let mut count = 0;
        for _msg in rx {
            count += 1;
        }
        count
    });

    for p in producers {
        p.join().unwrap();
    }

    let final_count = consumer.join().unwrap();
    println!("Final counter value (Channel) = {}", final_count);
}

// ------------------ MAIN ------------------
fn main() {
    mutex_counter();
    channel_counter();
}
.