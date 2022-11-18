use rustgql::print::VizDotGraph;
use test_dir::{DirBuilder, TestDir};

pub mod person_graph_types;

///
/// Test Case for Printing to GraphViz dot format:
/// Create the String representation of a Graph, then compare it to the created String.
///
#[test]
fn test_petgraph_print() {
    let graph: Box<dyn VizDotGraph<_, _, _, _>> = Box::new(person_graph_types::make_sample_graph());
    // Printed String we get
    let dot_print = r#graph.print();
    // Result String we expect
    let actual_print = 
r#"digraph {
    0 [ label = "Student { name: \"tobias\", age: 99, matrical_number: 900000 }" ]
    1 [ label = "Student { name: \"stefan\", age: 9, matrical_number: 89000 }" ]
    2 [ label = "Student { name: \"horst\", age: 55, matrical_number: 823340 }" ]
    3 [ label = "Professor { name: \"bettina\", age: 36, faculty: \"Faculty of Software Engineering and Programming Languages\" }" ]
    0 -> 2 [ label = "FriendOf { since_year: 2020 }" ]
    2 -> 3 [ label = "FriendOf { since_year: 2010 }" ]
    3 -> 2 [ label = "FriendOf { since_year: 2010 }" ]
    1 -> 3 [ label = "FriendOf { since_year: 2018 }" ]
}
"#;
    assert_eq!(actual_print, dot_print);
}

///
/// Tests that graphviz can generate a .svg file for our sample graph.
///
#[test]
fn test_petgraph_svg_print() {
    let dir = TestDir::temp();
    let graph: Box<dyn VizDotGraph<_, _, _, _>> = Box::new(person_graph_types::make_sample_graph());
    let graph_svg_test_res = graph.print_to_svg(dir.path("persons").to_str().unwrap());
    assert!(graph_svg_test_res.is_ok());
}
