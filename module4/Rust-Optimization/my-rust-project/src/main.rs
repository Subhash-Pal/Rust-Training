use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Serialize, Deserialize, Debug)]
struct Greeting {
    message: String,
}

fn main() {
    let start = Instant::now();

    let greeting = Greeting {
        message: "Hello, optimized dev build!".to_string(),
    };

    // Simulate some work using a dependency (serde)
    let json = serde_json::to_string(&greeting).unwrap();
    println!("Serialized: {}", json);

    println!("Execution time: {:?}", start.elapsed());
}