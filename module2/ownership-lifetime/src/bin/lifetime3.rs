//4. Real-World Example: Caching System
//rust
#![allow(dead_code)]
struct Cache<'a, T> {
    data: Vec<(&'a str, T)>,
    max_size: usize,
}

impl<'a, T> Cache<'a, T> {
    fn new(max_size: usize) -> Self {
        Cache {
            data: Vec::with_capacity(max_size),
            max_size,
        }
    }
    
    fn insert(&mut self, key: &'a str, value: T) {
        if self.data.len() >= self.max_size {
            // Remove oldest entry (FIFO)
            self.data.remove(0);
        }
        self.data.push((key, value));
    }
    
    fn get(&self, key: &str) -> Option<&T> {
        self.data
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| v)
    }
}

// Struct with lifetime in a field
struct UserSession<'a> {
    username: &'a str,
    session_id: String,
    cache: Cache<'a, String>,
}

impl<'a> UserSession<'a> {
    fn new(username: &'a str, cache_size: usize) -> Self {
        UserSession {
            username,
            session_id: format!("session_{}", username),
            cache: Cache::new(cache_size),
        }
    }
    
    fn cache_data(&mut self, key: &'a str, data: String) {
        self.cache.insert(key, data);
    }
    
    fn get_cached_data(&self, key: &str) -> Option<&String> {
        self.cache.get(key)
    }
}

fn main() {
    let username = "alice";
    let mut session = UserSession::new(username, 10);
    
    let key1 = "user_profile";
    let data1 = String::from("{name: 'Alice', role: 'admin'}");
    
    // Cache some data
    session.cache_data(key1, data1);
    
    // Retrieve from cache
    if let Some(data) = session.get_cached_data(key1) {
        println!("Cached data: {}", data);
    }
    
    // The cache keys (&str) must live at least as long as the session
}