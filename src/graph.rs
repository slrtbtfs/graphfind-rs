use petgraph::Direction::Incoming;

/**
 * Graph is a generic trait specifying the functionality that must be implemented by Graph storage backends used for Querying.
 *
 * Lifetimes:
 * + Graph lifetime is `'g`.
 * + query lifetime is `'q`.
 */

pub trait Graph<NodeWeight, EdgeWeight> {
    type NodeRef<'a>;
    type EdgeRef<'a>;
    /**
     * Checks if the edges of this graph are directed.
     */
    fn is_directed(&self) -> bool;

    /**
     * Checks if the given edge is directed.
     */
    fn is_directed_edge(&self, edge: Self::EdgeRef<'_>) -> Option<bool>;

    type AdjacentEdgesIterator<'a>: Iterator<Item = Self::EdgeRef<'a>>
    where
        Self: 'a;
    /**
     * Gets a readonly handle of all adjacent edges of a node.
     * For directed graphs this includes all incoming and outgoing
     * edges.
     */
    fn adjacent_edges(&self, node: &Self::NodeRef<'_>) -> Self::AdjacentEdgesIterator<'_>;
    type IncomingEdgesIterator<'a>: Iterator<Item = Self::EdgeRef<'a>>
    where
        Self: 'a;
    /**
     * Gets a readonly handle of all incoming edges of a node.
     * For undirected graphs this is equivalent to calling `adjacent_edges`.
     */
    fn incoming_edges(&self, node: &Self::NodeRef<'_>) -> Self::IncomingEdgesIterator<'_>;
    type OutgoingEdgesIterator<'a>: Iterator<Item = Self::EdgeRef<'a>>
    where
        Self: 'a;
    /**
     * Gets a readonly handle of all outgoing edges of a node.
     * For undirected graphs this is equivalent to calling `adjacent_edges`.
     */
    fn outgoing_edges(&self, node: &Self::NodeRef<'_>) -> Self::OutgoingEdgesIterator<'_>;

    /**
     * Checks whether two references refer to the same edge.
     */
    fn do_ref_same_edge(&self, edge1: Self::EdgeRef<'_>, edge2: Self::EdgeRef<'_>) -> bool;

    /**
     * Checks whether two references refer to the same node.
     */
    fn do_ref_same_node(&self, node1: Self::NodeRef<'_>, node2: Self::NodeRef<'_>) -> bool;

    /**
     * Gets a readonly handle of the nodes an edge connects.
     * If the edge is directed, the first node is its source, and the second node its destination.
     */
    fn adjacent_nodes(
        &self,
        node: Self::EdgeRef<'_>,
    ) -> Option<(Self::NodeRef<'_>, Self::NodeRef<'_>)>;

    /**
     * Retrieve weight from a node reference.
     */
    fn node_weight(&self, node: Self::NodeRef<'_>) -> Option<&NodeWeight>;

    /**
     * Retrieve weight from an edge reference.
     */
    fn edge_weight(&self, edge: Self::EdgeRef<'_>) -> Option<&EdgeWeight>;

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

    type NodesIterator<'a>: Iterator<Item = Self::NodeRef<'a>>
    where
        Self: 'a;
    /**
     * Returns an Iterator over all nodes.
     */
    fn nodes(&self) -> Self::NodesIterator<'_>;

    type EdgesIterator<'a>: Iterator<Item = Self::EdgeRef<'a>>
    where
        Self: 'a;
    /**
     * Returns an Iterator over all edges.
     */
    fn edges(&self) -> Self::EdgesIterator<'_>;
}
