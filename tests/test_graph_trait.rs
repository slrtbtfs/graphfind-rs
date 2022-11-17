///
/// Defines simple tests for the graph API using a Petgraph backend.
///
// Declare to use code in the module/file ./person_graph_types.rs
pub mod person_graph_types;

#[test]
fn trial_and_error() {
    let graph = person_graph_types::make_sample_graph();
    let (nodes, edges) = graph.capacity();
    assert_eq!(4, nodes);
    assert_eq!(4, edges);
}
