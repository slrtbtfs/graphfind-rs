use petgraph::{graph::DefaultIx, Directed};
use rustgql::graph::Graph;

use crate::person_graph_types::{new_student, FriendOf};

///
/// Defines simple tests for the graph API using a Petgraph backend.
///
// Declare to use code in the module/file ./person_graph_types.rs
pub mod person_graph_types;

fn into_trait_object<N, E>(
    g: petgraph::graph::Graph<N, E, Directed, DefaultIx>,
) -> impl rustgql::graph::Graph<N, E> {
    g
}

#[test]
fn trial_and_error() {
    let graph = into_trait_object(person_graph_types::make_sample_graph());
    assert_eq!(graph.nodes().count(), 4);
    assert_eq!(graph.edges().count(), 4);

    let tobias = graph.nodes().next().unwrap();
    assert_eq!(
        *graph.node_weight(tobias).unwrap(),
        new_student("tobias", 99, 900000)
    );

    assert_eq!(graph.adjacent_edges(tobias).count(), 1);
    assert_eq!(graph.outgoing_edges(tobias).count(), 1);
    assert_eq!(graph.incoming_edges(tobias).count(), 0);

    assert!(graph.do_ref_same_edge(
        graph.adjacent_edges(tobias).next().unwrap(),
        graph.outgoing_edges(tobias).next().unwrap()
    ));
    let tobi_and_horst = graph.outgoing_edges(tobias).next().unwrap();
    assert!(graph.do_ref_same_node(graph.adjacent_nodes(tobi_and_horst).unwrap().0, tobias));

    let x = FriendOf::new(2020);
    assert!(graph.is_directed());
    assert!(graph.is_directed_edge(tobi_and_horst).unwrap());
    assert_eq!(*graph.edge_weight(tobi_and_horst).unwrap(), x);
}
