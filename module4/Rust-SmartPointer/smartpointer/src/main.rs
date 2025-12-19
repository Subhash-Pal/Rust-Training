// src/bin/smartpointer.rs - FINAL VERSION (No Warnings)
use std::rc::{Rc, Weak};
use std::cell::{Cell, RefCell};
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("=== COMPREHENSIVE SMART POINTERS DEMONSTRATION ===\n");
    
    // ------------------------------------------------------------
    // 1. BOX<T> - HEAP ALLOCATION
    // ------------------------------------------------------------
    println!("1. BOX<T> - HEAP ALLOCATION");
    println!("{}", "=".repeat(50));
    
    // Store data on the heap instead of the stack
    let boxed_i32 = Box::new(42);
    println!("Boxed i32: {}", boxed_i32);
    println!("Dereferenced: {}", *boxed_i32);
    println!("Via deref trait: {}", boxed_i32.deref());
    
    // Box enables recursive data structures
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    
    // Create a list and actually use all fields
    let list = List::Cons(1, 
        Box::new(List::Cons(2, 
            Box::new(List::Cons(3, 
                Box::new(List::Nil))))));
    
    // Function to print list and use all fields
    fn print_list(list: &List) {
        match list {
            List::Cons(value, next) => {
                println!("  Value: {}", value);
                print_list(next);
            }
            List::Nil => println!("  End of list"),
        }
    }
    
    println!("Linked list contents:");
    print_list(&list);
    
    // Box with array
    let array_box = Box::new([0u8; 10]);
    println!("Array on heap (first 3): {:?}", &array_box[..3]);
    
    // ------------------------------------------------------------
    // 2. RC<T> - REFERENCE COUNTED (SINGLE-THREADED)
    // ------------------------------------------------------------
    println!("\n\n2. RC<T> - REFERENCE COUNTED (SINGLE-THREADED)");
    println!("{}", "=".repeat(50));
    
    let rc_value = Rc::new(String::from("Shared Data"));
    
    // Clone increases reference count
    let rc_clone1 = Rc::clone(&rc_value);
    let rc_clone2 = Rc::clone(&rc_value);
    let _rc_clone3 = Rc::clone(&rc_value); // Prefix with underscore
    
    println!("Original: {}", rc_value);
    println!("Clone 1: {}", rc_clone1);
    println!("Clone 2: {}", rc_clone2);
    println!("Reference count: {}", Rc::strong_count(&rc_value));
    println!("Same memory address? {}", 
        Rc::as_ptr(&rc_value) == Rc::as_ptr(&rc_clone1));
    
    // RC with custom struct
    #[derive(Debug)]
    struct SharedConfig {
        name: String,
        version: i32,
    }
    
    let config = Rc::new(SharedConfig {
        name: String::from("AppConfig"),
        version: 1,
    });
    
    let config_clone = Rc::clone(&config);
    println!("Shared config name: {}, version: {}", 
             config.name, config.version);
    println!("Config clone name: {}, version: {}", 
             config_clone.name, config_clone.version);
    println!("Reference count: {}", Rc::strong_count(&config));
    
    // ------------------------------------------------------------
    // 3. ARC<T> - ATOMIC REFERENCE COUNTED (THREAD-SAFE)
    // ------------------------------------------------------------
    println!("\n\n3. ARC<T> - ATOMIC REFERENCE COUNTED (THREAD-SAFE)");
    println!("{}", "=".repeat(50));
    
    let arc_value = Arc::new(42);
    println!("Initial value: {}", arc_value);
    println!("Initial ref count: {}", Arc::strong_count(&arc_value));
    
    // Share across threads
    let arc_clone1 = Arc::clone(&arc_value);
    let arc_clone2 = Arc::clone(&arc_value);
    
    let thread1 = thread::spawn(move || {
        println!("Thread 1: Value = {}", arc_clone1);
    });
    
    let thread2 = thread::spawn(move || {
        println!("Thread 2: Value = {}", arc_clone2);
    });
    
    thread1.join().unwrap();
    thread2.join().unwrap();
    println!("Main thread still has: {}", arc_value);
    
    // Arc with Mutex for shared mutable state
    let shared_counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for i in 0..3 {
        let counter = Arc::clone(&shared_counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += i;
            println!("Thread {}: Counter = {}", i, *num);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final counter value: {}", *shared_counter.lock().unwrap());
    
    // ------------------------------------------------------------
    // 4. CELL<T> AND REFCELL<T> - INTERIOR MUTABILITY
    // ------------------------------------------------------------
    println!("\n\n4. CELL<T> AND REFCELL<T> - INTERIOR MUTABILITY");
    println!("{}", "=".repeat(50));
    
    let cell_value = Cell::new(42);
    cell_value.set(100);
    println!("Cell value: {}", cell_value.get());
    
    let refcell_value = RefCell::new(String::from("Hello"));
    {
        let mut borrowed = refcell_value.borrow_mut();
        borrowed.push_str(", World!");
    }
    println!("RefCell value: {}", refcell_value.borrow());
    
    // ------------------------------------------------------------
    // 5. RC<REFCELL<T>> - SHARED MUTABLE OWNERSHIP
    // ------------------------------------------------------------
    println!("\n\n5. RC<REFCELL<T>> - SHARED MUTABLE OWNERSHIP");
    println!("{}", "=".repeat(50));
    
    let shared_mutable = Rc::new(RefCell::new(0));
    
    let clone1 = Rc::clone(&shared_mutable);
    let clone2 = Rc::clone(&shared_mutable);
    let clone3 = Rc::clone(&shared_mutable);
    
    *clone1.borrow_mut() += 1;
    *clone2.borrow_mut() += 2;
    *clone3.borrow_mut() += 3;
    
    println!("Shared mutable value: {}", *shared_mutable.borrow());
    println!("Reference count: {}", Rc::strong_count(&shared_mutable));
    
    // ------------------------------------------------------------
    // 6. ARC<MUTEX<T>> - THREAD-SAFE SHARED MUTABLE STATE
    // ------------------------------------------------------------
    println!("\n\n6. ARC<MUTEX<T>> - THREAD-SAFE SHARED MUTABLE STATE");
    println!("{}", "=".repeat(50));
    
    let shared_data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let mut thread_handles = vec![];
    
    for i in 0..3 {
        let data = Arc::clone(&shared_data);
        let handle = thread::spawn(move || {
            let mut vec = data.lock().unwrap();
            vec.push(10 + i);
            println!("Thread {} added {}, vector: {:?}", i, 10 + i, vec);
        });
        thread_handles.push(handle);
    }
    
    for handle in thread_handles {
        handle.join().unwrap();
    }
    
    println!("Final vector: {:?}", shared_data.lock().unwrap());
    
    // ------------------------------------------------------------
    // 7. CUSTOM SMART POINTER
    // ------------------------------------------------------------
    println!("\n\n7. CUSTOM SMART POINTER");
    println!("{}", "=".repeat(50));
    
    struct MySmartPointer<T> {
        data: Box<T>,
        metadata: String,
    }
    
    impl<T> MySmartPointer<T> {
        fn new(value: T, meta: &str) -> Self {
            MySmartPointer {
                data: Box::new(value),
                metadata: meta.to_string(),
            }
        }
        
        fn get_metadata(&self) -> &str {
            &self.metadata
        }
    }
    
    impl<T> Deref for MySmartPointer<T> {
        type Target = T;
        
        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }
    
    impl<T> Drop for MySmartPointer<T> {
        fn drop(&mut self) {
            println!("Dropping MySmartPointer with metadata: {}", self.metadata);
        }
    }
    
    let my_ptr = MySmartPointer::new(42, "My custom pointer");
    println!("Value: {}", *my_ptr);
    println!("Metadata: {}", my_ptr.get_metadata());
    
    {
        let inner_ptr = MySmartPointer::new(String::from("Hello"), "Temporary");
        println!("Inner value: {}", *inner_ptr);
    }
    
    // ------------------------------------------------------------
    // 8. WEAK<T> - NON-OWNING REFERENCES
    // ------------------------------------------------------------
    println!("\n\n8. WEAK<T> - NON-OWNING REFERENCES");
    println!("{}", "=".repeat(50));
    
    let rc_string = Rc::new(String::from("Strong Reference"));
    let weak_ref = Rc::downgrade(&rc_string);
    
    println!("Strong count: {}", Rc::strong_count(&rc_string));
    println!("Weak count: {}", Rc::weak_count(&rc_string));
    
    match weak_ref.upgrade() {
        Some(strong_ref) => println!("Upgraded successfully: {}", strong_ref),
        None => println!("Value was dropped"),
    }
    
    // Circular reference prevention with Weak
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }
    
    let leaf = Rc::new(Node {
        value: 1,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    
    let branch = Rc::new(Node {
        value: 0,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    
    println!("Branch strong count: {}", Rc::strong_count(&branch));
    println!("Branch weak count: {}", Rc::weak_count(&branch));
    println!("Leaf strong count: {}", Rc::strong_count(&leaf));
    
    // Access and use children field
    {
        let leaf_children = leaf.children.borrow();
        println!("Leaf has {} children", leaf_children.len());
    }
    
    if let Some(parent) = leaf.parent.borrow().upgrade() {
        println!("Leaf's parent value: {}", parent.value);
        let branch_children = parent.children.borrow();
        println!("Branch has {} children", branch_children.len());
    }
    
    // ------------------------------------------------------------
    // 9. COMMON USE CASES AND PATTERNS
    // ------------------------------------------------------------
    println!("\n\n9. COMMON USE CASES AND PATTERNS");
    println!("{}", "=".repeat(50));
    
    // Pattern 1: Trait objects with Box
    trait Animal {
        fn speak(&self);
    }
    
    struct Dog;
    struct Cat;
    
    impl Animal for Dog {
        fn speak(&self) {
            println!("Woof!");
        }
    }
    
    impl Animal for Cat {
        fn speak(&self) {
            println!("Meow!");
        }
    }
    
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat),
    ];
    
    println!("Animals speaking:");
    for animal in animals {
        animal.speak();
    }
    
    // Pattern 2: Builder pattern with Box
    #[derive(Debug)]
    struct ComplexObject {
        parts: Vec<Box<str>>,
    }
    
    impl ComplexObject {
        fn display_parts(&self) {
            println!("Object has {} parts: {:?}", self.parts.len(), self.parts);
        }
    }
    
    struct ComplexObjectBuilder {
        parts: Vec<Box<str>>,
    }
    
    impl ComplexObjectBuilder {
        fn new() -> Self {
            ComplexObjectBuilder { parts: Vec::new() }
        }
        
        fn add_part(mut self, part: &str) -> Self {
            self.parts.push(part.into());
            self
        }
        
        fn build(self) -> ComplexObject {
            ComplexObject { parts: self.parts }
        }
    }
    
    let complex = ComplexObjectBuilder::new()
        .add_part("part1")
        .add_part("part2")
        .build();
    
    complex.display_parts();
    
    // Pattern 3: Observer pattern
    println!("\nPattern 3: Observer pattern:");
    
    trait Observer {
        fn update(&self, state: i32);
    }
    
    struct Subject {
        observers: RefCell<Vec<Rc<dyn Observer>>>,
        state: i32,
    }
    
    impl Subject {
        fn new() -> Self {
            Subject {
                observers: RefCell::new(Vec::new()),
                state: 0,
            }
        }
        
        fn attach(&self, observer: Rc<dyn Observer>) {
            self.observers.borrow_mut().push(observer);
        }
        
        fn set_state(&mut self, state: i32) {
            self.state = state;
            self.notify_observers();
        }
        
        fn notify_observers(&self) {
            for observer in self.observers.borrow().iter() {
                observer.update(self.state);
            }
        }
    }
    
    struct ConcreteObserver {
        name: String,
    }
    
    impl Observer for ConcreteObserver {
        fn update(&self, state: i32) {
            println!("Observer {} notified: state changed to {}", 
                     self.name, state);
        }
    }
    
    let mut subject = Subject::new();
    let obs1: Rc<dyn Observer> = Rc::new(ConcreteObserver { 
        name: "Observer 1".to_string() 
    });
    let obs2: Rc<dyn Observer> = Rc::new(ConcreteObserver { 
        name: "Observer 2".to_string() 
    });
    
    subject.attach(obs1);
    subject.attach(obs2);
    subject.set_state(10);
    subject.set_state(20);
    
    // ------------------------------------------------------------
    // 10. MEMORY LEAK DETECTION AND PREVENTION
    // ------------------------------------------------------------
    println!("\n\n10. MEMORY LEAK DETECTION AND PREVENTION");
    println!("{}", "=".repeat(50));
    
    struct TreeNode {
        name: String,
        parent: RefCell<Weak<TreeNode>>,
        children: RefCell<Vec<Rc<TreeNode>>>,
    }
    
    impl TreeNode {
        fn new(name: &str) -> Rc<Self> {
            Rc::new(TreeNode {
                name: name.to_string(),
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(Vec::new()),
            })
        }
        
        fn add_child(parent: &Rc<TreeNode>, child: &Rc<TreeNode>) {
            parent.children.borrow_mut().push(Rc::clone(child));
            *child.parent.borrow_mut() = Rc::downgrade(parent);
        }
        
        fn display_info(&self) {
            println!("Node: {}", self.name);
            let children_count = self.children.borrow().len();
            println!("  Children count: {}", children_count);
        }
    }
    
    let root = TreeNode::new("root");
    let child1 = TreeNode::new("child1");
    let child2 = TreeNode::new("child2");
    
    TreeNode::add_child(&root, &child1);
    TreeNode::add_child(&root, &child2);
    
    root.display_info();
    child1.display_info();
    
    println!("Root strong count: {}", Rc::strong_count(&root));
    println!("Root weak count: {}", Rc::weak_count(&root));
    println!("Child1 strong count: {}", Rc::strong_count(&child1));
    
    // ------------------------------------------------------------
    // 11. SUMMARY AND COMPARISON
    // ------------------------------------------------------------
    println!("\n\n11. SMART POINTERS COMPARISON");
    println!("{}", "=".repeat(50));
    
    println!("Box<T>:");
    println!("  • Moves data to heap");
    println!("  • Single ownership");
    println!("  • Fixed size (one pointer)");
    println!("  • Use: Recursive types, large data, trait objects");
    
    println!("\nRc<T>:");
    println!("  • Reference counted");
    println!("  • Multiple owners, immutable");
    println!("  • Single-threaded only");
    println!("  • Use: Shared read-only data in single thread");
    
    println!("\nArc<T>:");
    println!("  • Atomic reference counted");
    println!("  • Thread-safe version of Rc");
    println!("  • Slightly more overhead");
    println!("  • Use: Shared data across threads");
    
    println!("\nCell<T>:");
    println!("  • Interior mutability for Copy types");
    println!("  • Zero-cost abstraction");
    println!("  • Use: Simple types that need mutation through &self");
    
    println!("\nRefCell<T>:");
    println!("  • Interior mutability with runtime checking");
    println!("  • Borrow checking at runtime");
    println!("  • Use: Complex types needing mutation through &self");
    
    println!("\nRc<RefCell<T>>:");
    println!("  • Shared mutable ownership");
    println!("  • Single-threaded");
    println!("  • Use: Graphs, UI trees, observers");
    
    println!("\nArc<Mutex<T>>:");
    println!("  • Thread-safe shared mutable state");
    println!("  • Runtime locking");
    println!("  • Use: Shared state in concurrent programs");
    
    println!("\nWeak<T>:");
    println!("  • Non-owning reference");
    println!("  • Prevents reference cycles");
    println!("  • Use: Parent-child relationships, caches");
    
    println!("\n=== SMART POINTERS DEMONSTRATION COMPLETE ===");
}