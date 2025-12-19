// ==============================================
// Hour 29: Async + Channels + Streams
// Async Progress Reporter
// ==============================================

use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

async fn worker( progress_tx: mpsc::Sender<u32>) {
    for i in 1..=5 {
        sleep(Duration::from_secs(1)).await;
        progress_tx.send(i * 20).await.unwrap(); // progress %
    }
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(10);

    // Spawn async worker
    tokio::spawn(worker(tx));

    // Progress listener (async stream-like)
    while let Some(progress) = rx.recv().await {
        println!("Progress: {}%", progress);
        if progress == 100 {
            println!("Task completed!");
            break;
        }
    }
}
