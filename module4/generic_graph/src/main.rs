use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

// -------- Graph Trait (Associated Types) --------
trait Graph {
    type Node;

    fn add_node(&mut self, node: Self::Node);
    fn add_edge(&mut self, from: Self::Node, to: Self::Node);
    fn neighbors(&self, node: &Self::Node) -> Option<&Vec<Self::Node>>;
}

// -------- Concrete Graph Implementation --------
#[derive(Debug)]
struct GenericGraph<T>
where
    T: Eq + Hash + Clone,
{
    adjacency_list: HashMap<T, Vec<T>>,
}

impl<T> GenericGraph<T>
where
    T: Eq + Hash + Clone,
{
    fn new() -> Self {
        Self {
            adjacency_list: HashMap::new(),
        }
    }
}

// -------- Implement Graph Trait --------
impl<T> Graph for GenericGraph<T>
where
    T: Eq + Hash + Clone,
{
    type Node = T;

    fn add_node(&mut self, node: Self::Node) {
        self.adjacency_list.entry(node).or_insert(Vec::new());
    }

    fn add_edge(&mut self, from: Self::Node, to: Self::Node) {
        self.adjacency_list
            .entry(from)
            .or_insert(Vec::new())
            .push(to);
    }

    fn neighbors(&self, node: &Self::Node) -> Option<&Vec<Self::Node>> {
        self.adjacency_list.get(node)
    }
}

// -------- Utility Function for Printing --------
fn print_graph<T>(graph: &GenericGraph<T>)
where
    T: Eq + Hash + Clone + Debug,
{
    for (node, neighbors) in &graph.adjacency_list {
        println!("{:?} -> {:?}", node, neighbors);
    }
}

// -------- Main --------
fn main() {
    let mut graph = GenericGraph::<&str>::new();

    graph.add_node("A");
    graph.add_node("B");
    graph.add_node("C");

    graph.add_edge("A", "B");
    graph.add_edge("A", "C");
    graph.add_edge("B", "C");

    println!("--- Graph Adjacency List ---");
    print_graph(&graph);

    println!("\nNeighbors of A:");
    if let Some(neighbors) = graph.neighbors(&"A") {
        for n in neighbors {
            println!("{}", n);
        }
    }
}
