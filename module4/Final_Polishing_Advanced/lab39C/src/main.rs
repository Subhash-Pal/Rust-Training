use std::env;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_DESC: &str = env!("CARGO_PKG_DESCRIPTION");

fn help() {
    println!("{APP_NAME} - {APP_DESC}");
    println!();
    println!("USAGE:");
    println!("  {APP_NAME} greet <name>");
    println!("  {APP_NAME} info");
    println!("  {APP_NAME} --help");
    println!("  {APP_NAME} --version");
}

fn version() {
    println!("{APP_NAME} v{APP_VERSION}");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    match args[1].as_str() {
        "--help" | "-h" => help(),

        "--version" | "-V" => version(),

        "info" => {
            println!("Application : {APP_NAME}");
            println!("Version     : {APP_VERSION}");
            println!("Profile     : release-ready");
        }

        "greet" => {
            let name = args.get(2).map(String::as_str).unwrap_or("Guest");
            println!("Hello, {} 🚀", name);
        }

        _ => {
            eprintln!("Unknown command");
            help();
        }
    }
}

/*
▶️ Step 1: Build & Test
cargo run -- --help
cargo run -- --version
cargo run -- greet Rust

🚀 Step 2: Release Build
cargo build --release


Run binary:

target\release\lab39C.exe --version

*/