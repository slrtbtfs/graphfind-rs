use std::hash::Hash;

use petgraph::visit::EdgeRef;

use crate::graph::Graph;

///
/// AdjGraph defines a custom directed graph based on a adjacency list.
///
pub struct AdjGraph<NWeight, EWeight, NRef, ERef> {
    ///
    /// List of node references.
    ///
    node_refs: Vec<NRef>,
    ///
    /// List of edge references.
    ///
    edges: Vec<ERef>,

    ///
    /// List of Node Weights.
    ///
    weight1: Vec<NWeight>,
    weight2: Vec<EWeight>,
}

impl<NodeWeight, EdgeWeight, NRef, ERef> Graph<NodeWeight, EdgeWeight>
    for AdjGraph<NodeWeight, EdgeWeight, NRef, ERef>
where
    NRef: Copy + Eq + PartialOrd + Hash,
    ERef: Copy + Eq + PartialOrd + Hash,
{
    type NodeRef = NRef;
    type EdgeRef = ERef;

    type NodesIterator<'a> = impl Iterator<Item = Self::NodeRef> + 'a
    where
        Self: 'a;

    fn is_directed(&self) -> bool {
        true
    }

    fn is_directed_edge(&self, edge: Self::EdgeRef) -> bool {
        todo!()
    }

    type AdjacentEdgesIterator<'a> = impl Iterator<Item = Self::EdgeRef> + 'a
    where
        Self: 'a;

    fn adjacent_edges(&self, node: Self::NodeRef) -> Self::AdjacentEdgesIterator<'_> {
        self.edges.iter().copied()
    }

    type IncomingEdgesIterator<'a> = impl Iterator<Item = Self::EdgeRef> + 'a
    where
        Self: 'a;

    fn incoming_edges(&self, node: Self::NodeRef) -> Self::IncomingEdgesIterator<'_> {
        self.edges.iter().copied()
    }

    type OutgoingEdgesIterator<'a> = impl Iterator<Item = Self::EdgeRef> + 'a
    where
        Self: 'a;

    fn outgoing_edges(&self, node: Self::NodeRef) -> Self::OutgoingEdgesIterator<'_> {
        self.edges.iter().copied()
    }

    fn adjacent_nodes(&self, edge: Self::EdgeRef) -> (Self::NodeRef, Self::NodeRef) {
        todo!()
    }

    fn node_weight(&self, node: Self::NodeRef) -> &NodeWeight {
        todo!()
    }

    fn edge_weight(&self, edge: Self::EdgeRef) -> &EdgeWeight {
        todo!()
    }

    type NodeWeightsIterator<'a> = impl Iterator<Item = &'a NodeWeight> + 'a
    where
        Self: 'a,
        NodeWeight: 'a;

    fn node_weights(&self) -> Self::NodeWeightsIterator<'_> {
        self.weight1.iter()
    }

    type EdgeWeightsIterator<'a> = impl Iterator<Item = &'a EdgeWeight> + 'a
    where
        Self: 'a,
        EdgeWeight: 'a;

    fn edge_weights(&self) -> Self::EdgeWeightsIterator<'_> {
        self.weight2.iter()
    }

    fn nodes(&self) -> Self::NodesIterator<'_> {
        self.node_refs.iter().copied()
    }

    type EdgesIterator<'a> = impl Iterator<Item = Self::EdgeRef> + 'a
    where
        Self: 'a;

    fn edges(&self) -> Self::EdgesIterator<'_> {
        self.edges.iter().copied()
    }
}
