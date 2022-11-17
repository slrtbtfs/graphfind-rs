use crate::file_io::{GraphImpl, GraphReadWriter};
use serde::{de::DeserializeOwned, Serialize};
use std::{fs::File, io::Error as IOError};

///
/// Struct to be used by clients.
///
#[derive(Default)]
pub struct JsonGraphReadWriter {}

impl JsonGraphReadWriter {
    ///
    /// Constructs a new JsonGraphReadWriter struct.
    ///
    pub fn new() -> Self {
        JsonGraphReadWriter {}
    }
}

///
/// Implementation of GraphReadWriter trait using serde_json.
/// Nodes and Edges need to implement Serializable and Deserializable
/// in order for serde to work.
///  
impl<NodeWeight, EdgeWeight> GraphReadWriter<NodeWeight, EdgeWeight> for JsonGraphReadWriter
where
    NodeWeight: Serialize + DeserializeOwned,
    EdgeWeight: Serialize + DeserializeOwned,
{
    ///
    /// Serializes the graph to JSON. This overwrites the file given under path.
    /// If serde_json fails, packs the underlying error in an std::io::Error for examination.
    ///
    fn serialize_graph(
        &self,
        path: &str,
        graph: &GraphImpl<NodeWeight, EdgeWeight>,
    ) -> Result<(), IOError> {
        let file = File::create(path)?;
        serde_json::ser::to_writer(file, graph)
            .map_err(|e| IOError::new(std::io::ErrorKind::Other, e))
    }

    ///
    /// Deserializes a graph stored as JSON, and packs it into a Box.
    /// If serde_json fails, packs the underlying error in an std::io::Error for examination.
    ///
    fn deserialize_graph(
        &self,
        path: &str,
    ) -> Result<Box<GraphImpl<NodeWeight, EdgeWeight>>, IOError> {
        let file = File::open(path)?;
        serde_json::de::from_reader(file)
            .map(Box::new)
            .map_err(|e| IOError::new(std::io::ErrorKind::Other, e))
    }
}
