use petgraph::{
    graph::{DefaultIx, EdgeIndex, Graph as BaseGraph, NodeIndex},
    Directed, Undirected,
};
use rustgql::graph::Graph as RQLGraph;
use std::collections::HashMap;

use crate::person_graph_types::{new_student, FriendOf, Person};

///
/// Defines simple tests for the graph API using a Petgraph backend.
///
// Declare to use code in the module/file ./person_graph_types.rs
pub mod person_graph_types;

fn into_trait_object<N, E>(
    g: petgraph::graph::Graph<N, E, Directed, DefaultIx>,
) -> impl rustgql::graph::Graph<
    N,
    E,
    NodeRef = petgraph::graph::NodeIndex,
    EdgeRef = petgraph::graph::EdgeIndex,
> {
    g
}

fn into_trait_object_undirected<N, E>(
    g: petgraph::graph::Graph<N, E, Undirected, DefaultIx>,
) -> impl rustgql::graph::Graph<
    N,
    E,
    NodeRef = petgraph::graph::NodeIndex,
    EdgeRef = petgraph::graph::EdgeIndex,
> {
    g
}

/// NodeInfo and EdgeInfo types that allow us to compare
/// what we inserted into a Graph and what we get back-
type NodeInfo<N> = HashMap<NodeIndex, N>;
type EdgeInfo<E> = HashMap<EdgeIndex, (NodeIndex, NodeIndex, E)>;

///
/// Creates a sample graph for testing directed graphs.
/// See (TODO add file) for an graphical overview.
///
fn make_sample_graph() -> (
    BaseGraph<Person, FriendOf>,
    NodeInfo<Person>,
    EdgeInfo<FriendOf>,
) {
    // Graph maintains enums of Person, and FriendOf.
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
    // Stefan is a friend of Bettina, and vice versa.
    let x = FriendOf::new(2020);
    edge_raw.insert(graph.add_edge(t, h, x), (t, h, x));
    let x = FriendOf::new(2010);
    edge_raw.insert(graph.add_edge(h, b, x), (h, b, x));
    let x = FriendOf::new(2010);
    edge_raw.insert(graph.add_edge(b, h, x), (b, h, x));

    let x = FriendOf::new(2018);
    edge_raw.insert(graph.add_edge(s, b, x), (s, b, x));
    edge_raw.insert(graph.add_edge(b, s, x), (b, s, x));
    (graph, node_raw, edge_raw)
}

///
/// Creates a new undirected sample graph, based in part
/// on the tramways in Ulm.
///
fn make_sample_graph_undirected<'a>() -> (
    BaseGraph<&'a str, i32, Undirected>,
    NodeInfo<&'a str>,
    EdgeInfo<i32>,
) {
    let mut g = BaseGraph::new_undirected();
    let mut stations = HashMap::new();

    // Ehinger Tor
    let e = "Ehinger Tor";
    // Theater
    let t = "Theater";
    // Science Park
    let s = "Science Park";
    // Kuhberg Schools
    let k = "Kuhberg Schulzentrum";
    // Böfingen
    let b = "Böflingen";
    // Söflingen
    let sö = "Söflingen";

    // Add stations
    let ei = g.add_node(e);
    let ti = g.add_node(t);
    let si = g.add_node(s);
    let ki = g.add_node(k);
    let bi = g.add_node(b);
    let söi = g.add_node(sö);

    stations.insert(ti, t);
    stations.insert(si, s);
    stations.insert(ei, e);
    stations.insert(ki, k);
    stations.insert(bi, b);
    stations.insert(söi, sö);

    // Connections
    let ei_ti = g.add_edge(ei, ti, 5);
    let ti_si = g.add_edge(ti, si, 12);
    let ti_ki = g.add_edge(ti, ki, 15);
    let bi_ti = g.add_edge(bi, ti, 17);
    let ei_söi = g.add_edge(ei, söi, 8);

    // Add connections
    let mut routes = HashMap::new();
    routes.insert(ei_ti, (ei, ti, 5));
    routes.insert(ti_si, (si, ti, 12));
    routes.insert(ti_ki, (ki, ti, 15));
    routes.insert(bi_ti, (bi, ti, 17));
    routes.insert(ei_söi, (ei, söi, 8));

    (g, stations, routes)
}

///
/// Assert Node indices from 0 to 3. Petgraph should
/// guarantee these indices in a graph without deletion.
///
#[test]
fn query_node_indices() {
    let graph = make_sample_graph().0;
    let node_indices: Vec<_> = graph.node_indices().map(|n| n.index()).collect();
    assert_eq!(node_indices, vec![0, 1, 2, 3]);
}

///
/// Assert Edge indices from 0 to 4.
///
#[test]
fn query_edge_indices() {
    let graph = make_sample_graph().0;
    let edge_indices: Vec<_> = graph.edge_indices().map(|e| e.index()).collect();
    assert_eq!(edge_indices, vec![0, 1, 2, 3, 4]);
}

///
/// Query that the node references and weights are as we inserted them.
/// Also test the connections from nodes (which nodes are connected with what edges?)
///
#[test]
fn query_node_properties() {
    let (base_graph, node_data, edge_data) = make_sample_graph();
    let graph = into_trait_object(base_graph);

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
    let (base_graph, _, edge_data) = make_sample_graph();
    let graph = into_trait_object(base_graph);
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
    let graph = into_trait_object(make_sample_graph().0);
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
    let graph = into_trait_object(make_sample_graph().0);
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
///
#[test]
fn check_undirected_edges() {
    let (tramways, stations, routes) = make_sample_graph_undirected();
    let graph = into_trait_object_undirected(tramways);

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
}

///
/// Simple Graph Test.
///
#[test]
fn trial_and_error() {
    let graph = into_trait_object(person_graph_types::make_sample_graph());
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
