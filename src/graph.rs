/**
 * Graph is a generic trait specifying the functionality that must be implemented by Graph storage backends used for Querying.
 */

pub trait Graph<NodeWeight,EdgeWeight,NodeRef,EdgeRef> {

    /**
     * Gets a readonly handle of all adjacent edges of a node.
     */
    fn adjacent_edges(&self,  node: &NodeRef) -> Box<dyn Iterator<Item = EdgeRef> + '_>;

    /**
     * Gets a readonly handle of the nodes an edge connects.
     */
    fn adjacent_nodes(&self,  node: &EdgeRef) -> (NodeRef,NodeRef);

    /**
     * Retrieve weight from a node reference.
     */
    fn node_weight(&self, node: &NodeRef) -> NodeWeight;

    /**
     * Retrieve weight from an edge reference.
     */
    fn edge_weight(&self, edge: &EdgeRef) -> EdgeWeight;

    /**
     * Returns an Iterator over all node weights.
     */
    fn node_weights(&self) -> Box<dyn Iterator<Item = &NodeWeight> + '_>;
    /**
     * Returns an Iterator over all edge weights.
     */
    fn edge_weights(&self) -> Box<dyn Iterator<Item = &EdgeWeight> + '_>;
}


