// Log parser with lifetimes - real-world example
struct LogParser<'a> {
    log_line: &'a str,
    timestamp: &'a str,
    message: &'a str,
    log_level: &'a str,
}

impl<'a> LogParser<'a> {
    fn new(log_line: &'a str) -> Result<Self, &'static str> {
        // Split into timestamp and rest
        let (date, rest) = log_line.split_once(' ')
            .ok_or("Invalid log format: missing date")?;
        
        // Split again to get time
        let (time, rest) = rest.split_once(' ')
            .ok_or("Invalid log format: missing time")?;
        
        // Reconstruct full timestamp
        let timestamp = &log_line[..date.len() + 1 + time.len()];
        
        // Split log level and message
        let (log_level, message) = rest.split_once(": ")
            .ok_or("Invalid log format: missing ': ' separator")?;
        
        Ok(LogParser {
            log_line,
            timestamp,
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
    
    fn get_full_log(&self) -> &'a str {
        self.log_line
    }
    
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
    
    fn display(&self) {
        println!("Full: {}", self.get_full_log());
        println!("  Time: {}", self.get_timestamp());
        println!("  Level: {}", self.get_log_level());
        println!("  Message: {}", self.get_message());
    }
}

fn process_logs(log_data: &str) -> Vec<LogParser<'_>> {
    let logs: Vec<LogParser<'_>> = log_data
        .lines()
        .filter_map(|line| LogParser::new(line).ok())
        .collect();
    
    println!("=== Processing {} log entries ===", logs.len());
    
    for (i, log) in logs.iter().enumerate() {
        println!("\nEntry {}:", i + 1);
        log.display();
    }
    
    logs
}

fn analyze_logs(logs: &[LogParser<'_>]) {
    println!("\n=== Log Analysis ===");
    
    // Count by log level
    let mut error_count = 0;
    let mut warning_count = 0;
    let mut info_count = 0;
    
    for log in logs {
        match log.get_log_level() {
            "ERROR" => error_count += 1,
            "WARN" => warning_count += 1,
            "INFO" => info_count += 1,
            _ => {}
        }
    }
    
    println!("ERROR logs: {}", error_count);
    println!("WARN logs: {}", warning_count);
    println!("INFO logs: {}", info_count);
    
    // Find longest message
    if let Some(longest) = logs.iter().max_by_key(|log| log.get_message().len()) {
        println!("\nLongest message ({} chars):", longest.get_message().len());
        println!("  '{}'", longest.get_message());
    }
}

fn main() {
    println!("=== LOG PARSER WITH LIFETIMES ===\n");
    
    let log_data = String::from(
        "2023-01-01 10:30:00 INFO: Server started successfully\n\
         2023-01-01 10:31:00 WARN: High memory usage detected\n\
         2023-01-01 10:32:00 ERROR: Database connection failed\n\
         2023-01-01 10:33:00 INFO: Retrying connection...\n\
         2023-01-01 10:34:00 WARN: Disk space running low\n\
         2023-01-01 10:35:00 INFO: Backup completed"
    );
    
    // Parse logs
    let logs = process_logs(&log_data);
    
    // Analyze logs
    analyze_logs(&logs);
    
    // Demonstrate combine method
    if logs.len() >= 2 {
        println!("\n=== Combined Log Example ===");
        let combined = logs[0].combine_with_other_log(&logs[1]);
        println!("Combined: {}", combined);
    }
    
    // Show that references depend on original data
    println!("\n=== Lifetime Relationship ===");
    println!("All LogParser instances borrow from the original log_data.");
    println!("They cannot outlive the log_data string.");
}