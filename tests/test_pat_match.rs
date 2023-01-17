pub mod common;
use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
};

use common::{
    ActorType::Actor,
    MovieNode, MoviePerson,
    MovieType::{Movie, Tv, Video},
    Relation,
    Relation::PlaysIn,
};
use petgraph::{
    graph::{Graph, NodeIndex},
    visit::NodeRef,
};
use rustgql::{
    graph::Graph as QueryGraph,
    graph_backends::adj_graphs::AdjGraph,
    query::{PatternGraph, SubgraphAlgorithm},
    query_algorithms::{pattern_graphs, vf_algorithms::VfState},
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
    let mut query = VfState::init(&empty_pattern, &base_graph);
    query.run_query();
    let results = query.get_results();
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
    single_pattern.add_node_to_match("n", Box::new(|n: &MovieNode| true));

    // Explicitly specify result type.
    let mut query = VfState::init(&single_pattern, &base_graph);
    query.run_query();
    let results = query.get_results();

    // Assert nine results.
    assert_eq!(9, results.len());
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
    pattern_graph.add_node_to_match("s1", Box::new(|i: &i32| *i > 0));
    pattern_graph.add_node_to_match("s2", Box::new(|i: &i32| *i > 0));

    let mut base_graph = petgraph::graph::Graph::new();
    let index = base_graph.add_node(4);
    base_graph.add_edge(index, index, "equals");

    let mut query = VfState::init(&pattern_graph, &base_graph);
    query.run_query();
    assert_eq!(0, query.get_results().len());
}

///
/// Assert that when we query for a person pattern, we only get persons,
/// and that we get their different names.
///
#[test]
fn match_movie_nodes_only() {
    let mut pattern_graph = petgraph::graph::Graph::new();
    let person_index = pattern_graph.add_node_to_match(
        "p",
        Box::new(|mn| match *mn {
            MovieNode::Person(_) => true,
            _ => false,
        }),
    );

    let base_graph = node_graph().0;
    let mut query = VfState::init(&pattern_graph, &base_graph);
    query.run_query();

    let names: HashSet<_> = query
        .get_results()
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
    assert_eq!(3, names.len());
}

fn is_movie(node: &MovieNode) -> bool {
    match &node {
        MovieNode::Movie(_) => true,
        _ => false,
    }
}

fn is_person(node: &MovieNode) -> bool {
    match &node {
        MovieNode::Person(_) => true,
        _ => false,
    }
}

///
/// Given a pattern that matches a movie node and person node, assert that
/// we find all 6*3 = 18 different node pairs.
///
#[test]
fn match_two_node_pairs() {
    let mut pattern_graph = petgraph::graph::Graph::new();
    let m_index = pattern_graph.add_node_to_match("m", Box::new(is_movie));
    let p_index = pattern_graph.add_node_to_match("p", Box::new(is_person));
    let base_graph = node_graph().0;

    let mut query = VfState::init(&pattern_graph, &base_graph);
    query.run_query();
    let results = query.get_results();

    assert_eq!(6 * 3, results.len());
}

/*
#[test]
fn match_wrong_matches_only() {
    unimplemented!()
}
*/

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
