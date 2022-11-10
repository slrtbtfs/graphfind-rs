use crate::graph::Graph;

/**
 * The PrintGraph trait allows a given Graph to be printed to the GraphViz format.
 */
pub trait PrintToGraphVizDot<NodeWeight, EdgeWeight, NodeRef, EdgeRef> :
    Graph<NodeWeight, EdgeWeight, NodeRef, EdgeRef> {
    /**
     * Prints the given graph. This function returns a String. 
     */
    fn print(&self) -> String;
}