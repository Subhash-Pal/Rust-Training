use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;
use std::time::{Instant, Duration};

fn main() {
    let num_threads = 10;
    let num_messages_per_thread = 10000;

    // Benchmark using channels
    let channel_time = benchmark_channels(num_threads, num_messages_per_thread);

    // Benchmark using mutex
    let mutex_time = benchmark_mutex(num_threads, num_messages_per_thread);

    // Print results
    println!("Channels took {:.2?} seconds", channel_time);
    println!("Mutex took {:.2?} seconds", mutex_time);
}

// Benchmark using channels
fn benchmark_channels(num_threads: usize, num_messages_per_thread: usize) -> Duration {
    let (sender, receiver) = mpsc::channel();

    let start = Instant::now();

    // Spawn threads that send messages through the channel
    for i in 0..num_threads {
        let sender_clone = sender.clone();
        thread::spawn(move || {
            for j in 0..num_messages_per_thread {
                sender_clone.send((i, j)).unwrap();
            }
        });
    }

    // Drop the original sender to close the channel after all threads are done
    drop(sender);

    // Receive all messages
    for _ in 0..(num_threads * num_messages_per_thread) {
        receiver.recv().unwrap();
    }

    Instant::now() - start
}

// Benchmark using mutex
fn benchmark_mutex(num_threads: usize, num_messages_per_thread: usize) -> Duration {
    let shared_counter = Arc::new(Mutex::new(0));

    let start = Instant::now();

    // Spawn threads that increment the shared counter
    let mut handles = vec![];
    for _ in 0..num_threads {
        let counter_clone = Arc::clone(&shared_counter);
        let handle = thread::spawn(move || {
            for _ in 0..num_messages_per_thread {
                let mut counter = counter_clone.lock().unwrap();
                *counter += 1;
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    Instant::now() - start
}