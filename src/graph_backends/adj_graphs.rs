use std::{collections::HashMap, hash::Hash};

use crate::{graph::Graph, query::PatternGraph};

///
/// AdjGraph defines a custom directed graph based on a adjacency list.
///
pub struct AdjGraph<'a, NWeight, EWeight, NRef, ERef, P>
where
    P: PatternGraph<NWeight, EWeight, NodeRef = NRef, EdgeRef = ERef>,
{
    ///
    /// Index of node references and the stored associated nodes.
    ///
    nodes: HashMap<NRef, &'a NWeight>,
    ///
    /// Pattern Graph structure that we reuse.
    ///
    pattern: &'a P,

    ///
    /// List of Edge Weights.
    ///
    weight2: Vec<EWeight>,
}

///
/// Graph-specific constructor.
///
impl<'a, NWeight, EWeight, NRef, ERef, P> AdjGraph<'a, NWeight, EWeight, NRef, ERef, P>
where
    P: PatternGraph<NWeight, EWeight, NodeRef = NRef, EdgeRef = ERef>,
{
    ///
    /// Produces a new AdjGraph.
    ///
    /// ## Input:
    /// 1. nodes, the node map of the graph.
    /// 2. pattern, the underlying structure of the pattern graph.
    ///
    pub fn new(
        nodes: HashMap<NRef, &'a NWeight>,
        pattern: &'a P,
    ) -> AdjGraph<'a, NWeight, EWeight, NRef, ERef, P> {
        AdjGraph {
            nodes,
            pattern,
            weight2: vec![],
        }
    }
}

///
/// Implementation of the Graph trait.
///
impl<'b, NodeWeight, EdgeWeight, NRef, ERef, P> Graph<NodeWeight, EdgeWeight>
    for AdjGraph<'b, NodeWeight, EdgeWeight, NRef, ERef, P>
where
    NRef: Copy + Eq + PartialOrd + Hash,
    ERef: Copy + Eq + PartialOrd + Hash,
    P: PatternGraph<NodeWeight, EdgeWeight, NodeRef = NRef, EdgeRef = ERef>,
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
        self.pattern.is_directed_edge(edge)
    }

    type AdjacentEdgesIterator<'a> = impl Iterator<Item = Self::EdgeRef> + 'a
    where
        Self: 'a;

    fn adjacent_edges(&self, node: Self::NodeRef) -> Self::AdjacentEdgesIterator<'_> {
        self.pattern.adjacent_edges(node)
    }

    type IncomingEdgesIterator<'a> = impl Iterator<Item = Self::EdgeRef> + 'a
    where
        Self: 'a;

    ///
    /// Provides an iterator for the edge list of node.
    ///
    fn incoming_edges(&self, node: Self::NodeRef) -> Self::IncomingEdgesIterator<'_> {
        self.pattern.incoming_edges(node)
    }

    type OutgoingEdgesIterator<'a> = impl Iterator<Item = Self::EdgeRef> + 'a
    where
        Self: 'a;

    fn outgoing_edges(&self, node: Self::NodeRef) -> Self::OutgoingEdgesIterator<'_> {
        self.pattern.outgoing_edges(node)
    }

    ///
    /// Returns the value (node pair) stored for edge in edges.
    ///
    fn adjacent_nodes(&self, edge: Self::EdgeRef) -> (Self::NodeRef, Self::NodeRef) {
        self.pattern.adjacent_nodes(edge)
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
        self.pattern.edges()
    }
}
