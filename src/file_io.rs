use std::io;

/**
 * Module/Trait to serialize and deserialize a given Graph to a file.
 * 
 * See the Graph trait for the required generic types.
 * 
 * TODO Maybe we want to look into graph other data formats that JSON?
 */

pub trait FileIO<NodeWeight, EdgeWeight, NodeRef, EdgeRef> {
    /**
     * Serializes a given graph to a file defined by path.
     * The result tells us whether the operation succeeded or not.
     */  
    fn serialize_graph(&self, path: &str) -> Result<(), io::Error>;

    /**
     * Deserializes a graph stored in the given file.
     * The result tells us whether the operation succeeded or not.
     */  
    fn deserialize_graph(path: &str) -> Result<Box<Self>, io::Error>;
}