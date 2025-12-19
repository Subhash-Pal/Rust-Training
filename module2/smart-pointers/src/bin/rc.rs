use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct FileSystemNode {
    name: String,
    parent: RefCell<Weak<FileSystemNode>>,  // Weak to avoid cycles
    children: RefCell<Vec<Rc<FileSystemNode>>>,  // Shared ownership of children
}

impl FileSystemNode {
    fn new(name: &str) -> Rc<Self> {
        Rc::new(Self {
            name: name.to_string(),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(Vec::new()),
        })
    }
    
    fn add_child(parent: &Rc<Self>, child: &Rc<Self>) {
        child.parent.replace(Rc::downgrade(parent));
        parent.children.borrow_mut().push(Rc::clone(child));
    }
    
    fn print_path(&self) {
        let mut path = Vec::new();
        let mut current = self;
        
        path.push(&current.name);
        
        // Try to get parent via weak reference
        while let Some(parent) = current.parent.borrow().upgrade() {
            path.push(&parent.name);
            current = &*parent;
        }
        
        println!("{}", path.iter().rev().map(|s| s.as_str()).collect::<Vec<_>>().join("/"));
    }
}

fn main() {
    let root = FileSystemNode::new("");
    let home = FileSystemNode::new("home");
    let user = FileSystemNode::new("alice");
    let docs = FileSystemNode::new("Documents");
    
    FileSystemNode::add_child(&root, &home);
    FileSystemNode::add_child(&home, &user);
    FileSystemNode::add_child(&user, &docs);
    
    // Multiple references to same node
    let docs_alias = Rc::clone(&docs);
    
    docs.print_path();      // /home/alice/Documents
    docs_alias.print_path(); // /home/alice/Documents
    
    println!("Strong count of docs: {}", Rc::strong_count(&docs)); // 3
    println!("Weak count of docs: {}", Rc::weak_count(&docs));     // 1
}