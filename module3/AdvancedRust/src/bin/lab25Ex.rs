// ==============================================
// Hour 25 Exercise: Write Your Own Macro
// ==============================================

// Warning logging macro
macro_rules! log_warn {
    ($msg:expr) => {
        println!("[WARN] {}", $msg);
    };
    ($fmt:expr, $($arg:tt)*) => {
        println!(concat!("[WARN] ", $fmt), $($arg)*);
    };
}

fn main() {
    log_warn!("Disk space low");
    log_warn!("CPU usage at {}%", 92);
}
    