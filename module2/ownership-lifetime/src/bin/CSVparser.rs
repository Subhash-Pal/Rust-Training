// src/bin/CSVparser.rs
// Simple CSV Parser – Clean Version (Zero Warnings)

use std::fs::File;
use std::io::{BufRead, BufReader};  // ← Fixed: removed unused `self`

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Simple CSV Parser ===\n");

    let csv_data = r#"
name,age,city,"country, with comma"
Alice,30,"New York, NY",USA
Bob,25,London,"United Kingdom"
"Charlie ""The Great""",42,Paris,France
Diana,35,"São Paulo",Brazil
"#;

    std::fs::write("data.csv", csv_data.trim_start())?;
    println!("Created data.csv with sample data\n");

    let records = parse_csv("data.csv")?;

    println!("Parsed {} records:\n", records.len());
    for (i, record) in records.iter().enumerate() {
        println!("Record {}: {:?}", i + 1, record);
    }

    Ok(())
}

fn parse_csv(path: &str) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut records = Vec::new();

    for (line_number, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        let fields = parse_line(&line);
        records.push(fields);
        println!("Line {} parsed into {} fields", line_number + 1, records.last().unwrap().len());
    }

    Ok(records)
}

fn parse_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();

    while let Some(&c) = chars.peek() {
        chars.next();

        if c == '"' {
            if in_quotes && chars.peek() == Some(&'"') {
                current.push('"');
                chars.next();
            } else {
                in_quotes = !in_quotes;
            }
        } else if c == ',' && !in_quotes {
            fields.push(current.trim().to_string());
            current.clear();
        } else {
            current.push(c);
        }
    }

    fields.push(current.trim().to_string());
    fields
}