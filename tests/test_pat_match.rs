pub mod common;
use std::collections::HashMap;

use common::{
    ActorType::Actor,
    MovieNode,
    MovieType::{Movie, Tv, Video},
    Relation,
    Relation::PlaysIn,
};
use petgraph::graph::{Graph, NodeIndex};

macro_rules! add_person {
    ($g:expr, $map:expr, $name:expr, $ty:expr) => {
        let node = common::make_person($name.to_string(), $ty);
        $map.insert($name, $g.add_node(node));
    };
}

macro_rules! add_media {
    ($g:expr, $map:expr, $name:expr, $rat:expr, $year:expr, $ty:expr) => {
        let node = common::make_movie($name.to_string(), $rat, $year, $ty);
        $map.insert($name, $g.add_node(node));
    };
}

macro_rules! connect {
    ($g:expr, $map:expr, $from:expr, $e:expr, $to:expr) => {
        let node1 = $map[$from];
        let node2 = $map[$to];
        $g.add_edge(node1, node2, $e);
    };
}

///
/// Gives a graph that only contains nodes. Can be modified
/// to contain different edges to match patterns on.
///
fn node_graph<'a>() -> (Graph<MovieNode, Relation>, HashMap<&'a str, NodeIndex>) {
    let mut graph: petgraph::Graph<MovieNode, Relation> = Graph::new();
    let mut names: HashMap<&str, NodeIndex> = HashMap::new();

    // Stefan, Yves, Fabian
    add_person!(graph, names, "stefan", Actor);
    add_person!(graph, names, "yves", Actor);
    add_person!(graph, names, "fabian", Actor);

    // Media
    add_media!(graph, names, "Jurassic Park", 10.0, 1993, Movie);
    add_media!(graph, names, "Star Wars Holiday Special", -10.0, 1978, Tv);
    add_media!(
        graph,
        names,
        "Attack of the Killer Macros",
        6.9,
        2022,
        Video
    );

    (graph, names)
}

///
/// Match relations between Actors and Movies.
///
#[test]
fn test_create_match() {
    let (mut graph, nodes) = node_graph();

    // Three relations
    connect!(graph, nodes, "stefan", PlaysIn, "Jurassic Park");
    connect!(graph, nodes, "yves", PlaysIn, "Attack of the Killer Macros");
    connect!(graph, nodes, "fabian", PlaysIn, "Star Wars Holiday Special");
}

///
/// Assert that a graph is empty when there is no actor
/// that plays in a movie.
///
#[test]
fn test_create_negative() {
    // Take Graph without relations
    let (graph, _) = node_graph();
}

///
/// Match every Actor who plays in at least two movies,
/// and only give us the Actor-Movie relations.
///
#[test]
fn test_require_delete_two_movies() {}

///
/// Match a given subgraph with:
/// 1. Three different actors
/// 2. Three different movies
/// 3. Two actors play in all three movies.
///
/// Assume we get three matches.
///
#[test]
fn test_three_to_two_match() {}

///
/// Match a given subgraph with:
/// 1. Three different actors
/// 2. Three different movies
/// 3. Each actor plays in all three movies.
///
/// Assume we get nine matches.
///
#[test]
fn test_three_to_three_match() {}

///
/// Match a given graph on:
/// 1. Three different actors
/// 2. Three different movies
/// 3. Two actors play in the two same movies.
///
#[test]
fn test_two_to_two_match() {}

///
/// Find all persons who play in at least three movies,
/// and one other person plays in one of these movies as well.
/// However, do not give us the latter person.
///
#[test]
fn test_all_stereotypes() {}
