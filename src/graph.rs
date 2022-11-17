/**
 * Graph is a generic trait specifying the functionality that must be implemented by Graph storage backends used for Querying.
 *
 * Lifetimes:
 * + Graph lifetime is `'g`.
 * + query lifetime is `'q`.
 */

pub trait Graph<NodeWeight, EdgeWeight, NodeRef, EdgeRef> {
    /**
     * Checks if the edges of this graph are directed.
     */
    fn is_directed(&self) -> bool;

    /**
     * Checks if the given edge is directed.
     */
    fn is_directed_edge(&self, edge: EdgeRef) -> Option<bool>;

    /**
     * Gets a readonly handle of all adjacent edges of a node.
     * For directed graphs this includes all incoming and outgoing
     * edges.
     */
    fn adjacent_edges<'a>(&'a self, node: &'a NodeRef) -> Box<dyn Iterator<Item = EdgeRef> + 'a>
    where
        EdgeRef: 'a;
    /**
     * Gets a readonly handle of all incoming edges of a node.
     * For undirected graphs this is equivalent to calling `adjacent_edges`.
     */
    fn incoming_edges<'a>(&'a self, node: &'a NodeRef) -> Box<dyn Iterator<Item = EdgeRef> + 'a>
    where
        EdgeRef: 'a;
    /**
     * Gets a readonly handle of all outgoing edges of a node.
     * For undirected graphs this is equivalent to calling `adjacent_edges`.
     */
    fn outgoing_edges<'a>(&'a self, node: &'a NodeRef) -> Box<dyn Iterator<Item = EdgeRef> + 'a>
    where
        EdgeRef: 'a;

    /**
     * Checks whether two references refer to the same edge.
     */
    fn do_ref_same_edge(&self, edge1: EdgeRef, edge2: EdgeRef) -> bool;

    /**
     * Checks whether two references refer to the same node.
     */
    fn do_ref_same_node(&self, node1: NodeRef, node2: NodeRef) -> bool;

    /**
     * Gets a readonly handle of the nodes an edge connects.
     * If the edge is directed, the first node is its source, and the second node its destination.
     */
    fn adjacent_nodes(&self, node: EdgeRef) -> Option<(NodeRef, NodeRef)>;

    /**
     * Retrieve weight from a node reference.
     */
    fn node_weight(&self, node: NodeRef) -> Option<&NodeWeight>;

    /**
     * Retrieve weight from an edge reference.
     */
    fn edge_weight(&self, edge: EdgeRef) -> Option<&EdgeWeight>;

    /**
     * Returns an Iterator over all node weights.
     */
    fn node_weights<'a>(&'a self) -> Box<dyn Iterator<Item = &'a NodeWeight> + 'a> {
        let _it = self.nodes().map(|x| self.node_weight(x).unwrap());

        //Box::new(it)
        todo!()
    }

    /**
     * Returns an Iterator over all edge weights.
     */
    fn edge_weights<'a>(&'a self) -> Box<dyn Iterator<Item = &'a EdgeWeight> + 'a> {
        let _it = self.edges().map(|x| self.edge_weight(x).unwrap());
        //Box::new(it)
        todo!()
    }

    /**
     * Returns an Iterator over all nodes.
     */
    fn nodes<'a>(&'a self) -> Box<dyn Iterator<Item = NodeRef> + 'a>
    where
        NodeRef: 'a;

    /**
     * Returns an Iterator over all edges.
     */
    fn edges<'a>(&'a self) -> Box<dyn Iterator<Item = EdgeRef> + 'a>
    where
        EdgeRef: 'a;
}
