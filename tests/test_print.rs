use rustgql::print::VizDotGraph;

pub mod person_graph_types;

///
/// Test Case for Printing to GraphViz dot format:
/// Create the String representation of a StableGraph, then compare it to the created String.
/// 
#[test]
fn test_petgraph_print() {
    let graph: Box<dyn VizDotGraph<_, _, _ , _>> = Box::new(person_graph_types::make_sample_graph());
    // Printed String we get
    let dot_print = graph.print();
    // Result String we expect
    let actual_print = "digraph {
        0 [ label = \"Student { name: \\\"tobias\\\", age: 99, matrical_number: 900000 }\" ]
        1 [ label = \"Student { name: \\\"stefan\\\", age: 9, matrical_number: 89000 }\" ]
        2 [ label = \"Student { name: \\\"horst\\\", age: 55, matrical_number: 823340 }\" ]
        3 [ label = \"Professor { name: \\\"bettina\\\", age: 36, faculty: \\\"Faculty_of_Software_Engineering_and_Programming_Languages\\\" }\" ]
        0 -> 2 [ label = \"FriendOf { since_year: 2020 }\" ]
        2 -> 3 [ label = \"FriendOf { since_year: 2010 }\" ]
        3 -> 2 [ label = \"FriendOf { since_year: 2010 }\" ]
        1 -> 3 [ label = \"FriendOf { since_year: 2018 }\" ]
    }
    ");
    assert_eq!(actual_print.replace(' ', ""), dot_print.replace(' ', ""));
}
