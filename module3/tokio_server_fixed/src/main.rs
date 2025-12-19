use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    
    println!("========================================");
    println!("  Tokio Async Server READY");
    println!("  • http://localhost:8080");
    println!("  • http://localhost:8080/health");
    println!("  • http://localhost:8080/json");
    println!("  Stop with Ctrl+C");
    println!("========================================");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buffer = [0; 2048];
            let n = match socket.read(&mut buffer).await {
                Ok(0) => return,
                Ok(n) => n,
                Err(_) => return,
            };

            let request = String::from_utf8_lossy(&buffer[..n]);
            let request_line = request.lines().next().unwrap_or("");
            let path = request_line.split_whitespace().nth(1).unwrap_or("/");

            println!("{} → {}", addr, request_line);

            let (status, content_type, body) = match path {
                "/" => ("200 OK", "text/plain", "Hello from Tokio async server! 🚀"),
                "/health" => ("200 OK", "text/plain", "OK"),
                "/json" => ("200 OK", "application/json",
                    r#"{"server":"tokio","status":"running","version":"1.0"}"#),
                _ => ("404 Not Found", "text/plain", "Not Found 😢"),
            };

            let response = format!(
                "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                status,
                content_type,
                body.len(),
                body
            );

            let _ = socket.write_all(response.as_bytes()).await;
            let _ = socket.shutdown().await;
        });
    }
}