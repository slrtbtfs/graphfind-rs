use std::hash::Hash;

use crate::{filter_map::FilterMap, graph::Graph};

use super::{PatternElement, PatternGraph};
/// The SubgraphAlgorithm trait specifies any algorithm that can solve the subgraph isomorphism problem.
/// Solving this problem lies at the core of graph pattern matching.
pub trait SubgraphAlgorithm<
    'a,
    NodeWeight,
    EdgeWeight,
    NRef,
    ERef,
    N2Ref,
    E2Ref,
    PatternGraphType,
    BaseGraphType,
> where
    NRef: Copy + Eq + Hash,
    N2Ref: Copy + Eq + Hash,
    PatternGraphType: PatternGraph<NodeWeight, EdgeWeight, NodeRef = NRef, EdgeRef = ERef>,
    BaseGraphType: Graph<NodeWeight, EdgeWeight, NodeRef = N2Ref, EdgeRef = E2Ref>,
{
    /// Creates and evaluates a query to find all subgraphs of `base_graph` that match `pattern_graph`.
    ///
    /// # Input:
    /// Two graphs, `base_graph` (any Graph instance), and `pattern_graph` (a PatternGraph instance).
    /// `base_graph` and `pattern_graph` share both node and edge types (NodeWeight/EdgeWeight).
    ///
    /// `pattern_graph` uses NRef and ERef for references over nodes and edges.
    /// `base_graph` uses N2Ref and E2Ref respectively.
    ///
    /// Both `pattern_graph` and `base_graph` currently have the same lifetime 'a.
    ///
    /// # Output:
    /// A reference to a vector of MatchedGraph, whose nodes and edges have NodeWeight/EdgeWeight types,
    /// and its references NRef/ERef. We want to access matched elements of
    /// the base graph by references we set in the pattern graph.
    ///
    /// An implementation find_subgraphs should guarantee set semantics, so that every found
    /// graph pattern occurs only once.
    ///
    /// If `pattern_graph` is an empty graph without nodes (or edges), or if no subgraph of `base_graph`
    /// can be matched to it, then we return an empty vector.
    ///
    /// # Panics:
    /// `base_graph` is a directed graph, and `pattern_graph` is not, or vice versa.
    fn eval(
        pattern_graph: &'a PatternGraphType,
        base_graph: &'a BaseGraphType,
    ) -> Vec<MatchedGraph<'a, NodeWeight, EdgeWeight, PatternGraphType>>;
}
/// Type definition of MatchedGraph.
pub type MatchedGraph<'a, N, E, P> =
    FilterMap<'a, PatternElement<N>, PatternElement<E>, &'a N, &'a E, P>;
