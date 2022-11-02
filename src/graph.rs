/**
 * Graph is a generic trait specifying the functionality that must be implemented by Graph storage backends used for Querying.
 */

pub trait Graph {
    /**
     * Type used for Node Weights.
     */
    type NodeType;
    /**
     * Type used for Edge Weights.
     */
    type EdgeType;

    /**
     * Returns an Iterator over all node weights.
     */
    fn node_weights(&self) -> Box<dyn Iterator<Item = &'_ Self::NodeType> + '_>;
    /**
     * Returns an Iterator over all edge weights.
     */
    fn edge_weights(&self) -> Box<dyn Iterator<Item = &'_ Self::EdgeType> + '_>;
}

/**
 * Example implementation for in memory graphs stored using the petgraph library.
 */
impl<N, E> Graph for petgraph::graph::Graph<N, E> {
    type NodeType = N;

    type EdgeType = E;

    fn node_weights(&self) -> Box<dyn Iterator<Item = &'_ Self::NodeType> + '_> {
        Box::new(self.node_weights())
    }

    fn edge_weights(&self) -> Box<dyn Iterator<Item = &'_ Self::EdgeType> + '_> {
        Box::new(self.edge_weights())
    }
}
