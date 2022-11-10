use rustgql::graph::Graph;

extern crate rustgql;

#[test]
/**
 * Create a simple graph and check that correct node and edge weights are delivered through the Graph trait implementation.
 */
fn test_graph_weights() {
    // node and edge types are derived from added nodes and edges.
    let mut pg = petgraph::graph::Graph::new();
    let a = pg.add_node("a");
    let b = pg.add_node("b");
    pg.add_edge(a, b, 42);

    // Vague explanation: Cast into a rustgql::Graph trait object.
    // More precise: Create a Box (in C++ that would be a std::unique_ptr)
    // containing a trait implementation of with functions dynamically resolved at runtime,
    // in order to drop any petgraph functionality and keep only what's included in the Graph trait.
    let g: Box<dyn Graph<_, _, _, _>> = Box::new(pg);

    let x: Vec<&str> = g.node_weights().copied().collect();
    assert_eq!(x, vec!["a", "b"]);

    let weights = g.node_weights();
    let node_weights: Vec<&str> = weights.map(|x| *x).collect();
    assert_eq!(node_weights, vec!["a", "b"]);

    let edge_weights: Vec<&i32> = g.edge_weights().collect();
    assert_eq!(edge_weights, vec![&42]);
}
