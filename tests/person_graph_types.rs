use petgraph::{graph::Graph, stable_graph::DefaultIx, Directed, Undirected};

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
pub fn make_sample_graph() -> Graph<Person, FriendOf> {
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
