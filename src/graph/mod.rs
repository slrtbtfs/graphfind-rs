use std::{fmt::Debug, hash::Hash};

// Serializing graphs to files.
pub mod file_io;
// Printing graph visualizations in graphviz dot format.
pub mod print;

///
/// Graph is a generic trait specifying the functionality that must be implemented by Graph storage backends used for Querying.
///
pub trait Graph<NodeWeight, EdgeWeight> {
    ///
    /// NodeRef is the associated type for node references.
    ///
    /// It implements the Ord trait (compare references),
    /// Eq + Hash (allows insertion of Node Weights and their references into a Table),
    /// Copy (allows use of references in function parameters),
    /// and Debug (simplifies debugging).
    ///
    type NodeRef: Copy + Eq + Hash + Ord + Debug;
    ///
    /// EdgeRef is the associated type for edge references.
    /// It implements the same traits as NodeRef.
    ///
    type EdgeRef: Copy + Eq + Hash + Ord + Debug;
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

    ///
    /// Tests if the given graph is empty.
    ///
    fn is_empty_graph(&self) -> bool {
        self.count_nodes() == 0
    }

    ///
    /// Returns the number of nodes in this graph.
    ///
    fn count_nodes(&self) -> usize;
    ///
    /// Returns the number of edges in this graph.
    ///
    fn count_edges(&self) -> usize;
}

///
/// Convenience Method to directly access the predecessor nodes of node `n` in Graph `g`.
/// Returns an iterator over these nodes.
///
pub fn incoming_nodes<'a, G, N, NW: 'a, EW: 'a>(g: &'a G, n: N) -> impl Iterator<Item = N> + 'a
where
    G: Graph<NW, EW, NodeRef = N>,
{
    g.incoming_edges(n).map(|e| g.adjacent_nodes(e).1)
}

///
/// Convenience Method to directly access the successor nodes of node `n` in Graph `g`.
/// Returns an iterator over these nodes.
///
pub fn outgoing_nodes<'a, G, N, NW: 'a, EW: 'a>(g: &'a G, n: N) -> impl Iterator<Item = N> + 'a
where
    G: Graph<NW, EW, NodeRef = N>,
{
    g.outgoing_edges(n).map(|e| g.adjacent_nodes(e).1)
}
