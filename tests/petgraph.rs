use rustgql::graph::Graph;

extern crate rustgql;


fn test_graph() -> impl rustgql::graph::Graph<String, i32> {
    let mut g = petgraph::graph::Graph::new();
    let a = g.add_node(String::from("a"));
    let b = g.add_node(String::from("b"));
    g.add_edge(a, b, 42);

    g
}

#[test]
/**
 * Create a simple graph and check that correct node and edge weights are delivered through the Graph trait implementation.
 */
fn test_graph_weights() {
    // Vague explanation: Cast into a rustgql::Graph trait object.
    // More precise: Create a Box (in C++ that would be a std::unique_ptr)
    // containing a trait implementation of with functions dynamically resolved at runtime,
    // in order to drop any petgraph functionality and keep only what's included in the Graph trait.
    let g = test_graph();

    let x: Vec<String> = g.node_weights().cloned().collect();
    assert_eq!(x, vec!["a", "b"]);

    let weights = g.node_weights();
    let node_weights: Vec<String> = weights.cloned().collect();
    assert_eq!(node_weights, vec!["a", "b"]);

    let edge_weights: Vec<&i32> = g.edge_weights().collect();
    assert_eq!(edge_weights, vec![&42]);

    assert!(g.is_directed())
}
