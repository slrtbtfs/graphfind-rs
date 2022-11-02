extern crate rustgql;

#[test]
/**
 * Create a simple graph and check that correct node and edge weights are delivered through the graph interface implementation.
 */
fn test_graph() {
    // node and edge types are derived from added nodes and edges.
    let mut pg = petgraph::Graph::new();
    let a = pg.add_node("a");
    let b = pg.add_node("b");
    pg.add_edge(a, b, 42);

    let g: Box<dyn rustgql::graph::Graph<NodeType = &str, EdgeType = i32>> = Box::new(pg);

    let node_weights: Vec<&&str> = g.node_weights().collect();
    assert_eq!(node_weights, vec![&"a", &"b"]);

    let edge_weights: Vec<&i32> = g.edge_weights().collect();
    assert_eq!(edge_weights, vec![&42]);
}
