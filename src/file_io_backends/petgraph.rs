use std::fs::File;

use petgraph::{stable_graph::{DefaultIx, EdgeIndex, NodeIndex}, Directed};
use serde::{Serialize, de::DeserializeOwned};

use serde_json::Error as JSONError;
use std::io::Error as RustError;

use crate::file_io::FileIO;

/**
 * PetgraphToJsonIO Implements the FileIO trait for:
 * 
 * 1. Graphs using the petgraph input format,
 * 2. Output files that store JSON objects.
 * 
 * This implementation is based on serde 1.0; it requires nodes and edges to
 * implement Serializable/DeserializeOwned. See https://serde.rs/lifetimes.html.
 */
impl<NodeWeight, EdgeWeight>
    FileIO<NodeWeight, EdgeWeight, NodeIndex, EdgeIndex>
    for petgraph::stable_graph::StableGraph<NodeWeight, EdgeWeight, Directed, DefaultIx>
where
    NodeWeight: DeserializeOwned + Serialize,
    EdgeWeight: DeserializeOwned + Serialize,
{
    /// Serializes this graph. 
    fn serialize_graph(&self, path: &str) -> Result<(), RustError> {
        let file_handle = File::create(path)?;
        translate_error(serde_json::ser::to_writer(file_handle, &self))
    }

    /// Deserializes the file under the given handle into a new graph.
    /// Asserts that "path" only holds the graph to deserialize.
    fn deserialize_graph(path: &str) -> Result<Box<petgraph::prelude::StableGraph<NodeWeight, EdgeWeight>>, RustError> {
        let file_handle = File::open(path)?;
        translate_error(serde_json::de::from_reader(file_handle))
    }
}

/// Translates an serde_json error into a Rust IO Error (basically packs the serde_json in an IO error).
fn translate_error<T> (res: Result<T, JSONError>) -> Result<T, RustError> {
    match res {
        Ok(x) => Ok(x),
        Err(json_error) => Err(RustError::new(std::io::ErrorKind::Other, json_error)),
    }
}