use std::io;

use crate::graph::Graph;

///
/// Module/Trait to serialize and deserialize a given Graph to a file.
/// The file format depends on the graph type being used and can only
/// be assumed compatible with the same graph type.
///
pub trait GraphReadWriter<NodeWeight, EdgeWeight>: Graph<NodeWeight, EdgeWeight> {
    ///
    /// Serializes a given graph to a file defined by path.
    /// The result tells us whether the operation succeeded or not.
    ///  
    fn serialize_graph_to_file(&self, path: &str) -> Result<(), io::Error>;

    ///
    /// Deserializes a graph stored in the given file.
    /// The result tells us whether the operation succeeded or not.
    ///
    fn deserialize_graph_to_file(path: &str) -> Result<Box<Self>, io::Error>;
}
