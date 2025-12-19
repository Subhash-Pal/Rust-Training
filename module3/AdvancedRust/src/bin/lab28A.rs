// ==============================================
// Hour 28: Async Rust (Tokio Basics)
// Tasks + await + timers
// ==============================================

// NOTE:
// Add this to Cargo.toml before running:
//
// [dependencies]
// tokio = { version = "1", features = ["full"] }

use tokio::time::{sleep, Duration};

// Async function
async fn fetch_data(id: u32) -> String {
    // Simulate async I/O delay
    sleep(Duration::from_secs(1)).await;
    format!("Data fetched for request {}", id)
}

#[tokio::main]
async fn main() {
    println!("Starting async tasks...");

    // Spawn async tasks
    let task1 = tokio::spawn(fetch_data(1));
    let task2 = tokio::spawn(fetch_data(2));

    // Await results
    let result1 = task1.await.unwrap();
    let result2 = task2.await.unwrap();

    println!("{}", result1);
    println!("{}", result2);

    println!("All async tasks completed");
}

/*



*/