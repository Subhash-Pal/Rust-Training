use std::time::{SystemTime, UNIX_EPOCH};

// ----------------- Declarative Logging Macro -----------------
macro_rules! log {
    ($level:expr, $msg:expr) => {{
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        println!("[{}][{}] {}", $level, now, $msg);
    }};

    ($level:expr, $fmt:expr, $($arg:tt)*) => {{
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        println!(
            "[{}][{}] {}",
            $level,
            now,
            format!($fmt, $($arg)*)
        );
    }};
}

// ----------------- Custom “Derive-like” Trait -----------------
trait MyDebug {
    fn my_debug(&self) -> String;
}

struct User {
    id: u32,
    name: String,
    active: bool,
}

// Manual implementation (what derive does internally)
impl MyDebug for User {
    fn my_debug(&self) -> String {
        format!(
            "User {{ id: {}, name: \"{}\", active: {} }}",
            self.id, self.name, self.active
        )
    }
}

fn main() {
    log!("INFO", "Application started");

    let user = User {
        id: 101,
        name: "Alice".to_string(),
        active: true,
    };

    log!("DEBUG", "Created user struct");
    log!("INFO", "User details: {}", user.my_debug());

    if user.active {
        log!("WARN", "User {} is active", user.name);
    }

    log!("INFO", "Application finished cleanly");
}
