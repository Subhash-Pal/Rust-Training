use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;

// -----------------------------
// Data Model
// -----------------------------
#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
}

// -----------------------------
// Entry Point
// -----------------------------
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                anyhow::bail!("Task title missing");
            }
            let title = args[2..].join(" ");
            validate_title(&title)?;
            add_task(&title)?;
        }
        "remove" => {
            let id: u32 = args
                .get(2)
                .context("Task ID missing")?
                .parse()
                .context("Invalid task ID")?;
            remove_task(id)?;
        }
        "list" => list_tasks()?,
        _ => print_help(),
    }

    Ok(())
}

// -----------------------------
// Validation
// -----------------------------
fn validate_title(title: &str) -> Result<()> {
    let reserved = ["add", "remove", "list"];

    if title.trim().is_empty() {
        anyhow::bail!("Task title cannot be empty");
    }

    if reserved.contains(&title.trim()) {
        anyhow::bail!("'{}' is a reserved command", title);
    }

    Ok(())
}

// -----------------------------
// Persistence (FIXED)
// -----------------------------
const FILE_PATH: &str = "tasks.json";

fn load_tasks() -> Result<Vec<Task>> {
    if !Path::new(FILE_PATH).exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(FILE_PATH)
        .with_context(|| "Failed to read tasks.json")?;

    // ✅ FIX: handle empty file safely
    if content.trim().is_empty() {
        return Ok(Vec::new());
    }

    let tasks: Vec<Task> = serde_json::from_str(&content)
        .with_context(|| "tasks.json is corrupted")?;

    Ok(tasks)
}

fn save_tasks(tasks: &[Task]) -> Result<()> {
    let json = serde_json::to_string_pretty(tasks)
        .context("Failed to serialize tasks")?;

    fs::write(FILE_PATH, json)
        .context("Failed to write tasks.json")?;

    Ok(())
}

// -----------------------------
// Commands
// -----------------------------
fn add_task(title: &str) -> Result<()> {
    let mut tasks = load_tasks()?;

    let next_id = tasks.last().map(|t| t.id + 1).unwrap_or(1);

    tasks.push(Task {
        id: next_id,
        title: title.to_string(),
        completed: false,
    });

    save_tasks(&tasks)?;
    println!("Task added: {}", title);
    Ok(())
}

fn remove_task(id: u32) -> Result<()> {
    let mut tasks = load_tasks()?;
    let before = tasks.len();

    tasks.retain(|t| t.id != id);

    if tasks.len() == before {
        println!("Task not found.");
        return Ok(());
    }

    save_tasks(&tasks)?;
    println!("Task removed.");
    Ok(())
}

fn list_tasks() -> Result<()> {
    let tasks = load_tasks()?;

    if tasks.is_empty() {
        println!("No tasks found.");
        return Ok(());
    }

    println!("ID | Status | Title");
    println!("---------------------------");

    for t in tasks {
        let status = if t.completed { "✔" } else { " " };
        println!("{:2} | [{}]   | {}", t.id, status, t.title);
    }

    Ok(())
}

// -----------------------------
// Help
// -----------------------------
fn print_help() {
    println!(
        r#"
Task Manager CLI

Commands:
  add <title>       Add a task
  remove <id>       Remove a task
  list              List tasks

Examples:
  cargo run -- add Learn Rust Ownership
  cargo run -- list
  cargo run -- remove 1
"#
    );
}
