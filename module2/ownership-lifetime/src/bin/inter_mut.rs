// src/bin/inter_mut.rs - Fixed version
use std::cell::{Cell, RefCell};
//use std::rc::Rc;

fn main() {
    println!("=== FIXED INTERIOR MUTABILITY DEMONSTRATION ===\n");
    
    // ------------------------------------------------------------
    // 1. CELL<T> - FIXED: Only works with Copy types
    // ------------------------------------------------------------
    println!("1. CELL<T> - Works with Copy types only");
    println!("{}", "=".repeat(50));
    
    let cell = Cell::new(42); // i32 is Copy
    println!("Cell value: {}", cell.get());
    
    // This would NOT compile - String is not Copy
    // let string_cell = Cell::new(String::from("hello"));
    // println!("{}", string_cell.get()); // ERROR: String is not Copy
    
    // ------------------------------------------------------------
    // 2. REFCELL<T> - Works with non-Copy types
    // ------------------------------------------------------------
    println!("\n\n2. REFCELL<T> - Works with non-Copy types");
    println!("{}", "=".repeat(50));
    
    let refcell = RefCell::new(String::from("Hello"));
    
    // Correct way to access String in RefCell
    {
        let borrowed = refcell.borrow();
        println!("Borrowed value: {}", *borrowed);
    }
    
    // Modify the value
    {
        let mut borrowed_mut = refcell.borrow_mut();
        borrowed_mut.push_str(", World!");
    }
    
    println!("After modification: {}", refcell.borrow());
    
    // ------------------------------------------------------------
    // 3. FIXED MockService example
    // ------------------------------------------------------------
    println!("\n\n3. FIXED MOCK SERVICE EXAMPLE");
    println!("{}", "=".repeat(50));
    
    struct MockService {
        call_log: RefCell<Vec<String>>,
        response: RefCell<String>, // Changed from Cell to RefCell
    }
    
    impl MockService {
        fn new() -> Self {
            MockService {
                call_log: RefCell::new(Vec::new()),
                response: RefCell::new(String::from("Default response")),
            }
        }
        
        fn call(&self, endpoint: &str) -> String {
            // Log the call
            self.call_log.borrow_mut().push(endpoint.to_string());
            
            // Return response - FIXED: use borrow() instead of get()
            self.response.borrow().clone() // Clone since we need to return a String
        }
        
        fn set_response(&self, response: &str) {
            // FIXED: Use borrow_mut() to modify String
            *self.response.borrow_mut() = response.to_string();
        }
        
        fn get_call_log(&self) -> Vec<String> {
            self.call_log.borrow().clone()
        }
    }
    
    let service = MockService::new();
    service.set_response("Hello from mock");
    println!("Call 1 result: {}", service.call("/api/test"));
    println!("Call 2 result: {}", service.call("/api/data"));
    println!("Call log: {:?}", service.get_call_log());
    
    // ------------------------------------------------------------
    // 4. WORKING EXAMPLE WITH CELL<OPTION<T>>
    // ------------------------------------------------------------
    println!("\n\n4. CELL WITH OPTION - A WORKAROUND");
    println!("{}", "=".repeat(50));
    
    // You CAN use Cell with Option if you replace the entire value
    struct CachedValue {
        value: Cell<Option<String>>, // Option<String> is fine with Cell
    }
    
    impl CachedValue {
        fn new() -> Self {
            CachedValue {
                value: Cell::new(None),
            }
        }
        
        fn set(&self, val: &str) {
            self.value.set(Some(val.to_string()));
        }
        
        fn get(&self) -> Option<String> {
            self.value.take() // Takes the value out, leaving None
        }
    }
    
    let cached = CachedValue::new();
    cached.set("Hello");
    
    match cached.get() {
        Some(val) => println!("Got value: {}", val),
        None => println!("No value"),
    }
    
    // After get(), it's None
    match cached.get() {
        Some(val) => println!("Got value again: {}", val),
        None => println!("No value (expected)"),
    }
    
    // ------------------------------------------------------------
    // 5. KEY DIFFERENCE SUMMARY
    // ------------------------------------------------------------
    println!("\n\n5. KEY DIFFERENCES SUMMARY");
    println!("{}", "=".repeat(50));
    
    println!("Cell<T> requirements:");
    println!("  • T must implement Copy trait");
    println!("  • Can use get()/set() methods");
    println!("  • Examples: i32, bool, f64, Option<T>");
    
    println!("\nRefCell<T> requirements:");
    println!("  • Any T works (Copy or non-Copy)");
    println!("  • Must use borrow()/borrow_mut() methods");
    println!("  • Runtime borrow checking");
    println!("  • Examples: String, Vec<T>, custom structs");
    
    println!("\nPractical rule:");
    println!("  • Use Cell for simple types (numbers, bools)");
    println!("  • Use RefCell for complex types (String, Vec, etc.)");
    println!("  • Use Cell<Option<T>> as a workaround for moving values");
    
    // ------------------------------------------------------------
    // 6. COMPLETE WORKING EXAMPLE
    // ------------------------------------------------------------
    println!("\n\n6. COMPLETE WORKING EXAMPLE");
    println!("{}", "=".repeat(50));
    
    struct UserSession {
        id: Cell<u32>,          // Copy type - use Cell
        username: RefCell<String>, // Non-Copy - use RefCell
        login_count: Cell<u32>, // Copy type - use Cell
        tokens: RefCell<Vec<String>>, // Non-Copy - use RefCell
    }
    
    impl UserSession {
        fn new(id: u32, username: &str) -> Self {
            UserSession {
                id: Cell::new(id),
                username: RefCell::new(username.to_string()),
                login_count: Cell::new(0),
                tokens: RefCell::new(Vec::new()),
            }
        }
        
        fn login(&self) {
            self.login_count.set(self.login_count.get() + 1);
            println!("User {} logged in (count: {})", 
                     self.username.borrow(), self.login_count.get());
        }
        
        fn add_token(&self, token: &str) {
            self.tokens.borrow_mut().push(token.to_string());
        }
        
        fn change_username(&self, new_name: &str) {
            *self.username.borrow_mut() = new_name.to_string();
        }
        
        fn display(&self) {
            println!("User ID: {}", self.id.get());
            println!("Username: {}", self.username.borrow());
            println!("Login count: {}", self.login_count.get());
            println!("Tokens: {:?}", self.tokens.borrow());
        }
    }
    
    let session = UserSession::new(1, "alice");
    session.login();
    session.add_token("abc123");
    session.change_username("alice_updated");
    session.login();
    session.add_token("def456");
    
    session.display();
    
    println!("\n=== DEMONSTRATION COMPLETE ===");
}