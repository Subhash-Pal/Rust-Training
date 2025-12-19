use std::rc::Rc;
use std::cell::RefCell;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
struct AppConfig {
    // Wrap fields we might want to modify in a RefCell
    debug: RefCell<bool>,
    api_url: RefCell<String>,
}

fn main() {

    //*******Simple Example 1**********//
     // Create an integer 5 on the heap, wrapped in an Rc.
    // The strong count is now 1.
    let a: Rc<i32> = Rc::new(5);

    // Create a new pointer 'b' that shares ownership with 'a'.
    // This increments the reference count. The strong count is now 2.
    let b: Rc<i32> = Rc::clone(&a);

    println!("Value of a: {}", a);
    println!("Value of b: {}", b);
    
    // Check the count of owners sharing the data
    println!("Total number of owners: {}", Rc::strong_count(&a));

    {
        let b: Rc<i32> = Rc::clone(&a);
        println!("Total number of owners: {}", Rc::strong_count(&a));
       

    }
    println!("Total number of owners: {}", Rc::strong_count(&a));

    //*******Simple Example 2**********//
    let config = Rc::new(AppConfig {
        debug: RefCell::new(true),
        api_url: RefCell::new("https://api.example.com".to_string()),
    });

    // Create shared pointers
    let service1_config = Rc::clone(&config);
    let service2_config = Rc::clone(&config);

    println!("--- Initial State ---");
    println!("Service1 config URL: {:?}", service1_config.api_url.borrow());
    println!("Service2 config URL: {:?}", service2_config.api_url.borrow());
    println!("Total Rc strong count: {}", Rc::strong_count(&config));

    // Modify the configuration through the 'config' variable (which uses the same underlying memory)
    println!("\n--- Modifying Config via 'config' owner ---");
    // We use .borrow_mut() to get mutable access to the RefCell's contents
    *config.api_url.borrow_mut() = "https://api.new-example.com".to_string();
    *config.debug.borrow_mut() = false;

    // Show that the changes are instantly reflected in the 'service1' and 'service2' Rcs
    println!("--- State After Modification ---");
    println!("Service1 config URL now sees change: {:?}", service1_config.api_url.borrow());
    println!("Service2 config debug status: {:?}", service2_config.debug.borrow());

    // You can also use one of the service variables to make a further change
    println!("\n--- Modifying Config via 'service2_config' owner ---");
    *service2_config.api_url.borrow_mut() = "https://api.final-example.com".to_string();

    // The original 'config' owner sees the change too
    println!("Original config URL now sees final change: {:?}", config.api_url.borrow());
    
    println!("\nSince a change made through one Rc pointer is visible to all others, the memory is demonstrably shared, not copied.");
}
