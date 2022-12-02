use petgraph::graph::{EdgeIndex, NodeIndex};
use rustgql::graph::Graph as RQLGraph;

use crate::person_graph_types::{new_student, FriendOf};

///
/// Defines simple tests for the graph API using a Petgraph backend.
///
// Declare to use code in the module/file ./person_graph_types.rs
pub mod person_graph_types;

///
/// Assert Node indices from 0 to 3. Petgraph should
/// guarantee these indices in a graph without deletion.
///
#[test]
fn query_node_indices() {
    let graph = person_graph_types::make_sample_graph().0;
    let node_indices: Vec<_> = graph.node_indices().map(|n| n.index()).collect();
    assert_eq!(node_indices, vec![0, 1, 2, 3]);
}

///
/// Assert Edge indices from 0 to 4.
///
#[test]
fn query_edge_indices() {
    let graph = person_graph_types::make_sample_graph().0;
    let edge_indices: Vec<_> = graph.edge_indices().map(|e| e.index()).collect();
    assert_eq!(edge_indices, vec![0, 1, 2, 3, 4]);
}

///
/// Query that the node references and weights are as we inserted them.
/// Also test the connections from nodes (which nodes are connected with what edges?)
///
#[test]
fn query_node_properties() {
    let (base_graph, node_data, edge_data) = person_graph_types::make_sample_graph();
    let graph = person_graph_types::into_trait_object(base_graph);

    // Check nodes on their own.
    for (index, weight) in node_data.iter() {
        assert!(graph.node_weight(*index) == weight);

        // Query adjacent edges from the graph...
        let mut adjacent_edges: Vec<usize> =
            graph.adjacent_edges(*index).map(|e| e.index()).collect();
        adjacent_edges.sort();

        // ... and from the data we put into it.
        let mut actual_adjacent_edges: Vec<_> = edge_data
            .iter()
            .filter(|(_, (a, b, _))| a == index || b == index)
            .map(|(e, _)| e.index())
            .collect();
        actual_adjacent_edges.sort();
        assert_eq!(adjacent_edges, actual_adjacent_edges);

        // Repeat this process for outgoing edges.
        let mut outgoing_edges: Vec<_> = graph.outgoing_edges(*index).map(|e| e.index()).collect();
        outgoing_edges.sort();
        let mut actual_outgoing_edges: Vec<_> = edge_data
            .iter()
            .filter(|(_, (a, _, _))| a == index)
            .map(|(e, _)| e.index())
            .collect();
        actual_outgoing_edges.sort();
        assert_eq!(outgoing_edges, actual_outgoing_edges);

        // And once again for incoming edges.
        let mut incoming_edges: Vec<_> = graph.incoming_edges(*index).map(|e| e.index()).collect();
        incoming_edges.sort();
        let mut actual_incoming_edges: Vec<_> = edge_data
            .iter()
            .filter(|(_, (_, a, _))| a == index)
            .map(|(e, _)| e.index())
            .collect();
        actual_incoming_edges.sort();
        assert_eq!(incoming_edges, actual_incoming_edges);
    }
}

///
/// Query that all edges are correctly defined, that their weights are what we expect, and their
/// endpoints are correctly set.
///
#[test]
fn query_edge_properties() {
    let (base_graph, _, edge_data) = person_graph_types::make_sample_graph();
    let graph = person_graph_types::into_trait_object(base_graph);
    // Graph direction, edge directions should be ok.
    assert!(graph.is_directed());

    // Check single edges in both directions.
    for (e_idx, (source_idx, target_idx, e_weight)) in edge_data.iter() {
        assert!(graph.is_directed_edge(*e_idx));
        assert_eq!(graph.adjacent_nodes(*e_idx), (*source_idx, *target_idx));
        assert_eq!(graph.edge_weight(*e_idx), e_weight);
    }
}

///
/// Check if node references are set correctly.
/// Graph should panic when I input an invalid index.
///
#[test]
#[should_panic(expected = "Couldn't find edge weight: Edge reference invalid.")]
fn check_edge_references() {
    let graph = person_graph_types::into_trait_object(person_graph_types::make_sample_graph().0);
    // Incorrect index/out of range.
    let faulty_idx = EdgeIndex::from(65);

    for idx in (0..4).map(|num| EdgeIndex::from(num as u32)) {
        let _x = graph.edge_weight(idx);
    }

    let _panic_provoke = graph.edge_weight(faulty_idx);
}

///
/// Check if node references are set correctly.
/// Graph should only return true when I input the same indices.
/// Also check I don't get a node for an invalid index.
///
#[test]
#[should_panic(expected = "Couldn't find node weight: Node reference invalid.")]
fn check_node_references() {
    let graph = person_graph_types::into_trait_object(person_graph_types::make_sample_graph().0);
    let faulty_idx = NodeIndex::from(45);

    for idx in graph.nodes() {
        let _weight = graph.node_weight(idx);
    }

    let _panic_provoke = graph.node_weight(faulty_idx);
}

///
/// Checks special properties for undirected graphs:
/// 1. Graph is undirected.
/// 2. Every edge is undirected.
/// 3. adjacent_edges, outgoing_edges, incoming_edges all yield the same result.
/// 4. adjacent_nodes yields the correct nodes.
///
#[test]
fn check_undirected_edges() {
    let (tramways, stations, routes) = person_graph_types::make_sample_graph_undirected();
    let graph = person_graph_types::into_trait_object(tramways);

    assert!(!graph.is_directed());
    assert!(!routes.keys().any(|edge| graph.is_directed_edge(*edge)));

    for station_idx in stations.keys() {
        let mut actual_routes: Vec<_> = routes
            .iter()
            .filter(|(_, (f, t, _))| f == station_idx || t == station_idx)
            .map(|(e, _)| e.index())
            .collect();
        actual_routes.sort();

        let mut outgoing_edges: Vec<_> = graph
            .outgoing_edges(*station_idx)
            .map(|e| e.index())
            .collect();
        outgoing_edges.sort();

        // TODO Don't make me access e.index()/Petgraph interna anymore. Hide that
        let mut incoming_edges: Vec<_> = graph
            .incoming_edges(*station_idx)
            .map(|e| e.index())
            .collect();
        incoming_edges.sort();

        let mut adjacent_edges: Vec<_> = graph
            .adjacent_edges(*station_idx)
            .map(|e| e.index())
            .collect();
        adjacent_edges.sort();

        assert_eq!(actual_routes, outgoing_edges);
        assert_eq!(actual_routes, incoming_edges);
        assert_eq!(actual_routes, adjacent_edges);
    }

    for (e_idx, (f, t, _)) in routes.iter() {
        let (start, end) = graph.adjacent_nodes(*e_idx);
        assert!((start == *f && end == *t) || (start == *t && end == *f));
    }
}

///
/// Tests that we reject invalid edge indices for undirected graphs.
///
#[test]
#[should_panic]
fn wrong_edge_index_directed_test() {
    let graph =
        person_graph_types::into_trait_object(person_graph_types::make_sample_graph_undirected().0);
    let wrong_idx = EdgeIndex::from(42);
    graph.is_directed_edge(wrong_idx);
}

///
/// Simple Graph Test.
///
#[test]
fn trial_and_error() {
    let graph =
        person_graph_types::into_trait_object(person_graph_types::make_sample_graph_variant());
    assert_eq!(graph.nodes().count(), 4);
    assert_eq!(graph.edges().count(), 4);

    let tobias = graph.nodes().next().unwrap();
    assert_eq!(
        *graph.node_weight(tobias),
        new_student("tobias", 99, 900000)
    );

    assert_eq!(graph.adjacent_edges(tobias).count(), 1);
    assert_eq!(graph.outgoing_edges(tobias).count(), 1);
    assert_eq!(graph.incoming_edges(tobias).count(), 0);

    let tobi_and_horst = graph.adjacent_edges(tobias).next().unwrap();

    let x = FriendOf::new(2020);
    assert!(graph.is_directed());
    assert!(graph.is_directed_edge(tobi_and_horst));
    assert_eq!(*graph.edge_weight(tobi_and_horst), x);
}
