use std::{collections::HashMap, hash::Hash};

use crate::graph::Graph;

///
/// AdjGraph defines a custom directed graph based on a adjacency list.
///
pub struct AdjGraph<'a, NWeight, EWeight, NRef, ERef> {
    ///
    /// Index of node references and the stored associated nodes.
    ///
    nodes: HashMap<NRef, &'a NWeight>,
    ///
    /// List of edge references, and their associated nodes.
    ///
    edge_list: HashMap<ERef, (NRef, NRef)>,
    ///
    /// Maps Nodes to the incoming adjacency lists.
    ///
    incoming_adj_list: HashMap<NRef, Vec<ERef>>,

    ///
    /// List of Edge Weights.
    ///
    weight2: Vec<EWeight>,
}

///
/// Graph-specific constructor.
///
impl<'a, NWeight, EWeight, NRef, ERef> AdjGraph<'a, NWeight, EWeight, NRef, ERef> {
    ///
    /// Produces a new AdjGraph.
    ///
    /// ## Input:
    /// 1. nodes, the node map of the graph.
    /// 2. edges, the edge content list (endpoints).
    ///
    pub fn new(
        nodes: HashMap<NRef, &NWeight>,
        incoming_adj_list: HashMap<NRef, Vec<ERef>>,
        edge_list: HashMap<ERef, (NRef, NRef)>,
    ) -> AdjGraph<NWeight, EWeight, NRef, ERef> {
        AdjGraph {
            nodes,
            edge_list,
            incoming_adj_list,
            weight2: vec![],
        }
    }
}

///
/// Implementation of the Graph trait.
///
impl<'b, NodeWeight, EdgeWeight, NRef, ERef> Graph<NodeWeight, EdgeWeight>
    for AdjGraph<'b, NodeWeight, EdgeWeight, NRef, ERef>
where
    NRef: Copy + Eq + PartialOrd + Hash,
    ERef: Copy + Eq + PartialOrd + Hash,
{
    type NodeRef = NRef;
    type EdgeRef = ERef;

    type NodesIterator<'a> = impl Iterator<Item = Self::NodeRef> + 'a
    where
        Self: 'a;

    ///
    /// All AdjGraphs are directed.
    ///
    fn is_directed(&self) -> bool {
        true
    }

    ///
    /// Checks if edge is directed (i.e. belongs to the saved edge references).
    ///
    fn is_directed_edge(&self, edge: Self::EdgeRef) -> bool {
        assert!(self.edge_list.contains_key(&edge));
        true
    }

    type AdjacentEdgesIterator<'a> = impl Iterator<Item = Self::EdgeRef> + 'a
    where
        Self: 'a;

    fn adjacent_edges(&self, node: Self::NodeRef) -> Self::AdjacentEdgesIterator<'_> {
        self.edge_list.keys().copied()
    }

    type IncomingEdgesIterator<'a> = impl Iterator<Item = Self::EdgeRef> + 'a
    where
        Self: 'a;

    ///
    /// Provides an iterator for the edge list of node.
    ///
    fn incoming_edges(&self, node: Self::NodeRef) -> Self::IncomingEdgesIterator<'_> {
        self.incoming_adj_list[&node].iter().copied()
    }

    type OutgoingEdgesIterator<'a> = impl Iterator<Item = Self::EdgeRef> + 'a
    where
        Self: 'a;

    fn outgoing_edges(&self, node: Self::NodeRef) -> Self::OutgoingEdgesIterator<'_> {
        self.edge_list.keys().copied()
    }

    ///
    /// Returns the value (node pair) stored for edge in edges.
    ///
    fn adjacent_nodes(&self, edge: Self::EdgeRef) -> (Self::NodeRef, Self::NodeRef) {
        // let x = self.ed
        *self.edge_list.get(&edge).unwrap()
    }

    fn node_weight(&self, node: Self::NodeRef) -> &NodeWeight {
        self.nodes.get(&node).unwrap()
    }

    fn edge_weight(&self, edge: Self::EdgeRef) -> &EdgeWeight {
        todo!()
    }

    type NodeWeightsIterator<'a> = impl Iterator<Item = &'a NodeWeight> + 'a
    where
        Self: 'a,
        NodeWeight: 'a;

    ///
    /// Returns an iterator over the nodes values, i.e. their keys.
    ///
    fn node_weights(&self) -> Self::NodeWeightsIterator<'_> {
        self.nodes.values().copied()
    }

    type EdgeWeightsIterator<'a> = impl Iterator<Item = &'a EdgeWeight> + 'a
    where
        Self: 'a,
        EdgeWeight: 'a;

    fn edge_weights(&self) -> Self::EdgeWeightsIterator<'_> {
        self.weight2.iter()
    }

    ///
    /// Returns an Iterator over the nodes keys.
    ///
    fn nodes(&self) -> Self::NodesIterator<'_> {
        self.nodes.keys().copied()
    }

    type EdgesIterator<'a> = impl Iterator<Item = Self::EdgeRef> + 'a
    where
        Self: 'a;

    ///
    /// Returns an iterator over the edges keys.
    ///
    fn edges(&self) -> Self::EdgesIterator<'_> {
        self.edge_list.keys().copied()
    }
}
