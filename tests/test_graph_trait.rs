/**
 * Defines simple tests for the graph API using a Petgraph backend.
 */
// Declare to use code in the module/file ./person_graph_types.rs
pub mod person_graph_types;
 

use petgraph::stable_graph::StableGraph;

// Use Code
use crate::person_graph_types::{FriendOf, Person};

/// In this case, the best fix would be to return an owned data type rather than a
/// reference so the calling function is then responsible for cleaning up the value.

fn make_sample_graph<'a>() ->
    StableGraph<Person, FriendOf> {
    // Graph maintains pointers to Person, and FriendOf.
    let mut graph: StableGraph<Person, FriendOf> = StableGraph::new();


    // Student 1/Tobias
    let tobias = person_graph_types::new_student("tobias", 99, 900000);
    let t = graph.add_node(tobias);

    // Student 2/Stefan
    let stefan = person_graph_types::new_student("stefan", 9, 89000);
    let s = graph.add_node(stefan);

    // Student 3/Horst
    let horst = person_graph_types::new_student("horst", 55, 823340);
    let h = graph.add_node(horst);

    // Professor/Bettina
    let bettina = person_graph_types::new_professor(
        "bettina",
        36,
        "Faculty of Software Engineering and Programming Langauges",
    );
    let b = graph.add_node(bettina);

    // Connect with edges:
    // Tobias is a friend of Horst
    // Horst and Bettina are friends
    // Stefan is a friend of Bettina
    let x = FriendOf::new(2020);
    graph.add_edge(t, h, x);
    let x = FriendOf::new(2010);
    graph.add_edge(h, b, x);
    let x = FriendOf::new(2010);
    graph.add_edge(b, h, x);
    graph.add_edge(s, b, FriendOf::new(2018));

    graph
}

#[test]
fn trial_and_error() {
    let graph = make_sample_graph();
    let (nodes, edges) = graph.capacity();
    assert_eq!(4, nodes);
    assert_eq!(4, edges);
}