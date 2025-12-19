/*🧪 Lab 39A – Basic (Binary Release Build + Help Command)
🎯 Objective

Create a simple CLI binary

Add a --help command

Learn debug vs release build

Prepare code that is ready to package
*/
use std::env;

fn print_help() {
    println!("Lab39A - Basic Rust CLI");
    println!("Usage:");
    println!("  lab39A greet <name>");
    println!("  lab39A --help");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // No arguments
    if args.len() == 1 {
        println!("No arguments provided.");
        print_help();
        return;
    }

    match args[1].as_str() {
        "--help" | "-h" => {
            print_help();
        }
        "greet" => {
            if args.len() < 3 {
                println!("Error: Name missing");
            } else {
                println!("Hello, {}! 👋", args[2]);
            }
        }
        _ => {
            println!("Unknown command.");
            print_help();
        }
    }
}
/*
🧪 Lab 39A – Basic (Binary Release Build + Help Command)
*/