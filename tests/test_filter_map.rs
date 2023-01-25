use std::vec;

use petgraph::{
    graph::{EdgeIndex, NodeIndex},
    stable_graph::DefaultIx,
    Graph, Undirected,
};
use rustgql::{filter_pattern, graph::Graph as RQLGraph, graph_backends::filter_map::FilterMap};

pub mod common;
use common::{
    into_trait_object, make_sample_graph_undirected, make_sample_graph_variant, FriendOf, Person,
    Role::Student,
};

///
/// Function Tests for filter_map
///

///
/// Returns a petgraph consisting of 3000 nodes (weight 0, ..., 2999)
/// and 2000 edges (0, ..., 1999), so that edge 0 connect nodes 0 and 1,
/// edge 1 0 and 2, edge 2 nodes 3 and 4, edge 3 3 and 5, etc.
/// This graph is undirected.
///
fn make_sample_graph_mass_filter_map() -> Graph<u64, u64, Undirected> {
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
    let graph = make_sample_graph_mass_filter_map();
    let result = FilterMap::weight_filter(&graph, |n| n % 3 != 2, |e| e % 2 != 1);

    // Check nodes.
    let nodes: Vec<NodeIndex<DefaultIx>> = result.nodes().collect();
    assert_eq!(nodes.len(), 2000);
    assert_eq!(result.count_nodes(), 2000);

    for n in nodes {
        let idx = n.index();
        assert!(idx % 3 != 2 && idx <= 3000);
        let w = *result.node_weight(n);
        let w_actual = graph.node_weight(n).unwrap();
        assert_eq!(*w, *w_actual);

        // Only one edge: Edge 2i connects nodes 3i and 3i + 1.
        let edges: Vec<_> = result.adjacent_edges(n).map(|e| e.index()).collect();
        let e_idx = if idx % 3 == 1 {
            (2 * (idx - 1)) / 3
        } else {
            (2 * idx) / 3
        };
        assert_eq!(edges, vec![e_idx]);
    }

    // Check edges.
    let edges: Vec<EdgeIndex<DefaultIx>> = result.edges().collect();
    assert_eq!(edges.len(), 1000);
    assert_eq!(result.count_edges(), 1000);

    for e in edges {
        let idx = e.index();
        assert!(idx % 2 != 1 && idx <= 2000);
        assert_eq!(*result.edge_weight(e), graph.edge_weight(e).unwrap());
        let (n1, n2) = result.adjacent_nodes(e);
        assert_eq!((3 * idx) / 2, n1.index());
        assert_eq!(((3 * idx) / 2) + 1, n2.index());
    }
}

///
/// Remove one of three nodes (0, 3, ...), thus removing all edges.
/// Triple the weights of remaining nodes. This also tests edge removal.
///
#[test]
fn test_weight_node_only() {
    let graph = make_sample_graph_mass_filter_map();
    let result = FilterMap::weight_filter_map(
        &graph,
        |n| Some(3 * n).filter(|n| n % 3 != 0),
        |e| Some(2 * e),
    );

    // Assert no edges.
    let edge_count = result.edges().count();
    assert_eq!(edge_count, 0);

    // Check nodes: Expected Values, no edges.
    for node in result.nodes() {
        let weight = result.node_weight(node);
        let idx = node.index();
        assert!(idx % 3 != 0 && idx < 3000);
        assert_eq!(3 * weight, idx as u64);

        let adj_edge_count = result.adjacent_edges(node).count();
        assert_eq!(adj_edge_count, 0);
    }
}

///
/// Take the Persons graph, and give me the names of all persons,
/// and all existing friendships that existed after 2011.
///
#[test]
fn test_edge_node_projection() {
    let graph = make_sample_graph_variant();
    let result = FilterMap::weight_filter_map(
        &graph,
        |p| Some(&p.name),
        |e| Some(e.since_year).filter(|year| *year > 2011),
    );

    // We should get still the same names.
    let mut names: Vec<_> = result.node_weights().copied().collect();
    names.sort();
    let actual_names = vec!["bettina", "horst", "stefan", "tobias"];
    assert_eq!(names, actual_names);

    // We should only get two dates.
    let mut dates: Vec<_> = result.edge_weights().copied().collect();
    dates.sort();
    let actual_dates = vec![2018, 2020];
    assert_eq!(dates, actual_dates);

    // From Tobias, only Horst
    let edges_out_t: Vec<_> = result.outgoing_edges(NodeIndex::from(0)).collect();
    assert_eq!(edges_out_t.len(), 1);
    assert_eq!(edges_out_t[0].index(), 0);
    // From Bettina, only Stefan
    let edges_in_t: Vec<_> = result.incoming_edges(NodeIndex::from(3)).collect();
    assert_eq!(edges_in_t.len(), 1);
    assert_eq!(edges_in_t[0].index(), 3);

    // We assume all edges still to be directed.
    assert!(result.is_directed());
    for e in result.edges() {
        assert!(result.is_directed_edge(e));
    }
}

///
/// Take the Tramways graph, and take all stations with >= 2 incoming/outgoing edges.
/// Use general_filter_map directly.
///
#[test]
fn test_filter_map_directly() {
    let graph = make_sample_graph_undirected().0;
    let result = FilterMap::general_filter_map(
        &graph,
        |g, n| {
            let degree = g.edges(n).count();
            Some(degree).filter(|_| degree >= 2)
        },
        |g, e| Some(("highly frequented", g.edge_weight(e).unwrap())),
    );

    // Two nodes; new data is degree (3).
    // Assert one connection to the same edge.
    let mut node_indices: Vec<_> = result.nodes().map(|n| n.index()).collect();
    node_indices.sort();
    assert_eq!(node_indices, vec![0, 1]);
    for n in result.nodes() {
        assert_eq!(*result.node_weight(n), 3);
        assert_eq!(result.adjacent_edges(n).count(), 1);
    }

    // One edge; data is "highly frequented" + old edge weight.
    let edge_index: Vec<_> = result.edges().collect();
    assert_eq!(edge_index[0].index(), 0);
    assert_eq!(edge_index.len(), 1);
    assert_eq!(
        *result.edge_weight(edge_index[0]),
        ("highly frequented", &5)
    );
}

///
/// Square all weights in the filter_map sample graph twice, using weight_map.
/// Assert that the structure remains unchanged.
///
#[test]
fn test_map_only() {
    let graph = into_trait_object(make_sample_graph_mass_filter_map());
    let temp_1 = FilterMap::weight_map(&graph, |n| n * n, |e| e * e);
    let result = FilterMap::weight_map(&temp_1, |n| n * n, |e| e * e);

    // Check node + edge refs.
    let mut node_refs: Vec<_> = result.nodes().collect();
    let mut node_refs_actual: Vec<_> = graph.nodes().collect();
    node_refs.sort();
    node_refs_actual.sort();
    assert_eq!(node_refs, node_refs_actual);
    for n in node_refs {
        let weight = *result.node_weight(n);
        let w = n.index() as u64;
        assert_eq!(w * w * w * w, weight);

        // Check edges.
        let mut edges: Vec<_> = result.adjacent_edges(n).collect();
        edges.sort();
        let mut edges_actual: Vec<_> = graph.adjacent_edges(n).collect();
        edges_actual.sort();
        assert_eq!(edges, edges_actual);
    }

    let mut edge_refs: Vec<_> = result.edges().collect();
    edge_refs.sort();
    let mut edge_refs_actual: Vec<_> = graph.edges().collect();
    edge_refs_actual.sort();
    assert_eq!(edge_refs, edge_refs_actual);

    for e in edge_refs {
        let weight = *result.edge_weight(e);
        let w = e.index() as u64;
        assert_eq!(w * w * w * w, weight);

        let (p1, p2) = result.adjacent_nodes(e);
        let (a1, a2) = graph.adjacent_nodes(e);
        assert_eq!(p1, a1);
        assert_eq!(p2, a2);
    }
}

///
/// Test that access to a non-existing node fails in the result graph of a filter-map.
///
#[test]
#[should_panic]
fn test_wrong_node_index() {
    let graph = into_trait_object(make_sample_graph_mass_filter_map());
    let result = FilterMap::weight_filter(&graph, |n| n % 3 != 0, |_| true);
    assert_eq!(*graph.node_weight(NodeIndex::from(0)), 0);
    let _invalid_weight = result.node_weight(NodeIndex::from(0));
}

///
/// Test that access to the adjacent non-existing node fails in the result graph of a filter-map.
///
#[test]
#[should_panic]
fn test_wrong_node_adjacent_edges() {
    let graph = into_trait_object(make_sample_graph_mass_filter_map());
    let result = FilterMap::weight_filter(&graph, |n| n % 3 != 0, |_| true);
    assert_eq!(graph.adjacent_edges(NodeIndex::from(0)).count(), 2);
    let _invalid_edge_iter = result.adjacent_edges(NodeIndex::from(0));
}

///
/// Test that access to the outgoing non-existing node fails in the result graph of a filter-map.
///
#[test]
#[should_panic]
fn test_wrong_node_outgoing_edges() {
    let graph = into_trait_object(make_sample_graph_mass_filter_map());
    let result = FilterMap::weight_filter(&graph, |n| n % 3 != 0, |_| true);
    assert_eq!(graph.outgoing_edges(NodeIndex::from(0)).count(), 2);
    let _invalid_edge_iter = result.outgoing_edges(NodeIndex::from(0));
}

///
/// Test that access to the incoming non-existing node fails in the result graph of a filter-map.
///
#[test]
#[should_panic]
fn test_wrong_node_incoming_edges() {
    let graph = into_trait_object(make_sample_graph_mass_filter_map());
    let result = FilterMap::weight_filter(&graph, |n| n % 3 != 0, |_| true);
    assert_eq!(graph.incoming_edges(NodeIndex::from(0)).count(), 0);
    let _invalid_edge_iter = result.outgoing_edges(NodeIndex::from(0));
}

///
/// Test that access to a non-existing edge fails.
///
#[test]
#[should_panic]
fn test_wrong_edge_index() {
    let graph = into_trait_object(make_sample_graph_mass_filter_map());
    let result = FilterMap::weight_filter(&graph, |n| n % 3 != 0, |_| true);
    assert_eq!(*graph.edge_weight(EdgeIndex::from(0)), 0);
    let _invalid_weight = result.edge_weight(EdgeIndex::from(0));
}

///
/// Test that access to the ends of a non-existing node fails.
///
#[test]
#[should_panic]
fn test_wrong_edge_ends() {
    let graph = into_trait_object(make_sample_graph_mass_filter_map());
    let result = FilterMap::weight_filter(&graph, |n| n % 3 != 0, |_| true);
    assert_eq!(
        graph.adjacent_nodes(EdgeIndex::from(0)),
        (NodeIndex::from(0), NodeIndex::from(1))
    );
    let (_s, _e) = result.adjacent_nodes(EdgeIndex::from(0));
}

///
/// Test that the filter_pattern macro evaluates to correct code.
///
#[test]
fn test_filter_pattern() {
    let base_graph = make_sample_graph_variant();
    // only consider students aged between 10 and 100
    let students = filter_pattern!(
        &base_graph,
        node_pattern: Person {
            role: Student { .. },
            age: 10..=100,
            ..
        }
    );
    assert_eq!(students.nodes().count(), 2);

    // only consider friendships that exist since at least 2015.
    let old_friends = filter_pattern!(&base_graph, edge_pattern: FriendOf{since_year: 0..=2015});
    assert_eq!(old_friends.edges().count(), 2);
}

///
/// Test that the filter_pattern macro is aware of the type of the matched
/// pattern.
///
#[test]
fn test_filter_pattern_result() {
    // generate a graph having Results as values
    let mut graph = petgraph::Graph::new();
    let ok = graph.add_node(Ok("ok"));
    let not_ok = graph.add_node(Err("error"));
    graph.add_edge(ok, not_ok, ());

    let ok_only = filter_pattern!(&graph, node_pattern: Ok(..));

    assert_eq!(ok_only.nodes().count(), 1);
}
