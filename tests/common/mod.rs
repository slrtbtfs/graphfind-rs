use std::collections::HashMap;

use petgraph::{
    graph::{DefaultIx, EdgeIndex, Graph, NodeIndex},
    Directed, EdgeType, Undirected,
};

use serde::{Deserialize, Serialize};

///
/// Turn into Trait Object, moved here.
///
pub fn into_trait_object<N, E, D: EdgeType>(
    g: petgraph::graph::Graph<N, E, D, DefaultIx>,
) -> impl graphfind_rs::graph::Graph<
    N,
    E,
    NodeRef = petgraph::graph::NodeIndex,
    EdgeRef = petgraph::graph::EdgeIndex,
> {
    g
}

///
/// Defines the data of a test graph:
///
/// - Nodes:
/// - Person (name, age)
/// - Student (extends Student, matrical number)
/// - Professor (extends Student, faculty)
/// - Edges:
/// - FriendOf (since year)
///

// Defined Node Types
///
/// Person enum/Uses redundant data for now.
///

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub role: Role,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Role {
    Professor { faculty: String },
    Student { matrical_number: u32 },
}

///
/// FriendOf Struct
///
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub struct FriendOf {
    pub since_year: i32,
}

///
/// Implementation of Edge Type.
///
impl FriendOf {
    pub fn new(year: i32) -> FriendOf {
        FriendOf { since_year: year }
    }
}

///
/// Factory Method for Student.
///
pub fn new_student(name: &str, age: u32, matrical_number: u32) -> Person {
    Person {
        name: String::from(name),
        age,
        role: Role::Student { matrical_number },
    }
}

///
/// Factory Method for Professor.
///
pub fn new_professor(name: &str, age: u32, faculty: &str) -> Person {
    Person {
        name: String::from(name),
        age,
        role: Role::Professor {
            faculty: String::from(faculty),
        },
    }
}

/// NodeInfo and EdgeInfo types that allow us to compare
/// what we inserted into a Graph and what we get back.
type NodeInfo<N> = HashMap<NodeIndex, N>;
type EdgeInfo<E> = HashMap<EdgeIndex, (NodeIndex, NodeIndex, E)>;

///
/// Creates a sample graph for testing directed graphs, and copies of node/edge weights & references
/// to conduct structural tests.
///
/// See (TODO add file) for an graphical overview.
///
pub fn make_sample_graph() -> (
    Graph<Person, FriendOf>,
    NodeInfo<Person>,
    EdgeInfo<FriendOf>,
) {
    // Graph maintains enums of Person, and FriendOf.
    let mut graph: Graph<Person, FriendOf, Directed, DefaultIx> = Graph::new();
    let mut node_raw = HashMap::new();
    let mut edge_raw = HashMap::new();

    // Student 1/Tobias
    let tobias = new_student("tobias", 99, 900000);
    let x = tobias.clone();
    let t = graph.add_node(tobias);
    node_raw.insert(t, x);

    // Student 2/Stefan
    let stefan = new_student("stefan", 9, 89000);
    let x = stefan.clone();
    let s = graph.add_node(stefan);
    node_raw.insert(s, x);

    // Student 3/Horst
    let horst = new_student("horst", 55, 823340);
    let x = horst.clone();
    let h = graph.add_node(horst);
    node_raw.insert(h, x);

    // Professor/Bettina
    let bettina = new_professor(
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

/// Provides a second set of test data; missing one edge and directly accessible structural information.
///
/// Returns an owned data type rather than a reference so the calling function
/// is then responsible for cleaning up the value.
pub fn make_sample_graph_variant() -> Graph<Person, FriendOf> {
    // Graph maintains pointers to Person, and FriendOf.
    let mut graph: Graph<Person, FriendOf> = Graph::new();

    // Student 1/Tobias
    let tobias = new_student("tobias", 99, 900000);
    let t = graph.add_node(tobias);

    // Student 2/Stefan
    let stefan = new_student("stefan", 9, 89000);
    let s = graph.add_node(stefan);

    // Student 3/Horst
    let horst = new_student("horst", 55, 823340);
    let h = graph.add_node(horst);

    // Professor/Bettina
    let bettina = new_professor(
        "bettina",
        36,
        "Faculty of Software Engineering and Programming Languages",
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

///
/// Creates a new undirected sample graph, based in part
/// on the tramways in Ulm.
///
pub fn make_sample_graph_undirected<'a>() -> (
    Graph<&'a str, i32, Undirected>,
    NodeInfo<&'a str>,
    EdgeInfo<i32>,
) {
    let mut g = Graph::new_undirected();
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
    let bi_ti = g.add_edge(bi, ei, 17);
    let ei_söi = g.add_edge(ei, söi, 8);

    // Add connections
    let mut routes = HashMap::new();
    routes.insert(ei_ti, (ei, ti, 5));
    routes.insert(ti_si, (si, ti, 12));
    routes.insert(ti_ki, (ki, ti, 15));
    routes.insert(bi_ti, (bi, ei, 17));
    routes.insert(ei_söi, (ei, söi, 8));

    (g, stations, routes)
}

///
/// This part defines the movie meta-model for graph queries.
///

///
/// Movie type.
///
#[derive(Debug)]
pub enum MovieType {
    Movie,
    Tv,
    Video,
    VideoGame,
}

///
/// Movie (name, type, rating, year).
///
#[derive(Debug)]
pub struct Movie {
    pub title: String,
    pub rating: f64,
    pub year: i32,
    pub type_of: MovieType,
}

///
/// Actor type.
///
#[derive(Debug)]
pub enum ActorType {
    Actor,
    Actress,
}

///
/// MoviePerson (Name, Type).
///
#[derive(Debug)]
pub struct MoviePerson {
    pub name: String,
    pub type_of: ActorType,
}

///
/// Creates a new Person that plays in movies
///
pub fn make_person(name: String, type_of: ActorType) -> MovieNode {
    MovieNode::Person(MoviePerson { name, type_of })
}

///
/// Creates a new Movie.
///
pub fn make_movie(title: String, rating: f64, year: i32, type_of: MovieType) -> MovieNode {
    MovieNode::Movie({
        Movie {
            title,
            rating,
            year,
            type_of,
        }
    })
}

///
/// Possible Movie objects.
///
#[derive(Debug)]
pub enum MovieNode {
    Movie(Movie),
    Person(MoviePerson),
}

///
/// Relations between Movie graph objects.
///
#[derive(Debug)]
pub enum Relation {
    /// Knows (MoviePerson -> MoviePerson)
    Knows,
    /// Couple (MoviePerson - MoviePerson, undirected)
    Couple,
    /// PlaysIn (MoviePerson -> Movie, directed)
    PlaysIn,
    /// Successor (Movie -> Movie, directed)
    Successor,
}
