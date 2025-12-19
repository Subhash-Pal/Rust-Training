// ==============================================
// Hour 25: Macros (Declarative + Derive)
// ==============================================

// ---------- Declarative Macro (macro_rules!) ----------
// Simple logging macro
macro_rules! log_info {
    ($msg:expr) => {
        println!("[INFO] {}", $msg);
    };
    ($fmt:expr, $($arg:tt)*) => {
        println!(concat!("[INFO] ", $fmt), $($arg)*);
    };
}

// ---------- Derive Macro Example ----------
// Using built-in derive(Debug)
#[derive(Debug)]
struct User {
    id: u32,
    name: String,
    active: bool,
}

// ---------- Main ----------
fn main() {
    // Declarative macro usage
    log_info!("Application started");
    log_info!("User id = {}, active = {}", 1, true);

    // Derive macro usage
    let user = User {
        id: 1,
        name: "Alice".to_string(),
        active: true,
    };

    println!("Debug output: {:?}", user);
}
