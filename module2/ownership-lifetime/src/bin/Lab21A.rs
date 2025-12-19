// Lab 21 – File I/O + Serialization (Serde)

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;

// -------------------------------
// Configuration Struct
// -------------------------------
#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    app_name: String,
    version: String,
    debug: bool,
    max_connections: u32,
}

// -------------------------------
// Main
// -------------------------------
fn main() -> Result<()> {
    println!("=== Hour 21: File I/O + Serialization (Serde) ===\n");

    let config = AppConfig {
        app_name: "RustServer".to_string(),
        version: "1.0.0".to_string(),
        debug: true,
        max_connections: 500,
    };

    // -------------------------------
    // JSON
    // -------------------------------
    println!("--- JSON Serialization ---");
    save_config_json("config.json", &config)?;
    let loaded_json = load_config_json("config.json")?;
    println!("Loaded from JSON: {:?}\n", loaded_json);

    // -------------------------------
    // CSV
    // -------------------------------
    println!("--- CSV Serialization ---");
    save_config_csv("config.csv", &config)?;
    let loaded_csv = load_config_csv("config.csv")?;
    println!("Loaded from CSV: {:?}\n", loaded_csv);

    Ok(())
}

// -------------------------------
// JSON: Save
// -------------------------------
fn save_config_json(path: &str, config: &AppConfig) -> Result<()> {
    let json = serde_json::to_string_pretty(config)
        .context("Failed to serialize config to JSON")?;

    let mut file = File::create(path)
        .with_context(|| format!("Failed to create file: {}", path))?;

    file.write_all(json.as_bytes())
        .context("Failed to write JSON to file")?;

    println!("Config saved to {}", path);
    Ok(())
}

// -------------------------------
// JSON: Load
// -------------------------------
fn load_config_json(path: &str) -> Result<AppConfig> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read JSON file: {}", path))?;

    let config = serde_json::from_str(&content)
        .context("Failed to deserialize JSON")?;

    Ok(config)
}

// -------------------------------
// CSV: Save
// -------------------------------
fn save_config_csv(path: &str, config: &AppConfig) -> Result<()> {
    let mut writer = csv::Writer::from_path(path)
        .with_context(|| format!("Failed to create CSV file: {}", path))?;

    writer
        .serialize(config)
        .context("Failed to write config to CSV")?;

    writer.flush().context("Failed to flush CSV writer")?;

    println!("Config saved to {}", path);
    Ok(())
}

// -------------------------------
// CSV: Load
// -------------------------------
fn load_config_csv(path: &str) -> Result<AppConfig> {
    let mut reader = csv::Reader::from_path(path)
        .with_context(|| format!("Failed to open CSV file: {}", path))?;

    let mut records = reader.deserialize::<AppConfig>();

    let config = records
        .next()
        .context("CSV file is empty")??
        ;

    Ok(config)
}
