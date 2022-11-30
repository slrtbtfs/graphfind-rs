use petgraph::{
    graph::{EdgeIndex, NodeIndex},
    stable_graph::DefaultIx,
    Graph, Undirected,
};
use rustgql::{graph::Graph as RQLGraph, graph_backends::filter_map::FilterMapGraph};

///
/// Function Tests for filter_map
///

///
/// Returns a petgraph consisting of 3000 nodes (weight 0, ..., 2999)
/// and 2000 edges (0, ..., 1999), so that edge 0 connect nodes 0 and 1,
/// edge 1 0 and 2, edge 2 nodes 3 and 4, edge 3 3 and 5, etc.
/// This graph is undirected.
///
fn make_sample_graph() -> Graph<i32, i32, Undirected> {
    let mut graph = petgraph::graph::Graph::new_undirected();

    for i in 0..1000 {
        let u1 = graph.add_node(3 * i);
        let u2 = graph.add_node(3 * i + 1);
        graph.add_edge(u1, u2, 2 * i);
        let u3 = graph.add_node(3 * i + 2);
        graph.add_edge(u1, u3, 2 * i + 1);
    }

    graph
}

///
/// Remove all even-numbered edges, and every third node.
/// Assume we get the graph we expect: edges 1, 3, ...
///
#[test]
fn test_filter_only() {
    let graph = make_sample_graph();
    let result = FilterMapGraph::weight_filter(&graph, |n| n % 3 != 2, |e| e % 2 != 1);

    // Check nodes.
    let nodes: Vec<NodeIndex<DefaultIx>> = result.nodes().collect();
    assert_eq!(nodes.len(), 2000);
    for n in nodes {
        let idx = n.index();
        assert!(idx % 3 != 2 && idx <= 3000);
        let w = *result.node_weight(n);
        let w_actual = graph.node_weight(n).unwrap();
        assert_eq!(*w, *w_actual);
    }

    // Check edges.
    let edges: Vec<EdgeIndex<DefaultIx>> = result.edges().collect();
    assert_eq!(edges.len(), 1000);
    for e in edges {
        let idx = e.index();
        assert!(idx % 2 != 1 && idx <= 2000);
        assert_eq!(*result.edge_weight(e), graph.edge_weight(e).unwrap());
        let (n1, n2) = result.adjacent_nodes(e);
        assert_eq!((3 * idx) / 2, n1.index());
        assert_eq!(((3 * idx) / 2) + 1, n2.index());
    }
}
