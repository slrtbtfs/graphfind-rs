/**
 * Defines simple tests for the graph API using a Petgraph backend.
 */
// Declare to use code in the module/file ./person_graph_types.rs
pub mod person_graph_types;
use std::{iter::Map, collections::HashMap};

use petgraph::stable_graph::{StableGraph, NodeIndex, EdgeIndex};

// Use Code
use crate::person_graph_types::{Person, Professor, ProfessorStruct, Student, StudentStruct, FriendOf};

fn make_sample_graph() {
    // Student 1/Tobias
    let tobias: StudentStruct = 
        Student::new("tobias", 99, 900000);
    
    // Student 2/Stefan
    let stefan: StudentStruct =
        Student::new("stefan", 9, 89000);

    // Student 3/Horst
    let horst: StudentStruct =
        Student::new("horst", 55, 823340);
    
    // Professor/Bettina
    let bettina: ProfessorStruct = 
        Professor::new("bettina", 36, "Faculty of Software Engineering and Programming Langauges");
    
    // Tobias is a friend of Horst
    let th = FriendOf::new(2020);
    // Horst and Bettina are friends
    let hb = FriendOf::new(2010);
    let bh = FriendOf::new(2010);
    // Stefan is a friend of Bettina
    let sb = FriendOf::new(2018);

    // Graph maintains pointers to StudentStructs, and FriendOf.
    // &dyn Person tells Rust I want Types that have trait Person.
    let mut graph: StableGraph<&dyn Person, &FriendOf> =
        StableGraph::new();
    
    // Add nodes to graph, maintain mapping.
    let mut node_refs: HashMap<NodeIndex, &dyn Person> = HashMap::new();
    let edge_refs: HashMap<EdgeIndex, &FriendOf> = HashMap::new();

    let t = graph.add_node(&tobias);
    let h = graph.add_node(&horst);
    let b = graph.add_node(&bettina);
    let s = graph.add_node(&stefan);

    // Connect with edges
    let th_edge = graph.add_edge(t, h, &th);
    let hb_edge = graph.add_edge(h, b, &hb);
    let bh_edge = graph.add_edge(b, h, &bh);
    let sb_edge = graph.add_edge(s, b, &sb);
} 