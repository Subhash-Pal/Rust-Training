// ==============================================
// Hour 28 Exercise: Parallel HTTP Requests
// Using reqwest + tokio
// ==============================================

// Cargo.toml dependencies:
//
// [dependencies]
// tokio = { version = "1", features = ["full"] }
// reqwest = { version = "0.11", features = ["json"] }

use reqwest::Client;

async fn fetch_url(client: &Client, url: &str) -> Result<String, reqwest::Error> {
    let response = client.get(url).send().await?;
    let body = response.text().await?;
    Ok(body)
}

#[tokio::main]
async fn main() {
    let client = Client::new();

    let urls = vec![
        "https://httpbin.org/get",
        "https://httpbin.org/uuid",
        "https://httpbin.org/ip",
    ];

    // Spawn parallel requests
    let tasks: Vec<_> = urls
        .into_iter()
        .map(|url| {
            let client = client.clone();
            tokio::spawn(async move {
                fetch_url(&client, url).await
            })
        })
        .collect();

    // Await all tasks
    for task in tasks {
        match task.await.unwrap() {
            Ok(body) => println!("Response received ({} chars)", body.len()),
            Err(e) => eprintln!("Request failed: {}", e),
        }
    }
}

/*
🧠 What This Exercise Teaches

Async HTTP calls

Client reuse (Client::clone)

True parallelism with async tasks

Error handling in async Rust
*/