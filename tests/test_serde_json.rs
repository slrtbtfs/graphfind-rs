use petgraph::{
    graph::Graph,
    visit::{EdgeRef, IntoNodeReferences},
    Directed,
};
use graphfind_rs::graph::GraphReadWriter;
use test_dir::{DirBuilder, TestDir};

pub mod common;
use common::{make_sample_graph_variant, FriendOf, Person};
/// File names
const NAME_TO_READ_AND_WRITE: &str = "path.json";
const EMPTY_FILE_NAME: &str = "empty.json";
const MISSING_NAME: &str = "missing.json";
const MISSING_DIR_NAME: &str = "missing_dir/unwritable_file.json";

#[test]
fn test_serde_json_graph_read_write() {
    // Make a new temp directory
    let dir = TestDir::current_rnd();

    let graph_back = make_sample_graph_variant();

    // Serialize graph.
    graph_back
        .serialize_graph_to_file(
            // Append file name to temp dir; make to string, expect it to be there.
            &append_path(&dir, NAME_TO_READ_AND_WRITE),
        )
        .unwrap();
    // Deserialize and pack in a Box.
    let graph: Box<petgraph::graph::Graph<Person, FriendOf, Directed, _>> =
        GraphReadWriter::deserialize_graph_from_file(&append_path(&dir, NAME_TO_READ_AND_WRITE))
            .unwrap();

    // Assert # of edges, nodes is equal.
    assert_eq!(graph_back.edge_count(), graph.edge_weights().count());
    assert_eq!(graph_back.node_count(), graph.node_weights().count());

    // Iterate over nodes, assume equivalency
    for node_ref in graph_back.node_references() {
        assert_eq!(graph.node_weight(node_ref.0).unwrap(), node_ref.1);
    }
    // Iterate over edges, assume equivalency
    for edge_ref in graph_back.edge_references() {
        assert_eq!(
            graph.edge_weight(EdgeRef::id(&edge_ref)).unwrap(),
            edge_ref.weight()
        );
    }
}

/// Test that checks for read failures
#[test]
fn test_serde_file_not_exists() {
    let dir = TestDir::temp().create(EMPTY_FILE_NAME, test_dir::FileType::EmptyFile);

    // Fail to serialize from a file that doesn't exist.
    let read_attempt: Result<Box<Graph<Person, FriendOf, Directed, _>>, std::io::Error> =
        GraphReadWriter::deserialize_graph_from_file(&append_path(&dir, MISSING_NAME));
    let err = read_attempt.expect_err("Read attempt from missing file should fail.");

    // Default Rust Error to expect
    assert_eq!(err.kind(), std::io::ErrorKind::NotFound);

    // Create an empty file and try to deserialize from it.
    let read_attempt: Result<Box<Graph<Person, FriendOf, Directed, _>>, std::io::Error> =
        GraphReadWriter::deserialize_graph_from_file(&append_path(&dir, EMPTY_FILE_NAME));
    let err = read_attempt.expect_err("Read from empty file should fail.");

    assert_eq!(err.kind(), std::io::ErrorKind::Other);
    assert!(err.into_inner().is_some());
}

#[test]
fn test_write_error_nonexistent_dir() {
    let dir = TestDir::temp();

    let graph: Graph<(), ()> = petgraph::Graph::new();
    let write_attempt = graph.serialize_graph_to_file(&append_path(&dir, MISSING_DIR_NAME));

    dbg!(&write_attempt);

    let err = write_attempt.expect_err("Write to nonexistent dir should fail");
    assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
}

fn append_path(dir: &TestDir, path: &str) -> String {
    let buffer = dir.path(path);
    buffer.to_str().unwrap().to_string()
}
