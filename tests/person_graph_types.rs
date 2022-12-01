use std::collections::HashMap;

use petgraph::{
    graph::{DefaultIx, EdgeIndex, Graph as BaseGraph, NodeIndex},
    Directed, Undirected,
};

use serde::{Deserialize, Serialize};

///
/// Turn into Trait Object, moved here.
///
pub fn into_trait_object<N, E>(
    g: petgraph::graph::Graph<N, E, Directed, DefaultIx>,
) -> impl rustgql::graph::Graph<
    N,
    E,
    NodeRef = petgraph::graph::NodeIndex,
    EdgeRef = petgraph::graph::EdgeIndex,
> {
    g
}

pub fn into_trait_object_undirected<N, E>(
    g: petgraph::graph::Graph<N, E, Undirected, DefaultIx>,
) -> impl rustgql::graph::Graph<
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
pub enum Person {
    Student {
        name: String,
        age: u32,
        matrical_number: u32,
    },
    Professor {
        name: String,
        age: u32,
        faculty: String,
    },
}

impl Person {
    pub fn name(&self) -> &String {
        match self {
            Person::Student { name, .. } => name,
            Person::Professor { name, .. } => name,
        }
    }
}

/**
 * FriendOf Struct
 */
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub struct FriendOf {
    pub since_year: i32,
}

// Implementation of Edge Type
impl FriendOf {
    pub fn new(year: i32) -> FriendOf {
        FriendOf { since_year: year }
    }
}

// Factory Methods
pub fn new_student(name: &str, age: u32, matrical_number: u32) -> Person {
    Person::Student {
        name: String::from(name),
        age,
        matrical_number,
    }
}

pub fn new_professor(name: &str, age: u32, faculty: &str) -> Person {
    Person::Professor {
        name: String::from(name),
        age,
        faculty: String::from(faculty),
    }
}

/// Provide Test Data -> TODO: Once graph_impl-branch is correct and merged into main,
/// update main so that test data is created in a own module, and used in other modules.

/// Returns an owned data type rather than a
/// reference so the calling function is then responsible for cleaning up the value.
pub fn make_sample_graph_2() -> BaseGraph<Person, FriendOf> {
    // Graph maintains pointers to Person, and FriendOf.
    let mut graph: BaseGraph<Person, FriendOf> = petgraph::graph::Graph::new();

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

/// NodeInfo and EdgeInfo types that allow us to compare
/// what we inserted into a Graph and what we get back-
type NodeInfo<N> = HashMap<NodeIndex, N>;
type EdgeInfo<E> = HashMap<EdgeIndex, (NodeIndex, NodeIndex, E)>;

///
/// Creates a sample graph for testing directed graphs.
/// See (TODO add file) for an graphical overview.
///
pub fn make_sample_graph() -> (
    BaseGraph<Person, FriendOf>,
    NodeInfo<Person>,
    EdgeInfo<FriendOf>,
) {
    // Graph maintains enums of Person, and FriendOf.
    let mut graph: BaseGraph<Person, FriendOf, Directed, DefaultIx> = BaseGraph::new();
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

///
/// Creates a new undirected sample graph, based in part
/// on the tramways in Ulm.
///
pub fn make_sample_graph_undirected<'a>() -> (
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
