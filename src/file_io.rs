use std::io;

use petgraph::{stable_graph::DefaultIx, Directed};

///
/// Shorthand for the Graph type to serialize/deserialize.
///
pub type GraphImpl<NodeWeight, EdgeWeight> =
    petgraph::graph::Graph<NodeWeight, EdgeWeight, Directed, DefaultIx>;

///
/// Module/Trait to serialize and deserialize a given Graph to a file.
/// Graph is implemented as petgraph::graph::Graph.
/// Possesses different implementations depending on the file format.
///
/// See the Graph trait for the required generic types.
///
/// TODO Maybe we want to look into graph other data formats that JSON?
/// TODO Maybe we don't want to be bound to a Graph type. Might require
/// our own deserializer implementation, however (extra effort).
///
pub trait GraphReadWriter<NodeWeight, EdgeWeight> {
    ///
    /// Serializes a given graph to a file defined by path.
    /// The result tells us whether the operation succeeded or not.
    ///  
    fn serialize_graph(
        &self,
        path: &str,
        graph: &GraphImpl<NodeWeight, EdgeWeight>,
    ) -> Result<(), io::Error>;

    ///
    /// Deserializes a graph stored in the given file.
    /// The result tells us whether the operation succeeded or not.
    ///
    fn deserialize_graph(
        &self,
        path: &str,
    ) -> Result<Box<GraphImpl<NodeWeight, EdgeWeight>>, io::Error>;
}
