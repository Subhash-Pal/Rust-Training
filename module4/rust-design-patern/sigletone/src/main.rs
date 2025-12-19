// src/main.rs
mod config;  // Imports the config module

use config::{get_setting, instance, set_setting};

fn main() {
    // First access → triggers lazy initialization
    
    println!("Initial theme: {}", get_setting("theme").unwrap_or("unknown".to_string()));
    // Direct access via lock (useful when you need multiple reads)
    {
        let config = instance();
        println!("Language: {}", config.get_setting("language").unwrap());
        println!("Timeout: {}", config.get_setting("timeout_seconds").unwrap());
    }

    // Demonstrate dynamic update (optional feature)
    set_setting("theme".to_string(), "light".to_string());

    // Prove it's the same instance
    println!("Updated theme: {}", get_setting("theme").unwrap());

    // Test from another thread
    let handle = std::thread::spawn(|| {
        let config = instance();
        println!(
            "From another thread - theme: {}",
            config.get_setting("theme").unwrap()
        );
    });

    handle.join().unwrap();

    // Try to create another instance? → Impossible from here!
    // config::ConfigurationManager::new();  // Compile error: private
}