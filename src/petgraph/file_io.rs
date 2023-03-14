use crate::graph::GraphReadWriter;
use petgraph::stable_graph::DefaultIx;
use serde::{de::DeserializeOwned, Serialize};
use std::{fs::File, io::Error as IOError};

///
/// Implementation of GraphReadWriter trait using serde_json.
/// Nodes and Edges need to implement Serializable and Deserializable
/// in order for serde to work.
///
impl<NodeWeight, EdgeWeight, EdgeType> GraphReadWriter<NodeWeight, EdgeWeight>
    for petgraph::Graph<NodeWeight, EdgeWeight, EdgeType, DefaultIx>
where
    NodeWeight: Serialize + DeserializeOwned,
    EdgeWeight: Serialize + DeserializeOwned,
    EdgeType: petgraph::EdgeType,
{
    ///
    /// Serializes the graph to JSON. This overwrites the file given under path.
    /// If serde_json fails, packs the underlying error in an std::io::Error for examination.
    ///
    fn serialize_graph_to_file(&self, path: &str) -> Result<(), IOError> {
        let file = File::create(path)?;
        serde_json::ser::to_writer(file, &self)
            .map_err(|e| IOError::new(std::io::ErrorKind::Other, e))
    }

    ///
    /// Deserializes a graph stored as JSON, and packs it into a Box.
    /// If serde_json fails, packs the underlying error in an std::io::Error for examination.
    ///
    fn deserialize_graph_from_file(path: &str) -> Result<Box<Self>, IOError> {
        let file = File::open(path)?;
        serde_json::de::from_reader(file)
            .map(Box::new)
            .map_err(|e| IOError::new(std::io::ErrorKind::Other, e))
    }
}
