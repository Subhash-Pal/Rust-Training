/*
🎯 Objective

Optimize Cargo release profile

Reduce binary size

Control panic behavior

Keep CLI usable with --help

Cargo Optimization (Cargo.toml)

Replace Cargo.toml with:

[package]
name = "lab39B"
version = "0.1.0"
edition = "2021"

[profile.release]
opt-level = "z"       # Optimize for size
lto = true            # Link Time Optimization
codegen-units = 1     # Better optimization
panic = "abort"       # No unwinding → smaller binary
strip = true          # Strip symbols (Rust 1.70+)


📌 Why this matters

opt-level = "z" → smallest binary

panic = "abort" → no stack traces

strip = true → production-ready binary

🧩 Step 3: Improved CLI Program (src/main.rs)
*/

use std::env;

fn help() {
    println!("Lab39B - Optimized Rust CLI");
    println!();
    println!("Commands:");
    println!("  info        Show build info");
    println!("  greet NAME  Greet user");
    println!("  --help      Show this help");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    match args[1].as_str() {
        "--help" | "-h" => help(),

        "info" => {
            println!("Binary: lab39B");
            println!("Build: release-optimized");
        }

        "greet" => {
            
        let default = String::from("Guest");
        let name = args.get(2).unwrap_or(&default);
        println!("Hello, {} 🚀", name);

        }

        _ => {
            eprintln!("Unknown command");
            help();
        }
    }
}


/*
 Build & Compare Sizes
cargo build
cargo build --release


Check sizes:

ls -lh target/debug/lab39B
ls -lh target/release/lab39B


🔍 You’ll notice:

Release binary much smaller

Faster startup

No panic stack traces

🪟 PowerShell Commands (Recommended)
🔹 Check DEBUG binary size
Get-Item target\debug\lab39B.exe | Select-Object Name, Length

🔹 Check RELEASE binary size
Get-Item target\release\lab39B.exe | Select-Object Name, Length

📏 Human-Readable Size (MB)
Debug
(Get-Item target\debug\lab39B.exe).Length / 1MB

Release
(Get-Item target\release\lab39B.exe).Length / 1MB

🔍 Compare Both in One Command
Get-Item target\debug\lab39B.exe, target\release\lab39B.exe |
Select-Object Name, @{Name="Size(MB)";Expression={[math]::Round($_.Length/1MB,2)}}


🧪 Test Optimized Binary
./target/release/lab39B --help
./target/release/lab39B greet Rustacean
./target/release/lab39B info

📦 Step 6: Packaging Insight

✔ This binary is suitable for:

Docker images

Embedded Linux

Server-side CLI tools
*/