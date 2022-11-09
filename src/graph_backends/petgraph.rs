use std::borrow::Cow;

use petgraph::graph::NodeIndex;
/// Alias to refer to the underlying graph implementation.
use petgraph::stable_graph::StableGraph as GraphImpl;

/// Allow access to edge references.
use petgraph::stable_graph::EdgeReference;
use petgraph::visit::{IntoEdgeReferences, EdgeRef, IntoNodeIdentifiers};
use petgraph::{stable_graph::DefaultIx, Directed};

use crate::graph::{Graph, Result};
/**
 * Example implementation for in memory graphs stored using the petgraph library.
 */
impl<'g: 'q, 'q, NodeWeight, EdgeWeight>
    Graph<'g, 'q, NodeWeight, EdgeWeight, &'q NodeIndex, EdgeReference<'q, EdgeWeight>>
    for &'g petgraph::stable_graph::StableGraph<NodeWeight, EdgeWeight, Directed, DefaultIx>
{
    /// Returns iterator over the petgraph edges connected to the node.
    fn adjacent_edges(
        &'g self,
        node: &NodeIndex,
    ) -> Result<Box<(dyn Iterator<Item = EdgeReference<'q, EdgeWeight>> + 'q)>> {
        Ok(Box::new(GraphImpl::edges(self, *node)))
    }

    fn adjacent_nodes(
        &'g self,
        edge: EdgeReference<'q, EdgeWeight>,
    ) -> Result<(&'q NodeIndex, &'q NodeIndex)> {
        let idx_pair = (EdgeRef::source(&edge), EdgeRef::target(&edge));
        
        todo!()
    }

    /// Looks up the node weight associated with the node index, and returns it in an Ok enum.
    /// Failing that, returns an Error.
    fn node_weight(&self, node: &NodeIndex) -> Result<&NodeWeight> {
        let found_weight = GraphImpl::node_weight(self, *node);
        found_weight.ok_or(String::from("invalid node reference"))
    }

    fn node_weights(&self) -> Box<dyn Iterator<Item = &NodeWeight> + '_> {
        Box::new(GraphImpl::node_weights(self))
    }

    fn edge_weights(&self) -> Box<dyn Iterator<Item = &EdgeWeight> + '_> {
        Box::new(GraphImpl::edge_weights(self))
    }

    fn is_directed(&self) -> bool {
        GraphImpl::is_directed(&self)
    }

    fn is_directed_edge(&self, edge: EdgeReference<'q, EdgeWeight>) -> Result<bool> {
        Ok(self.is_directed())
    }

    /// Compares two edges by their Edge Index.
    /// 
    /// TODO:
    /// Consider a Variant that checks by Node Indices (dependent on direction)
    /// and Edge Contents (make those comparable -> extend the Graph Trait).
    fn do_ref_same_edge(
        &'g self,
        edge1: EdgeReference<'q, EdgeWeight>,
        edge2: EdgeReference<'q, EdgeWeight>,
    ) -> Result<bool> {
        Ok(EdgeRef::id(&edge1) == EdgeRef::id(&edge2))
    }

    /// Checks if two nodes indices are equal.
    /// 
    /// Consider equality trait when comparing by node contents.
    fn do_ref_same_node(&'g self, node1: &'q NodeIndex, node2: &'q NodeIndex) -> Result<bool> {
        match (
            GraphImpl::node_weight(&self, *node1),
            GraphImpl::node_weight(&self, *node2),
        ) {
            (Some(_), Some(_)) => Ok(*node1 == *node2),
            (_, _) => Err(String::from("Node References invalid in this Context.")),
        }
    }

    /// Extracts the edge weight from a given reference.
    /// Should the given reference be invalid in the context of this graph, return an Error instead.
    fn edge_weight(&'_ self, edge: EdgeReference<'q, EdgeWeight>) -> Result<&EdgeWeight> {
        let n1_idx = EdgeRef::source(&edge);
        let n2_idx = EdgeRef::target(&edge);
        let e_idx = GraphImpl::find_edge(&self, n1_idx, n2_idx);

        GraphImpl::edge_weight(
            &self, e_idx.expect("Edge Reference is invalid in the context of this graph."))
        .ok_or(String::from("Something went wrong."))
    }

    fn nodes(&'g self) -> Box<(dyn Iterator<Item = &'q NodeIndex> + '_)> {
        let it =
            IntoNodeIdentifiers::node_identifiers(&self);
        // return Box::new(it);
        todo!()
    }

    /// Uses IntoEdgeReferences to provide reference to all edges.
    fn edges(&'g self) -> Box<dyn Iterator<Item = EdgeReference<'q, EdgeWeight>> + 'q> {
        Box::new(IntoEdgeReferences::edge_references(&self))
    }
}
