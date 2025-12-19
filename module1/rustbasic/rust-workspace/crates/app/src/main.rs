use utils::greet;

fn main() {
    println!("{}", greet("Rust Workspace"));
}

/*
⚠️ Common Workspace Mistakes

❌ Adding [package] to root Cargo.toml
❌ Wrong relative path in dependencies
❌ Naming conflicts between crates
❌ Running cargo run without -p
*/