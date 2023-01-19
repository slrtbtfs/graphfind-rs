use std::hash::Hash;

///
/// Graph is a generic trait specifying the functionality that must be implemented by Graph storage backends used for Querying.
///
pub trait Graph<NodeWeight, EdgeWeight> {
    ///
    /// NodeRef is the associated type for node references.
    ///
    /// It implements the Eq and PartialOrd traits (compare references),
    /// Hash (allows insertion of Node Weights and their references into a Table),
    /// and Copy (allows use of references in function parameters).
    ///
    type NodeRef: Copy + Eq + Hash + PartialOrd;
    ///
    /// EdgeRef is the associated type for edge references.
    /// It implements the same traits as NodeRef.
    ///
    type EdgeRef: Copy + Eq + Hash + PartialOrd;
    ///
    /// Checks if the edges of this graph are directed.
    ///
    fn is_directed(&self) -> bool;

    ///
    /// Checks if the given edge is directed.
    ///
    fn is_directed_edge(&self, edge: Self::EdgeRef) -> bool;

    type AdjacentEdgesIterator<'a>: Iterator<Item = Self::EdgeRef>
    where
        Self: 'a;
    ///
    /// Gets a readonly handle of all adjacent edges of a node.
    /// For directed graphs, this includes all incoming and outgoing
    /// edges.
    ///
    fn adjacent_edges(&self, node: Self::NodeRef) -> Self::AdjacentEdgesIterator<'_>;
    type IncomingEdgesIterator<'a>: Iterator<Item = Self::EdgeRef>
    where
        Self: 'a;
    ///
    /// Gets a readonly handle of all incoming edges of a node.
    /// For undirected graphs this is equivalent to calling `adjacent_edges`.
    ///
    fn incoming_edges(&self, node: Self::NodeRef) -> Self::IncomingEdgesIterator<'_>;
    type OutgoingEdgesIterator<'a>: Iterator<Item = Self::EdgeRef>
    where
        Self: 'a;
    ///
    /// Gets a readonly handle of all outgoing edges of a node.
    /// For undirected graphs this is equivalent to calling `adjacent_edges`.
    ///
    fn outgoing_edges(&self, node: Self::NodeRef) -> Self::OutgoingEdgesIterator<'_>;

    ///
    /// Gets a readonly handle of the nodes an edge connects.
    /// If the edge is directed, the first node is its source, and the second node its destination.
    ///
    fn adjacent_nodes(&self, edge: Self::EdgeRef) -> (Self::NodeRef, Self::NodeRef);

    type OutgoingNodesIterator<'a>: Iterator<Item = Self::NodeRef> + 'a
    where
        Self: 'a;
    ///
    /// Gets a read-only handle of the nodes of `ǹode` who are its direct successors.
    ///
    fn outgoing_nodes(&self, node: Self::NodeRef) -> Self::OutgoingNodesIterator<'_>;

    type IncomingNodesIterator<'a>: Iterator<Item = Self::NodeRef> + 'a
    where
        Self: 'a;
    ///
    /// Gets a read-only handle of the nodes of `ǹode` who are its direct successors.
    ///
    fn incoming_nodes(&self, node: Self::NodeRef) -> Self::IncomingNodesIterator<'_>;

    ///
    /// Retrieve weight from a node reference.
    ///
    fn node_weight(&self, node: Self::NodeRef) -> &NodeWeight;

    ///
    /// Retrieve weight from an edge reference.
    ///
    fn edge_weight(&self, edge: Self::EdgeRef) -> &EdgeWeight;

    type NodeWeightsIterator<'a>: Iterator<Item = &'a NodeWeight>
    where
        Self: 'a,
        NodeWeight: 'a;
    ///
    /// Returns an Iterator over all node weights.
    ///
    fn node_weights(&self) -> Self::NodeWeightsIterator<'_>;

    type EdgeWeightsIterator<'a>: Iterator<Item = &'a EdgeWeight>
    where
        Self: 'a,
        EdgeWeight: 'a;
    ///
    /// Returns an Iterator over all edge weights.
    ///
    fn edge_weights(&self) -> Self::EdgeWeightsIterator<'_>;

    type NodesIterator<'a>: Iterator<Item = Self::NodeRef>
    where
        Self: 'a;

    ///
    /// Returns an Iterator over all nodes by their references.
    ///
    fn nodes(&self) -> Self::NodesIterator<'_>;

    type EdgesIterator<'a>: Iterator<Item = Self::EdgeRef>
    where
        Self: 'a;
    ///
    /// Returns an Iterator over all edges by their references.
    ///
    fn edges(&self) -> Self::EdgesIterator<'_>;
}
