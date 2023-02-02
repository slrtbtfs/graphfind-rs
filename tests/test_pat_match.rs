pub mod common;
use std::collections::{HashMap, HashSet};

use common::{
    ActorType::Actor,
    MovieNode, MoviePerson,
    MovieType::{Movie, Tv, Video},
    Relation,
    Relation::{Knows, PlaysIn, Successor},
};
use petgraph::graph::{EdgeIndex, Graph, NodeIndex};
use rustgql::{
    graph::Graph as QueryGraph,
    query::{PatternGraph, SubgraphAlgorithm},
    query_algorithms::vf_algorithms::VfState,
};

use crate::common::ActorType;

fn add_person<'a>(
    g: &mut Graph<MovieNode, Relation>,
    names: &mut HashMap<&'a str, NodeIndex>,
    name: &'a str,
    ty: common::ActorType,
) {
    let node = common::make_person(name.to_string(), ty);
    names.insert(name, g.add_node(node));
}

fn add_media<'a>(
    g: &mut Graph<MovieNode, Relation>,
    names: &mut HashMap<&'a str, NodeIndex>,
    name: &'a str,
    rat: f64,
    year: i32,
    ty: common::MovieType,
) {
    let node = common::make_movie(name.to_string(), rat, year, ty);
    names.insert(name, g.add_node(node));
}

fn connect<NodeWeight, EdgeWeight>(
    g: &mut Graph<NodeWeight, EdgeWeight>,
    map: &HashMap<&str, NodeIndex>,
    from: &str,
    e: EdgeWeight,
    to: &str,
) {
    let node1 = map[from];
    let node2 = map[to];
    g.add_edge(node1, node2, e);
}

///
/// Gives a graph that only contains nodes. Can be modified
/// to contain different edges to match patterns on.
///
fn node_graph<'a>() -> (Graph<MovieNode, Relation>, HashMap<&'a str, NodeIndex>) {
    let mut graph: petgraph::Graph<MovieNode, Relation> = Graph::new();
    let mut names: HashMap<&str, NodeIndex> = HashMap::new();

    // 5 Actors
    add_person(&mut graph, &mut names, "stefan", Actor);
    add_person(&mut graph, &mut names, "yves", Actor);
    add_person(&mut graph, &mut names, "fabian", Actor);
    add_person(&mut graph, &mut names, "tobias", Actor);
    add_person(&mut graph, &mut names, "benedikt", Actor);

    // Media
    add_media(&mut graph, &mut names, "Jurassic Park", 10.0, 1993, Movie);
    add_media(
        &mut graph,
        &mut names,
        "Star Wars Holiday Special",
        -10.0,
        1978,
        Tv,
    );
    add_media(
        &mut graph,
        &mut names,
        "Attack of the Killer Macros",
        6.9,
        2022,
        Video,
    );
    add_media(
        &mut graph,
        &mut names,
        "Let's talk about Fight Club",
        4.2,
        2018,
        Video,
    );
    add_media(
        &mut graph,
        &mut names,
        "Star Wars: Rise of the Bechdel Test",
        2.5,
        2015,
        Video,
    );
    add_media(&mut graph, &mut names, "Sunday Uke Group", 10., 2018, Tv);

    (graph, names)
}

///
/// Produces the full graph to use for query tests. See
/// https://gitlab.informatik.uni-ulm.de/se/graphquery/rustgql/-/issues/36
/// for the structure.
///
fn full_graph<'a>() -> (
    Graph<MovieNode, Relation>,
    HashMap<&'a str, NodeIndex>,
    HashMap<&'a str, EdgeIndex>,
) {
    let (mut graph, nodes) = node_graph();
    let mut edges = HashMap::new();
    // PlaysIn Relations
    connect(&mut graph, &nodes, "stefan", PlaysIn, "Jurassic Park");
    connect(
        &mut graph,
        &nodes,
        "stefan",
        PlaysIn,
        "Star Wars Holiday Special",
    );
    connect(
        &mut graph,
        &nodes,
        "stefan",
        PlaysIn,
        "Star Wars: Rise of the Bechdel Test",
    );
    connect(&mut graph, &nodes, "yves", PlaysIn, "Jurassic Park");
    connect(
        &mut graph,
        &nodes,
        "yves",
        PlaysIn,
        "Star Wars Holiday Special",
    );
    connect(
        &mut graph,
        &nodes,
        "yves",
        PlaysIn,
        "Star Wars: Rise of the Bechdel Test",
    );
    connect(&mut graph, &nodes, "fabian", PlaysIn, "Jurassic Park");
    connect(
        &mut graph,
        &nodes,
        "fabian",
        PlaysIn,
        "Star Wars Holiday Special",
    );
    connect(
        &mut graph,
        &nodes,
        "fabian",
        PlaysIn,
        "Star Wars: Rise of the Bechdel Test",
    );

    // Knows Relations
    connect(&mut graph, &nodes, "stefan", Knows, "stefan");
    connect(&mut graph, &nodes, "stefan", Knows, "yves");
    connect(&mut graph, &nodes, "yves", Knows, "yves");
    connect(&mut graph, &nodes, "yves", Knows, "fabian");
    connect(&mut graph, &nodes, "fabian", Knows, "fabian");
    connect(&mut graph, &nodes, "fabian", Knows, "benedikt");
    connect(&mut graph, &nodes, "benedikt", Knows, "benedikt");
    connect(&mut graph, &nodes, "benedikt", Knows, "tobias");
    connect(&mut graph, &nodes, "tobias", Knows, "tobias");
    connect(&mut graph, &nodes, "tobias", Knows, "fabian");

    // Successor Relations
    connect(
        &mut graph,
        &nodes,
        "Jurassic Park",
        Successor,
        "Star Wars Holiday Special",
    );
    connect(
        &mut graph,
        &nodes,
        "Star Wars Holiday Special",
        Successor,
        "Star Wars: Rise of the Bechdel Test",
    );

    (graph, nodes, edges)
}

///
/// Given an empty pattern graph and a non-empty base graph,
/// assert that we do get an empty result.
///
#[test]
fn test_empty_pattern_no_results() {
    let base_graph = node_graph().0;
    let empty_pattern = petgraph::graph::Graph::new();
    assert!(empty_pattern.is_empty_graph());

    // Explicitly specify result type.
    let results = VfState::eval(&empty_pattern, &base_graph);
    assert!(results.is_empty());
}

///
/// Given a pattern with a single node to match, assert we get
/// eleven graphs with one node each, and that we find the previous indices.
///
#[test]
fn test_single_node_any_pattern() {
    let base_graph = node_graph().0;
    let mut single_pattern = petgraph::graph::Graph::new();
    single_pattern.add_node_to_match(Box::new(|_n: &MovieNode| true));

    // Explicitly specify result type.
    let results = VfState::eval(&single_pattern, &base_graph);

    // Assert eleven results.
    assert_eq!(11, results.len());
    let mut found_indices = HashSet::new();

    // Check that node indices are from the pattern graph.
    for res in results {
        let indices = res.nodes().collect::<Vec<_>>();
        assert_eq!(1, indices.len());
        let index = *indices.get(0).unwrap();
        assert!(single_pattern.node_indices().any(|i| i == index));
        found_indices.insert(index);
    }
    assert_eq!(1, found_indices.len());
}

#[test]
///
/// Given a pattern with two, and a graph with one node, assert that no results can be found.
///
fn match_pattern_too_large() {
    let mut pattern_graph = petgraph::graph::Graph::new();
    pattern_graph.add_node_to_match(Box::new(|i: &i32| *i > 0));
    pattern_graph.add_node_to_match(Box::new(|i: &i32| *i > 0));

    let mut base_graph = petgraph::graph::Graph::new();
    let index = base_graph.add_node(4);
    base_graph.add_edge(index, index, "equals");

    let results = VfState::eval(&pattern_graph, &base_graph);
    assert_eq!(0, results.len());
}

///
/// Assert that when we query for a person pattern, we only get persons,
/// and that we get their different names.
///
#[test]
fn match_person_nodes_only() {
    let mut pattern_graph = petgraph::graph::Graph::new();
    let person_index =
        pattern_graph.add_node_to_match(Box::new(|mn| matches!(*mn, MovieNode::Person(_))));

    let base_graph = node_graph().0;
    let results = VfState::eval(&pattern_graph, &base_graph);

    let names: HashSet<_> = results
        .iter()
        .map(|r| r.node_weight(person_index))
        .map(|m| match m {
            MovieNode::Person(MoviePerson { name, type_of: _ }) => name,
            _ => "",
        })
        .filter(|s| !s.is_empty())
        .collect();

    assert!(names.contains("stefan"));
    assert!(names.contains("yves"));
    assert!(names.contains("fabian"));
    assert!(names.contains("tobias"));
    assert!(names.contains("benedikt"));
    assert_eq!(5, names.len());
}

fn is_movie(node: &MovieNode) -> bool {
    matches!(node, MovieNode::Movie(_))
}

fn is_person(node: &MovieNode) -> bool {
    matches!(node, MovieNode::Person(_))
}

///
/// Given a pattern that matches a movie node and person node, assert that
/// we find all 6*5 = 30 different node pairs.
///
#[test]
fn match_two_node_pairs() {
    let mut pattern_graph = petgraph::graph::Graph::new();
    pattern_graph.add_node_to_match(Box::new(is_movie));
    pattern_graph.add_node_to_match(Box::new(is_person));
    let base_graph = node_graph().0;

    let results = VfState::eval(&pattern_graph, &base_graph);

    assert_eq!(6 * 5, results.len());
}

///
/// Test that we do not produce graphs where one node can never be matched.
///
#[test]
fn match_wrong_matches_only() {
    let base_graph = node_graph().0;
    let mut two_pattern = petgraph::graph::Graph::new();
    two_pattern.add_node_to_match(Box::new(|n| matches!(n, MovieNode::Movie(_))));

    two_pattern.add_node_to_match(Box::new(|n| {
        matches!(n, MovieNode::Movie(common::Movie { year: 32, .. }))
    }));

    let results = VfState::eval(&two_pattern, &base_graph);
    assert_eq!(0, results.len());
}

///
/// Test in a directed acyclic graph we find all four edges
/// when we match (n1) --> (..) --> (n2).
///
/// Query its edges, and check its references.
///
#[test]
fn match_single_edges() {
    let mut base_graph = petgraph::graph::Graph::new();
    let idx_0 = base_graph.add_node(0);
    let idx_1 = base_graph.add_node(1);
    let idx_2 = base_graph.add_node(2);
    let idx_3 = base_graph.add_node(3);

    base_graph.add_edge(idx_0, idx_1, 0);
    base_graph.add_edge(idx_0, idx_2, 1);
    base_graph.add_edge(idx_1, idx_3, 2);
    base_graph.add_edge(idx_2, idx_3, 3);

    let mut pattern_graph = petgraph::graph::Graph::new();
    let idx_4 = pattern_graph.add_node_to_match(Box::new(|_| true));
    let idx_5 = pattern_graph.add_node_to_match(Box::new(|_| true));
    let edge = pattern_graph.add_edge_to_match(idx_4, idx_5, Box::new(|_| true));

    let results = VfState::eval(&pattern_graph, &base_graph);

    // Assert four results.
    assert_eq!(4, results.len());

    for matched_graph in results {
        let edges: Vec<_> = matched_graph.edges().collect();
        assert_eq!(1, edges.len());

        let (idx_a1, idx_a2) = matched_graph.adjacent_nodes(edge);
        assert_eq!(idx_4, idx_a1);
        assert_eq!(idx_5, idx_a2);
    }
}

///
/// Implement a two-length pattern on the graph from `match_single_edges`.
/// Assert two results, and nodes 0/3 to be at the start/end,
/// with its second node to be 1/2.
///
#[test]
fn match_double_edges() {
    let mut base_graph = petgraph::graph::Graph::new();
    let idx_0 = base_graph.add_node(0);
    let idx_1 = base_graph.add_node(1);
    let idx_2 = base_graph.add_node(2);
    let idx_3 = base_graph.add_node(3);

    base_graph.add_edge(idx_0, idx_1, 0);
    base_graph.add_edge(idx_0, idx_2, 1);
    base_graph.add_edge(idx_1, idx_3, 2);
    base_graph.add_edge(idx_2, idx_3, 3);

    let mut pattern_graph = petgraph::graph::Graph::new();
    let idx_4 = pattern_graph.add_node_to_match(Box::new(|_| true));
    let idx_5 = pattern_graph.add_node_to_match(Box::new(|_| true));
    let idx_6 = pattern_graph.add_node_to_match(Box::new(|_| true));
    pattern_graph.add_edge_to_match(idx_5, idx_6, Box::new(|_| true));
    pattern_graph.add_edge_to_match(idx_6, idx_4, Box::new(|_| true));

    let results = VfState::eval(&pattern_graph, &base_graph);

    assert_eq!(2, results.len());
    for res in results {
        assert_eq!(&&0, res.node_weight(idx_5));
        assert_eq!(&&3, res.node_weight(idx_4));
        let middle = *res.node_weight(idx_6);
        assert!(middle == &1 || middle == &2);
    }
}

///
/// Define a six-star as base graph and a three-star as pattern.
/// Find all 120 = 6!/3! possible results. Assert that the middle is 0,
/// and we have four nodes + three edges.
///
/// Check that all incoming edges have 0 as pointer.
///
#[test]
fn match_three_star_in_six_star() {
    let mut six_star = petgraph::graph::Graph::new();
    let idx_0 = six_star.add_node(0);
    let idx_1 = six_star.add_node(1);
    let idx_2 = six_star.add_node(2);
    let idx_3 = six_star.add_node(3);
    let idx_4 = six_star.add_node(4);
    let idx_5 = six_star.add_node(5);
    let idx_6 = six_star.add_node(6);

    six_star.add_edge(idx_0, idx_1, 1);
    six_star.add_edge(idx_0, idx_2, 2);
    six_star.add_edge(idx_0, idx_3, 3);
    six_star.add_edge(idx_0, idx_4, 4);
    six_star.add_edge(idx_0, idx_5, 5);
    six_star.add_edge(idx_0, idx_6, 6);

    let mut three_star = petgraph::graph::Graph::new();
    let idx_13 = three_star.add_node_to_match(Box::new(|_| true));
    let idx_12 = three_star.add_node_to_match(Box::new(|_| true));
    let idx_11 = three_star.add_node_to_match(Box::new(|_| true));
    let idx_10 = three_star.add_node_to_match(Box::new(|_| true));

    // e1: single connection of n3 via n0.
    let e1 = three_star.add_edge_to_match(idx_10, idx_11, Box::new(|_| true));
    let e2 = three_star.add_edge_to_match(idx_10, idx_12, Box::new(|_| true));
    let e3 = three_star.add_edge_to_match(idx_10, idx_13, Box::new(|_| true));

    let results = VfState::eval(&three_star, &six_star);
    assert_eq!(120, results.len());

    for res in results {
        assert_eq!(4, res.nodes().count());
        assert_eq!(3, res.edges().count());

        let center_weight = res.node_weight(idx_10);
        assert_eq!(&0, *center_weight);

        assert!(res.is_directed_edge(e1));
        // Check incoming edges.
        assert_eq!(e1, res.incoming_edges(idx_11).next().unwrap());
        assert_eq!(e2, res.incoming_edges(idx_12).next().unwrap());
        assert_eq!(e3, res.incoming_edges(idx_13).next().unwrap());
    }
}

///
/// Using the graph from match_three_star_in_six_star, test that
/// we may filter edges on whether they have an even weight attached to them.
///
#[test]
fn match_three_star_even_weights() {
    let mut six_star = petgraph::graph::Graph::new();
    let idx_0 = six_star.add_node(0);
    let idx_1 = six_star.add_node(1);
    let idx_2 = six_star.add_node(2);
    let idx_3 = six_star.add_node(3);
    let idx_4 = six_star.add_node(4);
    let idx_5 = six_star.add_node(5);
    let idx_6 = six_star.add_node(6);

    six_star.add_edge(idx_0, idx_1, 1);
    six_star.add_edge(idx_0, idx_2, 2);
    six_star.add_edge(idx_0, idx_3, 3);
    six_star.add_edge(idx_0, idx_4, 4);
    six_star.add_edge(idx_0, idx_5, 5);
    six_star.add_edge(idx_0, idx_6, 6);

    let mut three_star = petgraph::graph::Graph::new();
    let idx_13 = three_star.add_node_to_match(Box::new(|_| true));
    let idx_12 = three_star.add_node_to_match(Box::new(|_| true));
    let idx_11 = three_star.add_node_to_match(Box::new(|_| true));
    let idx_10 = three_star.add_node_to_match(Box::new(|_| true));

    // e1: single connection of n3 via n0.
    three_star.add_edge_to_match(idx_10, idx_11, Box::new(|x| x % 2 == 0));
    three_star.add_edge_to_match(idx_10, idx_12, Box::new(|x| x % 2 == 0));
    three_star.add_edge_to_match(idx_10, idx_13, Box::new(|x| x % 2 == 0));

    let results = VfState::eval(&three_star, &six_star);
    assert_eq!(6, results.len());

    for res in results {
        assert_eq!(3, res.edges().count());
        for edge_ref in res.edges() {
            assert!(*res.edge_weight(edge_ref) % 2 == 0);
        }
    }
}

///
/// Using the graph from match_three_star_in_six_star, test that
/// we find an star with edges leading _to_ the center, not from it.
///
#[test]
fn match_three_star_inverse() {
    let mut six_star = petgraph::graph::Graph::new();
    let idx_0 = six_star.add_node(0);
    let idx_1 = six_star.add_node(1);
    let idx_2 = six_star.add_node(2);
    let idx_3 = six_star.add_node(3);
    let idx_4 = six_star.add_node(4);
    let idx_5 = six_star.add_node(5);
    let idx_6 = six_star.add_node(6);

    six_star.add_edge(idx_2, idx_0, 2);
    six_star.add_edge(idx_1, idx_0, 1);
    six_star.add_edge(idx_3, idx_0, 3);
    six_star.add_edge(idx_4, idx_0, 4);
    six_star.add_edge(idx_5, idx_0, 5);
    six_star.add_edge(idx_6, idx_0, 6);

    let mut three_star = petgraph::graph::Graph::new();
    let idx_11 = three_star.add_node_to_match(Box::new(|_| true));
    let idx_13 = three_star.add_node_to_match(Box::new(|_| true));
    let idx_10 = three_star.add_node_to_match(Box::new(|_| true));
    let idx_12 = three_star.add_node_to_match(Box::new(|_| true));

    // e1: single connection of n3 via n0.
    three_star.add_edge_to_match(idx_13, idx_10, Box::new(|x| x % 2 == 0));
    three_star.add_edge_to_match(idx_12, idx_10, Box::new(|x| x % 2 == 0));
    three_star.add_edge_to_match(idx_11, idx_10, Box::new(|x| x % 2 == 0));

    let results = VfState::eval(&three_star, &six_star);
    assert_eq!(6, results.len());
}

///
/// Given the star graph match_three_star_in_six_star as base graph,
/// and a clique with 4 members + 12 edges, assert we do not find a result.
///
#[test]
fn test_node_edge_counts_terminate_early() {
    let mut six_star = petgraph::graph::Graph::new();
    let idx_0 = six_star.add_node(0);
    let idx_1 = six_star.add_node(1);
    let idx_2 = six_star.add_node(2);
    let idx_3 = six_star.add_node(3);
    let idx_4 = six_star.add_node(4);
    let idx_5 = six_star.add_node(5);
    let idx_6 = six_star.add_node(6);

    six_star.add_edge(idx_2, idx_0, 2);
    six_star.add_edge(idx_1, idx_0, 1);
    six_star.add_edge(idx_3, idx_0, 3);
    six_star.add_edge(idx_4, idx_0, 4);
    six_star.add_edge(idx_5, idx_0, 5);
    six_star.add_edge(idx_6, idx_0, 6);

    let mut four_clique = petgraph::graph::Graph::new();
    let idx_11 = four_clique.add_node_to_match(Box::new(|_| true));
    let idx_13 = four_clique.add_node_to_match(Box::new(|_| true));
    let idx_10 = four_clique.add_node_to_match(Box::new(|_| true));
    let idx_12 = four_clique.add_node_to_match(Box::new(|_| true));

    four_clique.add_edge_to_match(idx_10, idx_11, Box::new(|_| true));
    four_clique.add_edge_to_match(idx_10, idx_12, Box::new(|_| true));
    four_clique.add_edge_to_match(idx_10, idx_13, Box::new(|_| true));
    four_clique.add_edge_to_match(idx_11, idx_10, Box::new(|_| true));
    four_clique.add_edge_to_match(idx_11, idx_12, Box::new(|_| true));
    four_clique.add_edge_to_match(idx_11, idx_13, Box::new(|_| true));
    four_clique.add_edge_to_match(idx_12, idx_10, Box::new(|_| true));
    four_clique.add_edge_to_match(idx_12, idx_11, Box::new(|_| true));
    four_clique.add_edge_to_match(idx_12, idx_13, Box::new(|_| true));
    four_clique.add_edge_to_match(idx_13, idx_10, Box::new(|_| true));
    four_clique.add_edge_to_match(idx_13, idx_11, Box::new(|_| true));
    four_clique.add_edge_to_match(idx_13, idx_12, Box::new(|_| true));

    let results = VfState::eval(&four_clique, &six_star);
    assert_eq!(0, results.len());
}

///
/// Match a given graph on:
/// 1. Three different actors.
/// 2. Three different movies.
/// 3. Next to other relations, we have three persons who know each other,
/// and also themselves.
///
/// Assert 3 matches.
///
#[test]
fn optimization_test() {
    let data_graph = full_graph().0;

    // Pattern
    let mut pattern_graph = petgraph::graph::Graph::new();
    let p1 = pattern_graph.add_node_to_match(Box::new(|m| matches!(m, MovieNode::Person(_))));
    let p2 = pattern_graph.add_node_to_match(Box::new(|m| matches!(m, MovieNode::Person(_))));
    let p3 = pattern_graph.add_node_to_match(Box::new(|m| matches!(m, MovieNode::Person(_))));
    pattern_graph.add_edge_to_match(p1, p2, Box::new(|m| matches!(m, Knows)));
    pattern_graph.add_edge_to_match(p2, p3, Box::new(|m| matches!(m, Knows)));
    pattern_graph.add_edge_to_match(p3, p1, Box::new(|m| matches!(m, Knows)));
    pattern_graph.add_edge_to_match(p1, p1, Box::new(|m| matches!(m, Knows)));
    pattern_graph.add_edge_to_match(p2, p2, Box::new(|m| matches!(m, Knows)));
    pattern_graph.add_edge_to_match(p3, p3, Box::new(|m| matches!(m, Knows)));

    // Algorithm
    let results = VfState::eval(&pattern_graph, &data_graph);

    // Check results
    assert_eq!(3, results.len());
    // Each graph with 3 nodes + edges
    for matched_graph in results {
        assert_eq!(3, matched_graph.count_nodes());
        assert_eq!(6, matched_graph.count_edges());

        // Check Node types.
        for p in matched_graph.node_weights() {
            assert!(matches!(p, MovieNode::Person(_)));
        }

        // Check edge types.
        for edge in matched_graph.edge_weights() {
            assert!(matches!(edge, Knows));
        }
    }
}

///
/// Match a given graph on:
/// 1. Three different actors.
/// 2. Three different movies.
/// 3. Next to other relations, we have three persons who know each other.
///
/// Assert 3 matches.
///
#[test]
fn cycle_in_knows_match() {
    let data_graph = full_graph().0;

    // Pattern
    let mut pattern_graph = petgraph::graph::Graph::new();
    let p1 = pattern_graph.add_node_to_match(Box::new(|m| matches!(m, MovieNode::Person(_))));
    let p2 = pattern_graph.add_node_to_match(Box::new(|m| matches!(m, MovieNode::Person(_))));
    let p3 = pattern_graph.add_node_to_match(Box::new(|m| matches!(m, MovieNode::Person(_))));
    pattern_graph.add_edge_to_match(p1, p2, Box::new(|m| matches!(m, Knows)));
    pattern_graph.add_edge_to_match(p2, p3, Box::new(|m| matches!(m, Knows)));
    pattern_graph.add_edge_to_match(p3, p1, Box::new(|m| matches!(m, Knows)));

    // Algorithm
    let results = VfState::eval(&pattern_graph, &data_graph);

    // Check results
    assert_eq!(3, results.len());
    // Each graph with 3 nodes + edges
    for matched_graph in results {
        assert_eq!(3, matched_graph.count_nodes());
        assert_eq!(3, matched_graph.count_edges());

        // Check Node types.
        assert!(matches!(
            matched_graph.node_weight(p1),
            MovieNode::Person(_)
        ));
        assert!(matches!(
            matched_graph.node_weight(p2),
            MovieNode::Person(_)
        ));
        assert!(matches!(
            matched_graph.node_weight(p3),
            MovieNode::Person(_)
        ));

        // Check edge types.
        for edge in matched_graph.edge_weights() {
            assert!(matches!(edge, Knows));
        }
    }
}

///
/// Find a sequence of five persons, starting with stefan,
/// in the graph.
///
#[test]
fn sequence_optimization() {
    let data_graph = full_graph().0;

    let mut pattern_graph = petgraph::graph::Graph::new();
    // Five persons
    let p0 = pattern_graph.add_node_to_match(Box::new(|p| matches!(p, MovieNode::Person(_))));
    let p1 = pattern_graph.add_node_to_match(Box::new(|p| matches!(p, MovieNode::Person(_))));
    let p2 = pattern_graph.add_node_to_match(Box::new(|p| matches!(p, MovieNode::Person(_))));
    let p3 = pattern_graph.add_node_to_match(Box::new(|p| matches!(p, MovieNode::Person(_))));
    let p4 = pattern_graph.add_node_to_match(Box::new(|p| matches!(p, MovieNode::Person(_))));
    // Sequence: p0 -> p1 -> ... -> p4
    pattern_graph.add_edge_to_match(p0, p1, Box::new(|e| matches!(e, Knows)));
    pattern_graph.add_edge_to_match(p1, p2, Box::new(|e| matches!(e, Knows)));
    pattern_graph.add_edge_to_match(p2, p3, Box::new(|e| matches!(e, Knows)));
    pattern_graph.add_edge_to_match(p3, p4, Box::new(|e| matches!(e, Knows)));

    // Query
    let query_results = VfState::eval(&pattern_graph, &data_graph);
    assert_eq!(1, query_results.len());

    // Persons are stefan, yves, fabian, benedikt, tobias
    let res_graph = &query_results[0];
    assert_eq!(5, res_graph.count_nodes());
    assert_eq!(4, res_graph.count_edges());
    // Check asserts
    assert!(matches!(res_graph.node_weight(p0), MovieNode::Person(_)));
    assert!(matches!(res_graph.node_weight(p1), MovieNode::Person(_)));
    assert!(matches!(res_graph.node_weight(p2), MovieNode::Person(_)));
    assert!(matches!(res_graph.node_weight(p3), MovieNode::Person(_)));
    assert!(matches!(res_graph.node_weight(p4), MovieNode::Person(_)));
}
