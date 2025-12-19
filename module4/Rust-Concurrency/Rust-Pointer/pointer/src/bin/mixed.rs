use std::cell::{Cell, RefCell};
use std::rc::Rc;

fn main() {
    // 1. Cell<T> – Simple counter that works with Copy types (no borrowing)
    let request_count = Cell::new(0u64);
    request_count.set(request_count.get() + 1); // works even in multi-threaded callbacks
    println!("Requests served: {}", request_count.get()); // → 1

    // 2. RefCell<T> – Mock database in tests (needs runtime borrow checking)
    let db = RefCell::new(vec!["alice".to_string(), "bob".to_string()]);
    
    // Normal mutable borrow
    db.borrow_mut().push("charlie".to_string());
    
    // Immutable borrow
    println!("Users: {:?}", db.borrow()); // → ["alice", "bob", "charlie"]

    // 3. Rc<RefCell<T>> – Shared mutable game score (multiple systems can modify it)
    let score = Rc::new(RefCell::new(100));

    // Player collects coin
    *score.borrow_mut() += 50;

    // Enemy hits player
    let score2 = Rc::clone(&score);
    *score2.borrow_mut() -= 30;

    // UI reads the score (shared ownership + interior mutability)
    println!("Final score: {}", score.borrow()); // → 120
}