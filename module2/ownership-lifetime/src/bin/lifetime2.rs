//3. Real-World Example: Text Processing (Log Parser)
//rust
// A log entry parser that extracts references to parts of the log line
struct LogParser<'a> {
    log_line: &'a str,
    timestamp: &'a str,
    message: &'a str,
    log_level: &'a str,  // Added log level field
}

impl<'a> LogParser<'a> {
    fn new(log_line: &'a str) -> Result<Self, &'static str> {
        // Better parsing: split by spaces
        let parts: Vec<&str> = log_line.splitn(3, ' ').collect();
        
        if parts.len() < 3 {
            return Err("Invalid log format");
        }
        
        let timestamp = format!("{} {}", parts[0], parts[1]).leak(); // Keep as &'static for demo
        let rest = parts[2];
        
        // Extract log level (e.g., INFO:, WARN:, ERROR:)
        let log_level_end = rest.find(':').unwrap_or(rest.len());
        let log_level = &rest[..log_level_end.min(rest.len())];
        let message = &rest[log_level_end + 1..].trim();
        
        Ok(LogParser {
            log_line,  // Now used in display method
            timestamp: &*timestamp, // Convert String to &str
            message,
            log_level,
        })
    }
    
    fn get_timestamp(&self) -> &'a str {
        self.timestamp
    }
    
    fn get_message(&self) -> &'a str {
        self.message
    }
    
    fn get_log_level(&self) -> &'a str {
        self.log_level
    }
    
    // Now actually using log_line field
    fn get_full_log(&self) -> &'a str {
        self.log_line
    }
    
    // Method that uses combine_with_other_log
    fn combine_with_other_log<'b>(
        &self,
        other: &'b LogParser<'b>
    ) -> String {
        format!("[{}] {} -- [{}] {}", 
                self.get_log_level(), 
                self.get_message(),
                other.get_log_level(),
                other.get_message())
    }
    
    // Display formatted log entry
    fn display(&self) {
        println!("Full: {}", self.get_full_log());
        println!("  Time: {}", self.get_timestamp());
        println!("  Level: {}", self.get_log_level());
        println!("  Message: {}", self.get_message());
        println!("---");
    }
}

fn process_logs<'a>(log_data:  &'a str) -> Vec<LogParser<'a>> {
    // Process multiple log lines
    let logs: Vec<LogParser> = log_data
        .lines()
        .filter_map(|line| LogParser::new(line).ok())
        .collect();
    
    println!("=== Processing {} log entries ===", logs.len());
    
    for log in &logs {
        log.display();
    }
    
    // Demonstrate combine_with_other_log
    if logs.len() >= 2 {
        println!("\n=== Combined Log Example ===");
        let combined = logs[0].combine_with_other_log(&logs[1]);
        println!("Combined: {}", combined);
    }
    
    logs
}

fn analyze_logs<'a>(logs: &'a [LogParser<'a>]) {
    println!("\n=== Log Analysis ===");
    
    let error_count = logs.iter()
        .filter(|log| log.get_log_level() == "ERROR")
        .count();
    
    let warning_count = logs.iter()
        .filter(|log| log.get_log_level() == "WARN")
        .count();
    
    let info_count = logs.iter()
        .filter(|log| log.get_log_level() == "INFO")
        .count();
    
    println!("ERROR logs: {}", error_count);
    println!("WARN logs: {}", warning_count);
    println!("INFO logs: {}", info_count);
    
    // Find the longest log message
    if let Some(longest) = logs.iter()
        .max_by_key(|log| log.get_message().len()) 
    {
        println!("\nLongest message ({} chars):", longest.get_message().len());
        println!("  {}", longest.get_message());
    }
}

fn main() {
    let log_data = String::from(
        "2023-01-01 10:30:00 INFO: Server started successfully\n\
         2023-01-01 10:31:00 WARN: High memory usage detected\n\
         2023-01-01 10:32:00 ERROR: Database connection failed\n\
         2023-01-01 10:33:00 INFO: Retrying connection...\n\
         2023-01-01 10:34:00 WARN: Disk space running low\n\
         2023-01-01 10:35:00 INFO: Backup completed"
    );
    
    // Parse and process logs
    let logs = process_logs(&log_data);
    
    // Analyze the logs
    analyze_logs(&logs);
    
    // Demonstrate lifetime constraints
    demonstrate_lifetimes(&logs);
}

fn demonstrate_lifetimes<'a>(logs: &'a [LogParser<'a>]) {
    println!("\n=== Lifetime Demonstration ===");
    
    // Create references to specific parts
    let first_log = &logs[0];
    let first_timestamp = first_log.get_timestamp();
    let first_message = first_log.get_message();
    
    println!("First timestamp lives as long as logs: {}", first_timestamp);
    println!("First message lives as long as logs: {}", first_message);
    
    // Show that we can't keep these references if logs are dropped
    // This is enforced by the compiler
    println!("All log references are valid while 'logs' is in scope");
}