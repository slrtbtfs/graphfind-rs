/**
 * Defines simple tests for the graph API using a Petgraph backend.
 */
// Declare to use code in the module/file ./person_graph_types.rs
pub mod person_graph_types;

use std::{collections::HashMap, vec};

use itertools::Itertools;
use petgraph::{
    graph::DefaultIx,
    graph::Graph as BaseGraph,
    stable_graph::{EdgeIndex, NodeIndex},
    Directed,
};
use rustgql::graph::Graph as RQLGraph;

// Use Code
use crate::person_graph_types::{FriendOf, Person};

/// In this case, the best fix would be to return an owned data type rather than a
/// reference so the calling function is then responsible for cleaning up the value.
fn make_sample_graph<'a>() -> (
    BaseGraph<Person, FriendOf>,
    HashMap<NodeIndex, Person>,
    HashMap<EdgeIndex, (NodeIndex, NodeIndex, FriendOf)>,
) {
    // Graph maintains enumbs of Person, and FriendOf.
    let mut graph: BaseGraph<Person, FriendOf, Directed, DefaultIx> = BaseGraph::new();
    let mut node_raw = HashMap::new();
    let mut edge_raw = HashMap::new();

    // Student 1/Tobias
    let tobias = person_graph_types::new_student("tobias", 99, 900000);
    let x = tobias.clone();
    let t = graph.add_node(tobias);
    node_raw.insert(t, x);

    // Student 2/Stefan
    let stefan = person_graph_types::new_student("stefan", 9, 89000);
    let x = stefan.clone();
    let s = graph.add_node(stefan);
    node_raw.insert(s, x);

    // Student 3/Horst
    let horst = person_graph_types::new_student("horst", 55, 823340);
    let x = horst.clone();
    let h = graph.add_node(horst);
    node_raw.insert(h, x);

    // Professor/Bettina
    let bettina = person_graph_types::new_professor(
        "bettina",
        36,
        "Faculty of Software Engineering and Programming Langauges",
    );
    let x = bettina.clone();
    let b = graph.add_node(bettina);
    node_raw.insert(b, x);

    // Connect with edges:
    // Tobias is a friend of Horst
    // Horst and Bettina are friends
    // Stefan is a friend of Bettina and vice versa
    let x = FriendOf::new(2020);
    edge_raw.insert(graph.add_edge(t, h, x), (t, h, x));
    let x = FriendOf::new(2010);
    edge_raw.insert(graph.add_edge(h, b, x), (h, b, x));
    let x = FriendOf::new(2010);
    graph.add_edge(b, h, x);

    let x = FriendOf::new(2018);
    edge_raw.insert(graph.add_edge(s, b, x), (s, b, x));
    edge_raw.insert(graph.add_edge(b, s, x), (b, s, x));
    (graph, node_raw, edge_raw)
}

#[test]
fn query_node_indices() {
    // Assert Node indices from 0 to 3. Petgraph should
    // guarantee these indices in a graph without deletion.
    let graph: Box<dyn RQLGraph<Person, FriendOf, _, _>> = Box::new(make_sample_graph().0);
    let node_indices: Vec<_> = graph.nodes().map(|n| n.index()).collect();
    assert_eq!(node_indices, vec![0, 1, 2, 3]);
}

#[test]
fn query_edge_indices() {
    // Assert Edge indices from 0 to 4.
    let graph: Box<dyn RQLGraph<Person, FriendOf, _, _>> = Box::new(make_sample_graph().0);
    let edge_indices: Vec<_> = graph.edges().map(|e| e.index()).collect();
    assert_eq!(edge_indices, vec![0, 1, 2, 3, 4]);
}

/// Query that the node references and weights are as we inserted them.
///
/// Also test the connections from nodes (which nodes are connected with what edges?)
#[test]
fn query_node_properties() {
    let (base_graph, node_data, edge_data) = make_sample_graph();
    let graph: Box<dyn RQLGraph<Person, FriendOf, _, _>> = Box::new(base_graph);

    // Check nodes on their own.
    for (index, weight) in node_data.iter() {
        assert!(graph.node_weight(*index).unwrap() == weight);

        // Query adjacent edges from the graph...
        let outgoing_edges: Vec<_> = graph
            .adjacent_edges(index)
            .map(|e| e.index())
            .sorted()
            .collect();

        // ... and from the data we put into it.
        let actual_outgoing_edges: Vec<_> = edge_data
            .iter()
            .filter(|(_, (a, b, _))| a == index || b == index)
            .map(|(e, _)| e.index())
            .sorted()
            .collect();
        // TODO: Fixme
        //assert_eq!(outgoing_edges, actual_outgoing_edges);
    }
}

/// Query that all edges are correctly defined,
/// that their weights are what we expect, and their
/// endpoints are correclty set.
#[test]
fn query_edge_properties() {
    let (base_graph, _, edge_data) = make_sample_graph();
    let graph: Box<dyn RQLGraph<Person, FriendOf, _, _>> = Box::new(base_graph);
    // Graph direction, edge directions should be ok.
    assert!(graph.is_directed());

    // Check single edges.
    for (e_idx, (source_idx, target_idx, e_weight)) in edge_data.iter() {
        // TODO: fixme
        //assert!(graph.is_directed_edge(*e_idx).unwrap());
        assert_eq!(
            graph.adjacent_nodes(*e_idx).unwrap(),
            (*source_idx, *target_idx)
        );
        assert_eq!(graph.edge_weight(*e_idx).unwrap(), e_weight);
    }
}

/// Check if node references are set correctly.
/// Graph should only return true when I input the same indices.
///
/// Also check I don't get an edge for an invalid index.
#[test]
fn check_edge_references() {
    let graph: Box<dyn RQLGraph<Person, FriendOf, _, _>> = Box::new(make_sample_graph().0);
    // Incorrect index/out of range.
    let faulty_idx = EdgeIndex::from(65_u32);

    for idx in (0..4).map(|num| EdgeIndex::from(num as u32)) {
        assert!(graph.do_ref_same_edge(idx, idx));
        assert!(graph.edge_weight(idx).is_some());
        assert!(!graph.do_ref_same_edge(idx, EdgeIndex::from((idx.index() + 3) as u32)));
        assert!(!graph.do_ref_same_edge(idx, faulty_idx));
    }

    assert!(graph.edge_weight(faulty_idx).is_none());
}

/// Check if node references are set correctly.
/// Graph should only return true when I input the same indices.
///
/// Also check I don't get a node for an invalid index.
#[test]
fn check_node_references() {
    let graph: Box<dyn RQLGraph<Person, FriendOf, _, _>> = Box::new(make_sample_graph().0);
    let node_indices = graph.nodes();
    let faulty_idx = NodeIndex::from(45);

    for idx in node_indices {
        let other_idx = NodeIndex::from(idx.index() as u32 + 1);

        assert!(graph.do_ref_same_node(idx, idx));
        assert!(graph.node_weight(idx).is_some());

        assert!(!graph.do_ref_same_node(idx, other_idx));
        assert!(!graph.do_ref_same_node(faulty_idx, idx));
    }

    assert!(graph.node_weight(faulty_idx).is_none());
}
