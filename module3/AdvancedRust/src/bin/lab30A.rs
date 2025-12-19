// ==============================================
// Hour 30: Traits + Async (Async Traits)
// Async DataProvider Trait
// ==============================================

// Cargo.toml dependencies:
//
// [dependencies]
// tokio = { version = "1", features = ["full"] }
// async-trait = "0.1"

use async_trait::async_trait;

// ---------- Async Trait Definition ----------
#[async_trait]
trait DataProvider {
    async fn fetch_data(&self, id: u32) -> String;
}

// ---------- Real Implementation ----------
struct ApiProvider;

#[async_trait]
impl DataProvider for ApiProvider {
    async fn fetch_data(&self, id: u32) -> String {
        // Simulate async API call
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        format!("Data from API for id {}", id)
    }
}

// ---------- Mock Implementation (Exercise Solved) ----------
struct MockProvider;

#[async_trait]
impl DataProvider for MockProvider {
    async fn fetch_data(&self, id: u32) -> String {
        format!("Mock data for id {}", id)
    }
}

// ---------- Main ----------
#[tokio::main]
async fn main() {
    let api = ApiProvider;
    let mock = MockProvider;

    let api_result = api.fetch_data(1).await;
    let mock_result = mock.fetch_data(2).await;

    println!("{}", api_result);
    println!("{}", mock_result);
}

/*
🧠 What You Learned (Hour 30)
Concept	                       Explanation
async_trait	                   Enables async methods in traits
Trait abstraction	           Swap real vs mock
Testability	                   Mock provider
Clean async design	           Dependency inversion

*/