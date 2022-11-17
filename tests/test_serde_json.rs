use std::fs::{File, self};

use petgraph::{visit::{IntoNodeReferences, IntoEdgeReferences, EdgeRef}, stable_graph::StableGraph};
use rustgql::{file_io::StableGraphReadWriter, file_io_backends, graph::Graph};
use test_dir::{TestDir, DirBuilder};
use crate::person_graph_types::{Person, FriendOf};

mod person_graph_types;

/// File names
const NAME_TO_READ_AND_WRITE: &str = "path.json";
const EMPTY_FILE_NAME: &str = "empty.json";
const MISSING_NAME: &str = "missing.json";

#[test]
fn test_serde_json_graph_read_write() {
    // Make a new temp directory
    let dir = TestDir::current_rnd();

    let read_writer
        = file_io_backends::petgraph::JsonStableGraphReadWriter::default();
    let graph_back = person_graph_types::make_sample_graph();

    // Serialize graph.
    read_writer.serialize_graph(
        // Append file name to temp dir; make to string, expect it to be there.
        &append_path(&dir, NAME_TO_READ_AND_WRITE),
        &graph_back).unwrap();
    // Deserialize and pack in a Box.
    let graph: Box<dyn Graph<Person, FriendOf, _, _>> =
        read_writer.deserialize_graph(&append_path(&dir, NAME_TO_READ_AND_WRITE))
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
        assert_eq!(graph.edge_weight(EdgeRef::id(&edge_ref)).unwrap(), edge_ref.weight());
    }
}

/// Test that checks for read failures
#[test]
fn test_serde_file_not_exists() {
    let dir = TestDir::temp()
        .create(EMPTY_FILE_NAME, test_dir::FileType::EmptyFile);
    
    // Fail to serialize from a file that doesn't exist.
    let read_writer
        = file_io_backends::petgraph::JsonStableGraphReadWriter::default();
    let read_attempt: Result<Box<StableGraph<Person, FriendOf, _, _>>, std::io::Error>
        = read_writer.deserialize_graph
        (&append_path(&dir, MISSING_NAME));
    let err = read_attempt.expect_err("Read attempt from missing file should fail.");

    // Default Rust Error to expect
    assert_eq!(err.kind(), std::io::ErrorKind::NotFound);

    // Create an empty file and try to deserialize from it.
    let read_attempt: Result<Box<StableGraph<Person, FriendOf, _, _>>, std::io::Error> =
        read_writer.deserialize_graph(&append_path(&dir, EMPTY_FILE_NAME));
    let err = read_attempt.expect_err("Read from empty file should fail.");

    assert_eq!(err.kind(), std::io::ErrorKind::Other);
    assert!(err.into_inner().is_some());
}

fn append_path(dir: &TestDir, path: &str) -> String {
    let buffer = dir.path(path);
    buffer.to_str().unwrap().to_string()
    // x.to_string()
    // todo!()
}