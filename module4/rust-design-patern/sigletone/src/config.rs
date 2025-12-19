// src/config.rs
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

// The global singleton instance (private to this module)
static CONFIG: LazyLock<Mutex<ConfigurationManager>> = LazyLock::new(|| {
    Mutex::new(ConfigurationManager::new())
});

/// The configuration manager struct
pub struct ConfigurationManager {
    settings: HashMap<String, String>,
}

impl ConfigurationManager {
    // Private constructor — only callable inside this file
    fn new() -> Self {
        let mut settings = HashMap::new();
        // Simulate loading from file/env — replace with real logic if needed
        settings.insert("theme".to_string(), "dark".to_string());
        settings.insert("language".to_string(), "en".to_string());
        settings.insert("timeout_seconds".to_string(), "30".to_string());

        println!("ConfigurationManager initialized with {} settings", settings.len());

        ConfigurationManager { settings }
    }

    /// Public method to read a setting
    pub fn get_setting(&self, key: &str) -> Option<&String> {
        self.settings.get(key)
    }

    /// Optional: Add a setter if you need to modify config at runtime
    pub fn set_setting(&mut self, key: String, value: String) {
        self.settings.insert(key, value);
        println!("Setting updated dynamically");
    }
}

// Public functions to access the singleton safely

/// Get a locked guard to the singleton (read-only or mutable access)
pub fn instance() -> std::sync::MutexGuard<'static, ConfigurationManager> {
    CONFIG.lock().unwrap()
}

/// Convenience function to read a setting without manually locking
pub fn get_setting(key: &str) -> Option<String> {
    let config = instance();
    config.get_setting(key).cloned()
}

/// Convenience function to set a setting (if needed)
pub fn set_setting(key: String, value: String) {
    let mut config = instance();
    config.set_setting(key, value);
}