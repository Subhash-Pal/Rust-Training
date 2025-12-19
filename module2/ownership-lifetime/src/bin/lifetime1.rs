//2. Real-World Example: Database Connection Pool
//rust
// Simulating a database connection
struct DatabaseConnection {
    connection_id: u32,
}

// Connection pool that holds references to connections
struct ConnectionPool<'a> {
    connections: Vec<&'a DatabaseConnection>,
    max_connections: usize,
}

impl<'a> ConnectionPool<'a> {
    fn new(max_connections: usize) -> Self {
        ConnectionPool {
            connections: Vec::with_capacity(max_connections),
            max_connections,
        }
    }
    
    fn add_connection(&mut self, conn: &'a DatabaseConnection) -> Result<(), &str> {
        if self.connections.len() >= self.max_connections {
            return Err("Connection pool is full");
        }
        self.connections.push(conn);
        Ok(())
    }
    
    fn get_connection(&self, index: usize) -> Option<&'a DatabaseConnection> {
        self.connections.get(index).copied()
    }
}

fn main() {
    // Create some database connections
    let conn1 = DatabaseConnection { connection_id: 1 };
    let conn2 = DatabaseConnection { connection_id: 2 };
    
    // Create a connection pool
    let mut pool = ConnectionPool::new(5);
    
    // Add connections to the pool
    pool.add_connection(&conn1).unwrap();
    pool.add_connection(&conn2).unwrap();
    
    // Use connections from the pool
    if let Some(conn) = pool.get_connection(0) {
        println!("Using connection ID: {}", conn.connection_id);
    }
    
    // The connections must live at least as long as the pool
    // Drop order matters: if we drop conn1 before pool, we get a compile error
}