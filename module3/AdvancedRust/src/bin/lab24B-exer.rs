// src/bin/lab24b.rs
// Lab 24B – Generic Graph Builder (FINAL FIXED VERSION)

fn main() {
    println!("=== Lab 24B – Generic Graph Builder ===\n");

    // Example 1: String nodes
    let mut graph = Graph::new();

    let node_a = "A".to_string();
    let node_b = "B".to_string();
    let node_c = "C".to_string();
    let node_d = "D".to_string();

    graph.add_node(node_a.clone());
    graph.add_node(node_b.clone());
    graph.add_node(node_c.clone());
    graph.add_node(node_d.clone());

    graph.add_edge(&node_a, &node_b, 10);
    graph.add_edge(&node_b, &node_c, 20);
    graph.add_edge(&node_c, &node_d, 30);
    graph.add_edge(&node_d, &node_a, 40);  // cycle

    println!("Graph nodes: {:?}", graph.nodes());
    println!("Neighbors of B: {:?}", graph.neighbors(&node_b));
    println!("Edge weight A-B: {:?}", graph.edge_weight(&node_a, &node_b));
    println!("Edge weight B-A: {:?}", graph.edge_weight(&node_b, &node_a));

    // Example 2: Integer nodes
    println!("\n--- Example 2: Integer graph ---");
    let mut int_graph = Graph::<u32, u32>::new();

    int_graph.add_node(1);
    int_graph.add_node(2);
    int_graph.add_node(3);

    int_graph.add_edge(&1, &2, 100);
    int_graph.add_edge(&2, &3, 200);

    println!("Int graph neighbors of 2: {:?}", int_graph.neighbors(&2));
}

// Trait for graphs
trait GraphTrait {
    type Node;
    type EdgeWeight;

    fn neighbors(&self, node: &Self::Node) -> Vec<Self::Node>;
    fn edge_weight(&self, from: &Self::Node, to: &Self::Node) -> Option<&Self::EdgeWeight>;
}

// Supertrait: MutableGraph requires GraphTrait
trait MutableGraph: GraphTrait {
    fn add_node(&mut self, node: Self::Node);
    fn add_edge(&mut self, from: &Self::Node, to: &Self::Node, weight: Self::EdgeWeight);
}

// Generic adjacency list graph
#[derive(Debug)]
struct Graph<N, W> {
    adj: std::collections::HashMap<N, Vec<(N, W)>>,
}

impl<N, W> Graph<N, W>
where
    N: std::hash::Hash + Eq + Clone,
    W: Clone,
{
    fn new() -> Self {
        Graph {
            adj: std::collections::HashMap::new(),
        }
    }

    fn nodes(&self) -> Vec<N> {
        self.adj.keys().cloned().collect()
    }
}

// Implement GraphTrait
impl<N, W> GraphTrait for Graph<N, W>
where
    N: std::hash::Hash + Eq + Clone,
    W: Clone,
{
    type Node = N;
    type EdgeWeight = W;

    fn neighbors(&self, node: &Self::Node) -> Vec<Self::Node> {
        self.adj
            .get(node)
            .unwrap_or(&vec![])
            .iter()
            .map(|(neighbor, _)| neighbor.clone())
            .collect()
    }

    fn edge_weight(&self, from: &Self::Node, to: &Self::Node) -> Option<&Self::EdgeWeight> {
        self.adj
            .get(from)?
            .iter()
            .find(|(n, _)| n == to)
            .map(|(_, weight)| weight)
    }
}

// Implement MutableGraph (fixed: clone weight for reverse edge)
impl<N, W> MutableGraph for Graph<N, W>
where
    N: std::hash::Hash + Eq + Clone,
    W: Clone,
{
    fn add_node(&mut self, node: Self::Node) {
        self.adj.entry(node).or_insert_with(Vec::new);
    }

    fn add_edge(&mut self, from: &Self::Node, to: &Self::Node, weight: Self::EdgeWeight) {
        self.adj
            .entry(from.clone())
            .or_insert_with(Vec::new)
            .push((to.clone(), weight.clone()));  // ← clone here

        // Undirected: add reverse edge
        self.adj
            .entry(to.clone())
            .or_insert_with(Vec::new)
            .push((from.clone(), weight));  // ← use original (no clone needed for last use)
    }
}