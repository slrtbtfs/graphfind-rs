pub mod common;
use std::collections::{HashMap, HashSet};

use common::{
    ActorType::Actor,
    MovieNode,
    MovieType::{Movie, Tv, Video},
    Relation,
    Relation::PlaysIn,
};
use petgraph::{
    graph::{Graph, NodeIndex},
    visit::NodeRef,
};
use rustgql::{
    query::{PatternGraph, SubgraphAlgorithm},
    query_algorithms::vf_algorithms::VfAlgorithm,
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

    // Stefan, Yves, Fabian
    add_person(&mut graph, &mut names, "stefan", Actor);
    add_person(&mut graph, &mut names, "yves", Actor);
    add_person(&mut graph, &mut names, "fabian", Actor);

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
/// Given an empty pattern graph and a non-empty base graph,
/// assert that we do get an empty result.
///
#[test]
fn test_empty_pattern_no_results() {
    let base_graph = node_graph().0;
    let empty_pattern = petgraph::graph::Graph::new();

    // Explicitly specify result type.
    let results: Vec<petgraph::graph::DiGraph<_, _, _>> =
        <VfAlgorithm as SubgraphAlgorithm>::find_subgraphs(&empty_pattern, &base_graph);
    assert!(results.is_empty());
}

///
/// Given a pattern with a single node to match, assert we get
/// nine graphs with one node each, and that we find the previous indices.
///
#[test]
fn test_single_node_any_pattern() {
    let base_graph = node_graph().0;
    let mut single_pattern = petgraph::graph::Graph::new();
    single_pattern.add_node_to_match("n", Box::new(|n: MovieNode| true));

    // Explicitly specify result type.
    let results: Vec<petgraph::graph::DiGraph<_, _, _>> =
        <VfAlgorithm as SubgraphAlgorithm>::find_subgraphs(&single_pattern, &base_graph);

    // Assert nine results.
    assert_eq!(9, results.len());
    let mut found_indices = HashSet::new();

    // Check that node indices are different.
    for res in results {
        assert_eq!(1, res.node_count());
        let indices = res.node_indices().collect::<Vec<_>>();
        let index = indices.get(0).unwrap().clone();
        found_indices.insert(index);
    }
    assert_eq!(9, found_indices.len());
}

/*
///
/// Match relations between Actors and Movies.
///
fn test_create_match() {
    let (mut graph, nodes) = node_graph();

    // Three relations
    connect(&mut graph, &nodes, "stefan", PlaysIn, "Jurassic Park");
    connect(
        &mut graph,
        &nodes,
        "yves",
        PlaysIn,
        "Attack of the Killer Macros",
    );
    connect(
        &mut graph,
        &nodes,
        "fabian",
        PlaysIn,
        "Star Wars Holiday Special",
    );

    //
    // Sketching a possible implementation:
    //
    // match! takes a Graph, and a pattern; produces a set of subgraphs.
    // Match on Person with given properties -> produce Result structure with
    // access to p and m, just as Cypher.
    //
    // let graphs: Vec<_> =
    // match!(graph, p:MovieNode::Person(..) -- [PlaysIn] m:MovieNode::Movie(..))
    // .collect();
    // assert_eq!(graphs.len(), 3);
    //
    // Translate that to:
    // let s = subgraph::match_on_node_pattern(&graph, MovieNode::Person(..));
    // let s = s::extend_directed(&g, PlaysIn, MovieNode::Movie(..));
    //
    // Actually, instead of a pattern, you could give a more general Fn: Node -> Option(Node),
    // and maintain for each subgraph the last node (index) we've seen.
    //
}

///
/// Assert that a graph is empty when there is no actor
/// that plays in a movie.
///
fn test_create_negative() {
    // Take Graph without relations
    let (graph, _) = node_graph();

    //
    // let graphs: Vec<_> =
    // match!(graph, MovieNode::Person(..) -> PlaysIn -> MovieNode::Movie(..))
    // .collect();
    // assert_eq!(graphs.len(), 0);
    //
}

///
/// Match every Actor who plays in at least two movies,
/// and only give us the Actor-Movie relations.
///
fn test_require_delete_two_movies() {}

///
/// Match a given subgraph with:
/// 1. Three different actors
/// 2. Three different movies
/// 3. Two actors play in all three movies.
///
/// Assume we get three matches.
///
fn test_three_to_two_match() {}

///
/// Match a given subgraph with:
/// 1. Three different actors
/// 2. Three different movies
/// 3. Each actor plays in all three movies.
///
/// Assume we get nine matches.
///
fn test_three_to_three_match() {}

///
/// Match a given graph on:
/// 1. Three different actors
/// 2. Three different movies
/// 3. Two actors play in the two same movies.
///
fn test_two_to_two_match() {}

///
/// Find all persons who play in at least three movies,
/// and one other person plays in one of these movies as well.
/// However, do not give us the latter person.
///
fn test_all_stereotypes() {}
*/
