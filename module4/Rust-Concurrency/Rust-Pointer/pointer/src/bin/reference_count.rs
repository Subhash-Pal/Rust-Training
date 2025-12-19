use std::rc::{Rc, Weak};

fn main() {
    // --- Initial State Setup ---

    // A strong reference, 'five', holds the data (the integer 5).
    let five = Rc::new(5);

    // --- Increasing Strong Count ---
    // Make 3 additional strong clones (c, d, e)
    let c = Rc::clone(&five);
    let d = Rc::clone(&five);
    let e = Rc::clone(&five);

    println!("--- After strong clones ---");
    // Strong count: 4, Weak count: 0
    println!("Strong count  {}", Rc::strong_count(&five)); 
    println!("Weak count {}", Rc::weak_count(&five));     

    // --- Increasing Weak Count ---
    // Make 2 weak references (weak_five, weak_five1)
    let weak_five = Rc::downgrade(&five);
    println!("Strong count  {}", Rc::strong_count(&five)); 
    println!("Weak count {}", Rc::weak_count(&five));  
    let weak_five1 = Rc::downgrade(&five);
    // Make a third weak reference via clone
    let weak_five2 = Weak::clone(&weak_five); 

    println!("\n--- After weak clones ---");
    // Strong count: 4, Weak count: 3
    println!("Strong count  {}", Rc::strong_count(&five)); 
    println!("Weak count {}", Rc::weak_count(&five));     

    // --- Demonstrating temporary Strong Count increase via upgrade() ---
    
    { // Start a new scope to show the temporary nature of the upgrade
        // This line creates a NEW, temporary strong reference 'stg'
        let stg = weak_five1.upgrade(); 

        println!("\n--- Inside scope (after calling .upgrade()) ---");
        // Strong count is now 5 (five, c, d, e, stg), Weak count stays at 3
        println!("Strong count  {}", Rc::strong_count(&five)); 
        
        println!("Weak count {}", Rc::weak_count(&five));       

        // 'stg' is dropped when this inner scope ends
    } // End of inner scope

    println!("\n--- Outside scope (stg dropped) ---");
    // Strong count drops back to 4
    println!("Strong count  {}", Rc::strong_count(&five)); 
    println!("Weak count {}", Rc::weak_count(&five));   




    //drop case 
    let five = Rc::new(5);
    let c = Rc::clone(&five);
    let d = Rc::clone(&five);
    
    println!("Strong count before early drop: {}", Rc::strong_count(&five)); // Output: 3

    // Manually drop the 'c' strong reference early
    drop(c); 

    println!("Strong count after early drop: {}", Rc::strong_count(&five)); // Output: 2
    // 'd' and 'five' are still in scope until the end of main()
  
}
