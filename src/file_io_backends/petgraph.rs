use serde::{Serialize, de::DeserializeOwned};
use crate::file_io::{StableGraphImpl, StableGraphReadWriter};
use std::{io::Error as IOError, fs::File};

///
/// Struct to be used by clients.
///
pub struct JsonStableGraphReadWriter {}

impl JsonStableGraphReadWriter {
    pub fn new() -> Self {
        JsonStableGraphReadWriter {}
    }
}

impl Default for JsonStableGraphReadWriter {
    ///
    /// Constructs a new JsonStableGraphReadWriter struct.
    /// 
    fn default() -> Self {
        JsonStableGraphReadWriter::new()
    }
}

///
/// Implementation of StableGraphReadWriter trait using serde_json.
/// Nodes and Edges need to implement Serializable and Deserializable
/// in order for serde to work.
///  
impl <NodeWeight, EdgeWeight>
    StableGraphReadWriter<NodeWeight, EdgeWeight>
    for JsonStableGraphReadWriter
where
    NodeWeight: Serialize + DeserializeOwned,
    EdgeWeight: Serialize + DeserializeOwned
{
    ///
    /// Serializes the graph to JSON. This overwrites the file given under path.
    /// If serde_json fails, packs the underlying error in an std::io::Error for examination.
    /// 
    fn serialize_graph(&self, 
        path: &str,
        graph: &StableGraphImpl<NodeWeight, EdgeWeight>) -> Result<(), IOError> {
        let file = File::create(path)?;
        match serde_json::ser::to_writer(file, graph) {
            Ok(_) => Ok(()),
            Err(e) => Err(IOError::new(std::io::ErrorKind::Other, e)),
        }
    }

    ///
    /// Deserializes a graph stored as JSON, and packs it into a Box.
    /// If serde_json fails, packs the underlying error in an std::io::Error for examination.
    /// 
    fn deserialize_graph(&self, 
        path: &str) -> Result<Box<StableGraphImpl<NodeWeight, EdgeWeight>>, IOError> {
        let file = File::open(path)?;
        let res: Result<StableGraphImpl<NodeWeight, EdgeWeight>, serde_json::Error> 
            = serde_json::de::from_reader(file);
        match res {
            Ok(graph) => Ok(Box::new(graph)),
            Err(e) => Err(IOError::new(std::io::ErrorKind::Other, e))
        }
    }
}