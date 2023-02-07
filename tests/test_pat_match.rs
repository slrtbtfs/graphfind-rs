pub mod common;
use std::collections::{HashMap, HashSet};

use common::{
    ActorType::Actor,
    MovieNode, MoviePerson,
    MovieType::{Movie, Tv, Video},
    Relation,
    Relation::{Knows, PlaysIn, Successor},
};
use petgraph::graph::{Graph, NodeIndex};
use rustgql::{
    graph::Graph as QueryGraph,
    matcher,
    query::{PatternGraph, SubgraphAlgorithm},
    query_algorithms::{pattern_graphs::new_pattern, vf_algorithms::VfState},
};

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
    add_media(&mut graph, &mut names, "Jurassic Park", 10.0, 1990, Movie);
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
fn full_graph<'a>() -> (Graph<MovieNode, Relation>, HashMap<&'a str, NodeIndex>) {
    let (mut graph, nodes) = node_graph();
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
    connect(&mut graph, &nodes, "fabian", PlaysIn, "Sunday Uke Group");

    // Knows Relations
    connect(&mut graph, &nodes, "stefan", Knows, "stefan");
    connect(&mut graph, &nodes, "stefan", Knows, "yves");
    connect(&mut graph, &nodes, "yves", Knows, "yves");
    connect(&mut graph, &nodes, "yves", Knows, "stefan");
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
    connect(
        &mut graph,
        &nodes,
        "Sunday Uke Group",
        Successor,
        "Star Wars: Rise of the Bechdel Test",
    );

    (graph, nodes)
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
    let mut single_pattern = new_pattern();
    single_pattern.add_node_to_match(matcher!());

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
        assert!(single_pattern.nodes().any(|i| i == index));
        found_indices.insert(index);
    }
    assert_eq!(1, found_indices.len());
}

#[test]
///
/// Given a pattern with two, and a graph with one node, assert that no results can be found.
///
fn match_pattern_too_large() {
    let mut pattern_graph = new_pattern();
    let positive = matcher!(i if *i > 0);
    pattern_graph.add_node_to_match(positive);
    pattern_graph.add_node_to_match(positive);

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
    let mut pattern_graph = new_pattern();
    let person_index = pattern_graph.add_node_to_match(matcher!(MovieNode::Person(_)));

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

fn is_person(node: &MovieNode) -> bool {
    matches!(node, MovieNode::Person(_))
}

///
/// Given a pattern that matches a movie node and person node, assert that
/// we find all 6*5 = 30 different node pairs.
///
#[test]
fn match_two_node_pairs() {
    let mut pattern_graph = new_pattern();

    let is_movie = matcher!(MovieNode::Movie(_));
    pattern_graph.add_node_to_match(is_movie);
    pattern_graph.add_node_to_match(is_person);
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
    let mut two_pattern = new_pattern();
    two_pattern.add_node_to_match(matcher!(MovieNode::Movie(_)));

    two_pattern.add_node_to_match(matcher!(MovieNode::Movie(common::Movie { year: 32, .. })));

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

    let mut pattern_graph = new_pattern();
    let idx_4 = pattern_graph.add_node_to_match(matcher!());
    let idx_5 = pattern_graph.add_node_to_match(matcher!());
    let edge = pattern_graph.add_edge_to_match(idx_4, idx_5, matcher!());

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

    let mut pattern_graph = new_pattern();
    let idx_4 = pattern_graph.add_node_to_match(matcher!());
    let idx_5 = pattern_graph.add_node_to_match(matcher!());
    let idx_6 = pattern_graph.add_node_to_match(matcher!());
    pattern_graph.add_edge_to_match(idx_5, idx_6, matcher!());
    pattern_graph.add_edge_to_match(idx_6, idx_4, matcher!());

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
    let center = six_star.add_node(0);

    for i in 1..=6 {
        let leaf = six_star.add_node(i);
        six_star.add_edge(center, leaf, i);
    }

    let mut e = vec![];
    let mut v = vec![];
    let mut three_star = new_pattern();
    let center = three_star.add_node_to_match(matcher!());
    for _ in 1..=3 {
        let leaf = three_star.add_node_to_match(matcher!());
        v.push(leaf);
        e.push(three_star.add_edge_to_match(center, leaf, matcher!()));
    }

    let results = VfState::eval(&three_star, &six_star);
    assert_eq!(120, results.len());

    for res in results {
        assert_eq!(4, res.nodes().count());
        assert_eq!(3, res.edges().count());

        let center_weight = res.node_weight(center);
        assert_eq!(&0, *center_weight);

        // Check incoming edges.i
        for i in 0..3 {
            assert!(res.is_directed_edge(e[i]));
            assert_eq!(e[i], res.incoming_edges(v[i]).next().unwrap());
        }
    }
}

///
/// Using the graph from match_three_star_in_six_star, test that
/// we may filter edges on whether they have an even weight attached to them.
///
#[test]
fn match_three_star_even_weights() {
    let mut six_star = petgraph::graph::Graph::new();
    let center = six_star.add_node(0);

    for i in 1..=6 {
        let leaf = six_star.add_node(i);
        six_star.add_edge(center, leaf, i);
    }

    let mut three_star = new_pattern();
    let center = three_star.add_node_to_match(matcher!());
    for _ in 1..=3 {
        let leaf = three_star.add_node_to_match(matcher!());
        three_star.add_edge_to_match(center, leaf, matcher!(i if i%2 == 0));
    }

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
    let center = six_star.add_node(0);

    for i in 1..=6 {
        let leaf = six_star.add_node(i);
        six_star.add_edge(leaf, center, i);
    }

    let mut three_star = new_pattern();
    let center = three_star.add_node_to_match(matcher!());
    for _ in 1..=3 {
        let leaf = three_star.add_node_to_match(matcher!());
        three_star.add_edge_to_match(leaf, center, matcher!(i if i%2 == 0));
    }

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
    let center = six_star.add_node(0);

    for i in 1..=6 {
        let leaf = six_star.add_node(i);
        six_star.add_edge(center, leaf, i);
    }
    let mut four_clique = new_pattern();
    let idx_11 = four_clique.add_node_to_match(matcher!());
    let idx_13 = four_clique.add_node_to_match(matcher!());
    let idx_10 = four_clique.add_node_to_match(matcher!());
    let idx_12 = four_clique.add_node_to_match(matcher!());

    four_clique.add_edge_to_match(idx_10, idx_11, matcher!());
    four_clique.add_edge_to_match(idx_10, idx_12, matcher!());
    four_clique.add_edge_to_match(idx_10, idx_13, matcher!());
    four_clique.add_edge_to_match(idx_11, idx_10, matcher!());
    four_clique.add_edge_to_match(idx_11, idx_12, matcher!());
    four_clique.add_edge_to_match(idx_11, idx_13, matcher!());
    four_clique.add_edge_to_match(idx_12, idx_10, matcher!());
    four_clique.add_edge_to_match(idx_12, idx_11, matcher!());
    four_clique.add_edge_to_match(idx_12, idx_13, matcher!());
    four_clique.add_edge_to_match(idx_13, idx_10, matcher!());
    four_clique.add_edge_to_match(idx_13, idx_11, matcher!());
    four_clique.add_edge_to_match(idx_13, idx_12, matcher!());

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
    let mut pattern_graph = new_pattern();
    let p1 = pattern_graph.add_node_to_match(is_person);
    let p2 = pattern_graph.add_node_to_match(is_person);
    let p3 = pattern_graph.add_node_to_match(is_person);
    pattern_graph.add_edge_to_match(p1, p2, matcher!(Knows));
    pattern_graph.add_edge_to_match(p2, p3, matcher!(Knows));
    pattern_graph.add_edge_to_match(p3, p1, matcher!(Knows));
    pattern_graph.add_edge_to_match(p1, p1, matcher!(Knows));
    pattern_graph.add_edge_to_match(p2, p2, matcher!(Knows));
    pattern_graph.add_edge_to_match(p3, p3, matcher!(Knows));

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
    let mut pattern_graph = new_pattern();
    let p1 = pattern_graph.add_node_to_match(is_person);
    let p2 = pattern_graph.add_node_to_match(is_person);
    let p3 = pattern_graph.add_node_to_match(is_person);
    pattern_graph.add_edge_to_match(p1, p2, |m| matches!(m, Knows));
    pattern_graph.add_edge_to_match(p2, p3, |m| matches!(m, Knows));
    pattern_graph.add_edge_to_match(p3, p1, |m| matches!(m, Knows));

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

    let mut pattern_graph = new_pattern();
    // Five persons
    let p0 = pattern_graph.add_node_to_match(is_person);
    let p1 = pattern_graph.add_node_to_match(is_person);
    let p2 = pattern_graph.add_node_to_match(is_person);
    let p3 = pattern_graph.add_node_to_match(is_person);
    let p4 = pattern_graph.add_node_to_match(is_person);
    // Sequence: p0 -> p1 -> ... -> p4
    let e0 = pattern_graph.add_edge_to_match(p0, p1, matcher!(Knows));
    let e1 = pattern_graph.add_edge_to_match(p1, p2, matcher!(Knows));
    let e2 = pattern_graph.add_edge_to_match(p2, p3, matcher!(Knows));
    let e3 = pattern_graph.add_edge_to_match(p3, p4, matcher!(Knows));

    // Query
    let query_results = VfState::eval(&pattern_graph, &data_graph);
    assert_eq!(1, query_results.len());

    // Persons are stefan, yves, fabian, benedikt, tobias
    let res_graph = &query_results[0];
    assert_eq!(5, res_graph.count_nodes());
    assert_eq!(4, res_graph.count_edges());
    // Check asserts
    assert!(check_for_actor(res_graph.node_weight(p0), "stefan"));
    assert!(matches!(res_graph.node_weight(p1), MovieNode::Person(_)));
    assert!(matches!(res_graph.node_weight(p2), MovieNode::Person(_)));
    assert!(matches!(res_graph.node_weight(p3), MovieNode::Person(_)));
    assert!(matches!(res_graph.node_weight(p4), MovieNode::Person(_)));

    assert!(res_graph.adjacent_nodes(e0) == (p0, p1));
    assert!(res_graph.adjacent_nodes(e1) == (p1, p2));
    assert!(res_graph.adjacent_nodes(e2) == (p2, p3));
    assert!(res_graph.adjacent_nodes(e3) == (p3, p4));
}

///
/// Find a sequence of four persons, starting with stefan or yves
/// in the graph. The first and last person know themselves.
///
#[test]
fn reflexive_optimization2() {
    let data_graph = full_graph().0;

    let mut pattern_graph = new_pattern();
    // Five persons
    let p0 = pattern_graph.add_node_to_match(is_person);
    let p1 = pattern_graph.add_node_to_match(is_person);
    let p2 = pattern_graph.add_node_to_match(is_person);
    let p3 = pattern_graph.add_node_to_match(is_person);
    // Sequence: p0 -> p1 -> ... -> p3
    let e0 = pattern_graph.add_edge_to_match(p0, p1, matcher!(Knows));
    let e1 = pattern_graph.add_edge_to_match(p1, p2, matcher!(Knows));
    let e2 = pattern_graph.add_edge_to_match(p2, p3, matcher!(Knows));
    let e3 = pattern_graph.add_edge_to_match(p0, p0, matcher!(Knows));
    let e4 = pattern_graph.add_edge_to_match(p3, p3, matcher!(Knows));

    // Query
    let query_results = VfState::eval(&pattern_graph, &data_graph);
    assert_eq!(2, query_results.len());

    // Persons are stefan, yves, fabian, benedikt, tobias
    for res_graph in query_results {
        assert_eq!(4, res_graph.count_nodes());
        assert_eq!(5, res_graph.count_edges());
        // Check asserts
        assert!(
            check_for_actor(res_graph.node_weight(p0), "stefan")
                || check_for_actor(res_graph.node_weight(p0), "yves")
        );
        assert!(matches!(res_graph.node_weight(p1), MovieNode::Person(_)));
        assert!(matches!(res_graph.node_weight(p2), MovieNode::Person(_)));
        assert!(matches!(res_graph.node_weight(p3), MovieNode::Person(_)));

        // Edges
        assert!(res_graph.adjacent_nodes(e0) == (p0, p1));
        assert!(res_graph.adjacent_nodes(e1) == (p1, p2));
        assert!(res_graph.adjacent_nodes(e2) == (p2, p3));
        // Find Self Loops
        assert!(res_graph.adjacent_nodes(e3) == (p0, p0));
        assert!(matches!(res_graph.edge_weight(e3), Knows));
        assert!(res_graph.adjacent_nodes(e4) == (p3, p3));
        assert!(matches!(res_graph.edge_weight(e4), Knows));
    }
}

///
/// Find a sequence of four persons, starting with stefan or yves
/// in the graph. All persons know themselves.
///
#[test]
fn reflexive_optimization() {
    let data_graph = full_graph().0;

    let mut pattern_graph = new_pattern();
    // Five persons
    let p0 = pattern_graph.add_node_to_match(is_person);
    let p1 = pattern_graph.add_node_to_match(is_person);
    let p2 = pattern_graph.add_node_to_match(is_person);
    let p3 = pattern_graph.add_node_to_match(is_person);
    // Sequence: p0 -> p1 -> ... -> p3
    let e0 = pattern_graph.add_edge_to_match(p0, p1, matcher!(Knows));
    let e1 = pattern_graph.add_edge_to_match(p1, p2, matcher!(Knows));
    let e2 = pattern_graph.add_edge_to_match(p2, p3, matcher!(Knows));
    // Self References
    let e3 = pattern_graph.add_edge_to_match(p0, p0, matcher!(Knows));
    let e4 = pattern_graph.add_edge_to_match(p3, p3, matcher!(Knows));
    let e5 = pattern_graph.add_edge_to_match(p1, p1, matcher!(Knows));
    let e6 = pattern_graph.add_edge_to_match(p2, p2, matcher!(Knows));

    // Query
    let query_results = VfState::eval(&pattern_graph, &data_graph);
    assert_eq!(2, query_results.len());

    // Persons are stefan, yves, fabian, benedikt, tobias
    for res_graph in query_results {
        assert_eq!(4, res_graph.count_nodes());
        assert_eq!(7, res_graph.count_edges());
        // Check asserts
        assert!(
            check_for_actor(res_graph.node_weight(p0), "stefan")
                || check_for_actor(res_graph.node_weight(p0), "yves")
        );
        assert!(matches!(res_graph.node_weight(p1), MovieNode::Person(_)));
        assert!(matches!(res_graph.node_weight(p2), MovieNode::Person(_)));
        assert!(matches!(res_graph.node_weight(p3), MovieNode::Person(_)));

        // Edges
        assert!(res_graph.adjacent_nodes(e0) == (p0, p1));
        assert!(res_graph.adjacent_nodes(e1) == (p1, p2));
        assert!(res_graph.adjacent_nodes(e2) == (p2, p3));
        // Find Self Loops
        assert!(res_graph.adjacent_nodes(e3) == (p0, p0));
        assert!(matches!(res_graph.edge_weight(e3), Knows));
        assert!(res_graph.adjacent_nodes(e4) == (p3, p3));
        assert!(matches!(res_graph.edge_weight(e4), Knows));
        assert!(res_graph.adjacent_nodes(e5) == (p1, p1));
        assert!(matches!(res_graph.edge_weight(e5), Knows));
        assert!(res_graph.adjacent_nodes(e6) == (p2, p2));
        assert!(matches!(res_graph.edge_weight(e6), Knows));
    }
}

fn check_for_actor(x: &MovieNode, given_name: &str) -> bool {
    match x {
        MovieNode::Person(y) => {
            let name = &y.name;
            name.eq(given_name)
        }
        _ => false,
    }
}

///
/// Find a star graph with an actor as center, and
/// three movies he plays in.
///
#[test]
fn delete_test() {
    let base_graph = full_graph().0;
    let mut pattern_graph = new_pattern();

    // Have Fabian
    let fab = pattern_graph.add_node_to_match(|p| check_for_actor(p, "fabian"));
    let m1 = pattern_graph.add_node_to_match(matcher!(MovieNode::Movie(_)));
    let m2 = pattern_graph.add_node_to_match(matcher!(MovieNode::Movie(_)));
    let m3 = pattern_graph.add_node_to_match(matcher!(MovieNode::Movie(_)));

    // Fabian plays in three movies
    pattern_graph.add_edge_to_match(fab, m1, matcher!(PlaysIn));
    pattern_graph.add_edge_to_match(fab, m2, matcher!(PlaysIn));
    pattern_graph.add_edge_to_match(fab, m3, matcher!(PlaysIn));

    // Get & check results
    let results = VfState::eval(&pattern_graph, &base_graph);
    assert_eq!(24, results.len());

    // Check nodes & edges
    for g in results {
        assert_eq!(4, g.count_nodes());
        assert!(check_for_actor(g.node_weight(fab), "fabian"));
        assert!(matches!(g.node_weight(m1), MovieNode::Movie(_)));
        assert!(matches!(g.node_weight(m2), MovieNode::Movie(_)));
        assert!(matches!(g.node_weight(m3), MovieNode::Movie(_)));

        assert_eq!(3, g.count_edges());
        for weight in g.edge_weights() {
            assert!(matches!(weight, PlaysIn));
        }
    }
}

///
/// Three actors (stefan, yves, fabian) all
/// play in three different movies.
///
#[test]
fn three_to_three() {
    let mut pattern_graph = new_pattern();
    // Actors
    let s = pattern_graph.add_node_to_match(|p| check_for_actor(p, "stefan"));
    let y = pattern_graph.add_node_to_match(|p| check_for_actor(p, "yves"));
    let f = pattern_graph.add_node_to_match(|p| check_for_actor(p, "fabian"));
    // Movies
    let m1 = pattern_graph.add_node_to_match(matcher!(MovieNode::Movie(_)));
    let m2 = pattern_graph.add_node_to_match(matcher!(MovieNode::Movie(_)));
    let m3 = pattern_graph.add_node_to_match(matcher!(MovieNode::Movie(_)));
    // Lots of connections
    pattern_graph.add_edge_to_match(s, m1, matcher!(PlaysIn));
    pattern_graph.add_edge_to_match(s, m2, matcher!(PlaysIn));
    pattern_graph.add_edge_to_match(s, m3, matcher!(PlaysIn));
    pattern_graph.add_edge_to_match(y, m1, matcher!(PlaysIn));
    pattern_graph.add_edge_to_match(y, m2, matcher!(PlaysIn));
    pattern_graph.add_edge_to_match(y, m3, matcher!(PlaysIn));
    pattern_graph.add_edge_to_match(f, m1, matcher!(PlaysIn));
    pattern_graph.add_edge_to_match(f, m2, matcher!(PlaysIn));
    pattern_graph.add_edge_to_match(f, m3, matcher!(PlaysIn));

    // Query
    let base_graph = full_graph().0;
    let results = VfState::eval(&pattern_graph, &base_graph);
    // Six results
    assert_eq!(6, results.len());

    // Checks
    for graph in results {
        // Find our actors
        assert_eq!(6, graph.count_nodes());
        assert!(check_for_actor(graph.node_weight(s), "stefan"));
        assert!(check_for_actor(graph.node_weight(y), "yves"));
        assert!(check_for_actor(graph.node_weight(f), "fabian"));
        // and our movies
        assert!(matches!(graph.node_weight(m1), MovieNode::Movie(_)));
        assert!(matches!(graph.node_weight(m2), MovieNode::Movie(_)));
        assert!(matches!(graph.node_weight(m3), MovieNode::Movie(_)));
        // and our playsIn relations
        assert_eq!(9, graph.count_edges());
        assert!(graph.edge_weights().all(matcher!(PlaysIn)));
    }
}

// /
/// Three actors (stefan, yves, fabian) all
/// play in two different movies.
///
#[test]
fn three_to_two() {
    let mut pattern_graph = new_pattern();
    // Actors
    let s = pattern_graph.add_node_to_match(|p| check_for_actor(p, "stefan"));
    let y = pattern_graph.add_node_to_match(|p| check_for_actor(p, "yves"));
    let f = pattern_graph.add_node_to_match(|p| check_for_actor(p, "fabian"));
    // Movies
    let m1 = pattern_graph.add_node_to_match(matcher!(MovieNode::Movie(_)));
    let m2 = pattern_graph.add_node_to_match(matcher!(MovieNode::Movie(_)));
    // Lots of connections
    pattern_graph.add_edge_to_match(s, m1, matcher!(PlaysIn));
    pattern_graph.add_edge_to_match(s, m2, matcher!(PlaysIn));
    pattern_graph.add_edge_to_match(y, m1, matcher!(PlaysIn));
    pattern_graph.add_edge_to_match(y, m2, matcher!(PlaysIn));
    pattern_graph.add_edge_to_match(f, m1, matcher!(PlaysIn));
    pattern_graph.add_edge_to_match(f, m2, matcher!(PlaysIn));

    // Query
    let base_graph = full_graph().0;
    let results = VfState::eval(&pattern_graph, &base_graph);
    // Six results
    assert_eq!(6, results.len());

    // Checks
    for graph in results {
        // Find our actors
        assert_eq!(5, graph.count_nodes());
        assert!(check_for_actor(graph.node_weight(s), "stefan"));
        assert!(check_for_actor(graph.node_weight(y), "yves"));
        assert!(check_for_actor(graph.node_weight(f), "fabian"));
        // and our movies
        assert!(matches!(graph.node_weight(m1), MovieNode::Movie(_)));
        assert!(matches!(graph.node_weight(m2), MovieNode::Movie(_)));
        // and our playsIn relations
        assert_eq!(6, graph.count_edges());
        assert!(graph.edge_weights().all(matcher!(PlaysIn)));
    }
}

///
/// Two actors (stefan, yves) all
/// play in two different movies.
///
#[test]
fn two_to_two() {
    let mut pattern_graph = new_pattern();
    // Actors
    let s = pattern_graph.add_node_to_match(|p| check_for_actor(p, "stefan"));
    let y = pattern_graph.add_node_to_match(|p| check_for_actor(p, "yves"));
    // Movies
    let m1 = pattern_graph.add_node_to_match(matcher!(MovieNode::Movie(_)));
    let m2 = pattern_graph.add_node_to_match(matcher!(MovieNode::Movie(_)));
    // Lots of connections
    pattern_graph.add_edge_to_match(s, m1, matcher!(PlaysIn));
    pattern_graph.add_edge_to_match(s, m2, matcher!(PlaysIn));
    pattern_graph.add_edge_to_match(y, m1, matcher!(PlaysIn));
    pattern_graph.add_edge_to_match(y, m2, matcher!(PlaysIn));

    // Query
    let base_graph = full_graph().0;
    let results = VfState::eval(&pattern_graph, &base_graph);
    // Six results
    assert_eq!(6, results.len());

    // Checks
    for graph in results {
        // Find our actors
        assert_eq!(4, graph.count_nodes());
        assert!(check_for_actor(graph.node_weight(s), "stefan"));
        assert!(check_for_actor(graph.node_weight(y), "yves"));
        // and our movies
        assert!(matches!(graph.node_weight(m1), MovieNode::Movie(_)));
        assert!(matches!(graph.node_weight(m2), MovieNode::Movie(_)));
        // and our playsIn relations
        assert_eq!(4, graph.count_edges());
        assert!(graph.edge_weights().all(matcher!(PlaysIn)));
    }
}

///
/// Stefan plays in the Star Wars Movies.
///
#[test]
fn attributes_with_parameters() {
    let mut pattern_graph = new_pattern();
    // Actors
    let s = pattern_graph.add_node_to_match(|p| check_for_actor(p, "stefan"));
    // Movies
    let m1 = pattern_graph.add_node_to_match(|m| check_movie(m, "Star Wars Holiday Special", 1978));
    let m2 = pattern_graph
        .add_node_to_match(|m| check_movie(m, "Star Wars: Rise of the Bechdel Test", 2015));
    // Lots of connections
    pattern_graph.add_edge_to_match(s, m1, matcher!(PlaysIn));
    pattern_graph.add_edge_to_match(s, m2, matcher!(PlaysIn));

    // Query
    let base_graph = full_graph().0;
    let results = VfState::eval(&pattern_graph, &base_graph);
    // One result
    assert_eq!(1, results.len());

    // Checks
    for graph in results {
        // Find our actors
        assert_eq!(3, graph.count_nodes());
        assert!(check_for_actor(graph.node_weight(s), "stefan"));
        // and our movies
        assert!(matches!(graph.node_weight(m1), MovieNode::Movie(_)));
        assert!(matches!(graph.node_weight(m2), MovieNode::Movie(_)));
        // and our playsIn relations
        assert_eq!(2, graph.count_edges());
        assert!(graph.edge_weights().all(matcher!(PlaysIn)));
    }
}

fn check_movie(element: &MovieNode, title1: &str, year1: i32) -> bool {
    match element {
        MovieNode::Movie(common::Movie { title, year, .. }) => title.eq(title1) && year == &year1,
        _ => false,
    }
}

///
/// Find all Actor-Movie pairs (10 overall)
///
#[test]
fn create() {
    let mut pattern_graph = new_pattern();
    let p = pattern_graph.add_node_to_match(matcher!(MovieNode::Person(_)));
    let m = pattern_graph.add_node_to_match(matcher!(MovieNode::Movie(_)));
    let pi = pattern_graph.add_edge_to_match(p, m, matcher!(PlaysIn));

    let base_graph = full_graph().0;
    let query_results = VfState::eval(&pattern_graph, &base_graph);
    assert_eq!(10, query_results.len());

    // Check structure
    for graph in query_results {
        assert_eq!(1, graph.count_edges());
        assert_eq!(2, graph.count_nodes());
        assert_eq!((p, m), graph.adjacent_nodes(pi));
        assert!(matches!(graph.node_weight(p), MovieNode::Person(_)));
        assert!(matches!(graph.node_weight(m), MovieNode::Movie(_)));
        assert!(matches!(graph.edge_weight(pi), PlaysIn));
    }
}

///
/// Negative Test: Do not allow an edge to be visible when it refers to a
/// node that is ignored (source)
///
#[test]
#[should_panic]
fn ignore_wrong_edge_source() {
    let mut pattern = petgraph::graph::Graph::new();
    let from = pattern.add_node_to_match_full(|_: &i32| true, true);
    let to = pattern.add_node_to_match(|_: &i32| true);
    pattern.add_edge_to_match(from, to, |_: &i32| false);
}

///
/// Negative Test: Do not allow an edge to be visible when it refers to a
/// node that is ignored (dest)
///
#[test]
#[should_panic]
fn ignore_wrong_edge_dest() {
    let mut pattern = petgraph::graph::Graph::new();
    let from = pattern.add_node_to_match(|_: &i32| true);
    let to = pattern.add_node_to_match_full(|_: &i32| true, true);
    pattern.add_edge_to_match(from, to, |_: &i32| false);
}

///
/// stefan plays in two movies. But we only want to have one movie in our result!
///
#[test]
fn require_delete() {
    let mut pattern = petgraph::graph::Graph::new();
    let p = pattern.add_node_to_match(|x| check_for_actor(x, "stefan"));
    let m1 = pattern.add_node_to_match(matcher!(MovieNode::Movie(_)));
    let m2 = pattern.add_node_to_match_full(matcher!(MovieNode::Movie(_)), true);
    pattern.add_edge_to_match_full(p, m2, matcher!(PlaysIn), true);
    let e = pattern.add_edge_to_match(p, m1, matcher!(PlaysIn));

    // Run query
    let base_graph = full_graph().0;
    let results = VfState::eval(&pattern, &base_graph);
    assert_eq!(3, results.len());

    // Check results
    for res in results {
        assert_eq!(2, res.count_nodes());
        assert_eq!(1, res.count_edges());

        assert!(check_for_actor(res.node_weight(p), "stefan"));
        assert!(matches!(res.node_weight(m1), MovieNode::Movie(_)));
        assert!(matches!(res.edge_weight(e), PlaysIn));
    }
}

///
/// Do not allow access for ignored nodes. These do not appear in the result.
///
#[test]
#[should_panic]
fn check_ignored_nodes() {
    let mut pattern = petgraph::graph::Graph::new();
    let p = pattern.add_node_to_match(|x| check_for_actor(x, "stefan"));
    let m2 = pattern.add_node_to_match_full(matcher!(MovieNode::Movie(_)), true);
    pattern.add_edge_to_match_full(p, m2, matcher!(PlaysIn), true);

    // Run query
    let base_graph = full_graph().0;
    let results = VfState::eval(&pattern, &base_graph);
    let graph = &results[0];

    // Force panic
    assert!(!graph.nodes().any(|x| x == m2));
    graph.node_weight(m2);
}

///
/// Do not allow access for ignored edges. These do not appear in the result.
///
#[test]
#[should_panic]
fn check_ignored_edges() {
    let mut pattern = petgraph::graph::Graph::new();
    let p = pattern.add_node_to_match(|x| check_for_actor(x, "stefan"));
    let m2 = pattern.add_node_to_match_full(matcher!(MovieNode::Movie(_)), true);
    let e = pattern.add_edge_to_match_full(p, m2, matcher!(PlaysIn), true);

    // Run query
    let base_graph = full_graph().0;
    let results = VfState::eval(&pattern, &base_graph);
    let graph = &results[0];

    // Force panic
    assert!(!graph.edges().any(|x| x == e));
    graph.edge_weight(e);
}

///
/// Two actors (stefan, yves) play in at least two movies.
///
#[test]
fn require() {
    let mut pattern_graph = petgraph::graph::Graph::new();
    // Actors
    let s = pattern_graph.add_node_to_match(|p| check_for_actor(p, "stefan"));
    let y = pattern_graph.add_node_to_match(|p| check_for_actor(p, "yves"));
    // Movies
    let m1 = pattern_graph.add_node_to_match_full(matcher!(MovieNode::Movie(_)), true);
    let m2 = pattern_graph.add_node_to_match_full(matcher!(MovieNode::Movie(_)), true);
    // Four Connections we all ignore
    pattern_graph.add_edge_to_match_full(s, m1, matcher!(PlaysIn), true);
    pattern_graph.add_edge_to_match_full(s, m2, matcher!(PlaysIn), true);
    pattern_graph.add_edge_to_match_full(y, m1, matcher!(PlaysIn), true);
    pattern_graph.add_edge_to_match_full(y, m2, matcher!(PlaysIn), true);

    // Query
    let base_graph = full_graph().0;
    let results = VfState::eval(&pattern_graph, &base_graph);
    // Single Result
    assert_eq!(1, results.len());

    // Checks
    for graph in results {
        // Find our actors
        assert_eq!(2, graph.count_nodes());
        assert!(check_for_actor(graph.node_weight(s), "stefan"));
        assert!(check_for_actor(graph.node_weight(y), "yves"));
    }
}

///
/// Find two persons and movies, so that both persons play in the second movie,
/// and one movie is the successor of another.
///
#[test]
fn all_stereotypes() {
    let mut pattern_graph = petgraph::graph::Graph::new();
    // Nodes
    let p1 = pattern_graph.add_node_to_match(|x| check_for_actor(x, "fabian"));
    let p2 = pattern_graph.add_node_to_match_full(matcher!(MovieNode::Person(_)), true);
    let m1 = pattern_graph.add_node_to_match(matcher!(MovieNode::Movie(_)));
    let m2 = pattern_graph.add_node_to_match(matcher!(MovieNode::Movie(_)));
    // Edges
    let pm1 = pattern_graph.add_edge_to_match(p1, m1, matcher!(PlaysIn));
    let pm2 = pattern_graph.add_edge_to_match(p1, m2, matcher!(PlaysIn));
    pattern_graph.add_edge_to_match_full(p2, m2, matcher!(PlaysIn), true);
    let su = pattern_graph.add_edge_to_match(m1, m2, matcher!(Successor));

    // Query
    let base_graph = full_graph().0;
    let results = VfState::eval(&pattern_graph, &base_graph);
    assert_eq!(3, results.len());

    // Tests
    for res in results {
        assert_eq!(3, res.count_edges());
        assert_eq!(3, res.count_nodes());

        assert!(check_for_actor(res.node_weight(p1), "fabian"));
        assert!(matches!(res.node_weight(m1), MovieNode::Movie(_)));
        assert!(matches!(res.node_weight(m2), MovieNode::Movie(_)));

        assert!(matches!(res.edge_weight(pm1), PlaysIn));
        assert!(matches!(res.edge_weight(pm2), PlaysIn));
        assert!(matches!(res.edge_weight(su), Successor));
    }
}

///
/// Two actors (stefan, yves) all
/// play in two different movies (we ignore the movies).
///
#[test]
fn bk_two_to_two() {
    let mut pattern_graph = petgraph::graph::Graph::new();
    // Actors
    let s = pattern_graph.add_node_to_match(|p| check_for_actor(p, "stefan"));
    let y = pattern_graph.add_node_to_match(|p| check_for_actor(p, "yves"));
    // Movies
    let m1 = pattern_graph.add_node_to_match_full(matcher!(MovieNode::Movie(_)), true);
    let m2 = pattern_graph.add_node_to_match_full(matcher!(MovieNode::Movie(_)), true);
    // Connections
    pattern_graph.add_edge_to_match_full(s, m1, matcher!(PlaysIn), true);
    pattern_graph.add_edge_to_match_full(s, m2, matcher!(PlaysIn), true);
    pattern_graph.add_edge_to_match_full(y, m1, matcher!(PlaysIn), true);
    pattern_graph.add_edge_to_match_full(y, m2, matcher!(PlaysIn), true);

    // Query
    let base_graph = full_graph().0;
    let results = VfState::eval(&pattern_graph, &base_graph);
    // One result
    assert_eq!(1, results.len());

    // Checks
    for graph in results {
        // Find our actors
        assert_eq!(2, graph.count_nodes());
        assert!(check_for_actor(graph.node_weight(s), "stefan"));
        assert!(check_for_actor(graph.node_weight(y), "yves"));
        // No edges
        assert_eq!(0, graph.edges().collect::<Vec<_>>().len())
    }
}

///
/// Two actors (stefan, yves) all
/// play in three different movies (we ignore the movies).
///
#[test]
fn bk_two_to_three() {
    let mut pattern_graph = petgraph::graph::Graph::new();
    // Actors
    let s = pattern_graph.add_node_to_match(|p| check_for_actor(p, "stefan"));
    let y = pattern_graph.add_node_to_match(|p| check_for_actor(p, "yves"));
    // Movies
    let m1 = pattern_graph.add_node_to_match_full(matcher!(MovieNode::Movie(_)), true);
    let m2 = pattern_graph.add_node_to_match_full(matcher!(MovieNode::Movie(_)), true);
    let m3 = pattern_graph.add_node_to_match_full(matcher!(MovieNode::Movie(_)), true);
    // Connections
    pattern_graph.add_edge_to_match_full(s, m1, matcher!(PlaysIn), true);
    pattern_graph.add_edge_to_match_full(s, m2, matcher!(PlaysIn), true);
    pattern_graph.add_edge_to_match_full(s, m3, matcher!(PlaysIn), true);
    pattern_graph.add_edge_to_match_full(y, m1, matcher!(PlaysIn), true);
    pattern_graph.add_edge_to_match_full(y, m2, matcher!(PlaysIn), true);
    pattern_graph.add_edge_to_match_full(y, m3, matcher!(PlaysIn), true);

    // Query
    let base_graph = full_graph().0;
    let results = VfState::eval(&pattern_graph, &base_graph);
    // One result
    assert_eq!(1, results.len());

    // Checks
    for graph in results {
        // Find our actors
        assert_eq!(2, graph.count_nodes());
        assert!(check_for_actor(graph.node_weight(s), "stefan"));
        assert!(check_for_actor(graph.node_weight(y), "yves"));
        // No edges
        assert_eq!(0, graph.edges().collect::<Vec<_>>().len())
    }
}

///
/// Assert that yves knows stefan and plays in Jurassic Park.
/// Does not hold in this graph; thus return empty result!
///
#[test]
fn create_with_attributes() {
    let mut pattern_graph = petgraph::graph::Graph::new();
    let s = pattern_graph.add_node_to_match_full(|x| check_for_actor(x, "stefan"), true);
    let y = pattern_graph.add_node_to_match(|x| check_for_actor(x, "yves"));
    let j = pattern_graph.add_node_to_match_full(|x| check_movie(x, "Jurassic Park", 1990), true);
    pattern_graph.add_edge_to_match_full(y, s, matcher!(Knows), true);
    pattern_graph.add_edge_to_match_full(y, j, matcher!(Knows), true);

    // Run query
    let base_graph = full_graph().0;
    assert_eq!(0, VfState::eval(&pattern_graph, &base_graph).len());
}

///
/// Find an actor that plays in a movie, and in its two successors,
/// as well as another movie.
///
/// This will be fabian, and "Jurassic Park" the fourth movie.
///
#[test]
fn req_test() {
    let mut pattern_graph = petgraph::graph::Graph::new();
    let f = pattern_graph.add_node_to_match(|x| check_for_actor(x, "fabian"));
    let j = pattern_graph.add_node_to_match(matcher!(MovieNode::Movie(_)));
    // Other movies
    let m1 = pattern_graph.add_node_to_match_full(matcher!(MovieNode::Movie(_)), true);
    let m2 = pattern_graph.add_node_to_match_full(matcher!(MovieNode::Movie(_)), true);
    let m3 = pattern_graph.add_node_to_match_full(matcher!(MovieNode::Movie(_)), true);

    // Relations
    let pi = pattern_graph.add_edge_to_match(f, j, matcher!(PlaysIn));
    pattern_graph.add_edge_to_match_full(f, m1, matcher!(PlaysIn), true);
    pattern_graph.add_edge_to_match_full(f, m2, matcher!(PlaysIn), true);
    pattern_graph.add_edge_to_match_full(f, m3, matcher!(PlaysIn), true);
    pattern_graph.add_edge_to_match_full(m1, m2, matcher!(Successor), true);
    pattern_graph.add_edge_to_match_full(m3, m2, matcher!(Successor), true);

    // Query
    let base_graph = full_graph().0;
    let results = VfState::eval(&pattern_graph, &base_graph);
    assert_eq!(1, results.len());

    // Check movie
    let (_, movie) = results[0].adjacent_nodes(pi);
    assert_eq!(movie, j);
    assert!(check_movie(
        results[0].node_weight(movie),
        "Jurassic Park",
        1990
    ));
}

///
/// Find a person that knows another person, that themselves plays in a movie.
/// Ignore the PlaysIn relation in the result.
///
#[test]
fn seq_create() {
    // Two persons, one movie
    let mut pattern_graph = petgraph::graph::Graph::new();
    let p1 = pattern_graph.add_node_to_match(matcher!(MovieNode::Person(_)));
    let p2 = pattern_graph.add_node_to_match(matcher!(MovieNode::Person(_)));
    let m = pattern_graph.add_node_to_match(matcher!(MovieNode::Movie(_)));

    // Two relations
    pattern_graph.add_edge_to_match(p1, p2, matcher!(Knows));
    pattern_graph.add_edge_to_match_full(p2, m, matcher!(PlaysIn), true);

    // Query yields seven results
    let base_graph = full_graph().0;
    let results = VfState::eval(&pattern_graph, &base_graph);
    assert_eq!(14, results.len());

    // PlaysIn does not appear in the result
    for res in results {
        assert_eq!(1, res.count_edges());
        assert_eq!(0, res.adjacent_edges(m).count());
    }
}

///
/// Find a person that knows to other persons.
/// These persons play in different movies, but we ignore those.
///
#[test]
fn all_stereotypes_2() {
    let mut pattern_graph = petgraph::graph::Graph::new();
    // Three persons
    let p1 = pattern_graph.add_node_to_match(matcher!(MovieNode::Person(_)));
    let p2 = pattern_graph.add_node_to_match(matcher!(MovieNode::Person(_)));
    let p3 = pattern_graph.add_node_to_match(matcher!(MovieNode::Person(_)));
    // Two movies
    let m1 = pattern_graph.add_node_to_match_full(matcher!(MovieNode::Movie(_)), true);
    let m2 = pattern_graph.add_node_to_match_full(matcher!(MovieNode::Movie(_)), true);

    // Four relations
    pattern_graph.add_edge_to_match(p1, p2, matcher!(Knows));
    pattern_graph.add_edge_to_match(p1, p3, matcher!(Knows));
    pattern_graph.add_edge_to_match_full(p2, m1, matcher!(PlaysIn), true);
    pattern_graph.add_edge_to_match_full(p3, m2, matcher!(PlaysIn), true);

    // Query with two results
    let base_graph = full_graph().0;
    let results = VfState::eval(&pattern_graph, &base_graph);
    assert_eq!(2, results.len())
}
