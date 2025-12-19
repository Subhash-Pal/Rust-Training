use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;

// We do NOT derive Debug automatically anymore
struct Node {
    value: i32,
    neighbors: RefCell<Vec<Rc<Node>>>,
}

// Manually implement the Debug trait to prevent infinite recursion
impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
         .field("value", &self.value)
         // Instead of printing the full neighbors vector recursively,
         // we just print the count of neighbors to break the cycle during debug output.
         .field("neighbors_count", &self.neighbors.borrow().len())
         .finish()
    }
}

fn main() {
    let node1 = Rc::new(Node {
        value: 1,
        neighbors: RefCell::new(vec![]),
    });
    let node2 = Rc::new(Node {
        value: 2,
        neighbors: RefCell::new(vec![]),
    });

    // Create the cycle
    node1.neighbors.borrow_mut().push(Rc::clone(&node2));
    node2.neighbors.borrow_mut().push(Rc::clone(&node1));

    // This print statement now works without crashing because of the custom Debug impl
    println!("Node 1 details: {:?}", node1);
    println!("Node 2 details: {:?}", node2);
    println!("\nThe cycle exists, but the 'Debug' output is safe from stack overflow.");
}
