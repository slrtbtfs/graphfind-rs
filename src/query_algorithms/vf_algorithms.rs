use std::collections::HashMap;

use crate::{
    graph::Graph,
    query::{PatternGraph, SubgraphAlgorithm},
};

///
/// Implements an subgraph isomorphism algorithm based on the papers
/// "A (Sub)Graph Isomorphism Algorithm for Matching Large Graphs"
/// by Cordella, Foggia, Sansone, and Vento, published in 2004.
/// The paper referenced above calls this algorithm VF2.
///

///
/// VfState defines the required data structures as defined in Subsection 2.4
/// of the 2004 paper.
///
struct VfState {}

///
/// Implementation of VfState. This contains the actual parts related
/// to subgraph isomorphism.
///
impl VfState {
    ///
    /// Creates a new VfState for the given pattern graph and base graph.
    /// Initialized for each base_graph instance, to use its specific indices.
    ///
    /// ## Input:
    /// 1. `pattern_graph`, a PatternGraph with N1RefType node references.
    /// 2. `base_graph`, any Graph with N2FerType node references.
    ///
    /// ## Output:
    /// A VfState struct.
    ///
    fn new<
        NodeWeight,
        EdgeWeight,
        NodeMatcher,
        EdgeMatcher,
        N1RefType,
        N2RefType,
        PatternGraphType,
        BaseGraphType,
    >(
        pattern_graph: &PatternGraphType,
        base_graph: &BaseGraphType,
    ) -> VfState
    where
        PatternGraphType: PatternGraph<NodeWeight, EdgeWeight, NodeRef = N1RefType>,
        BaseGraphType: Graph<NodeWeight, EdgeWeight, NodeRef = N2RefType>,
        NodeMatcher: Fn(NodeWeight) -> bool,
        EdgeMatcher: Fn(EdgeWeight) -> bool,
    {
        VfState {}
    }
}

///
/// The VfAlgorithm struct that is accessible to the user.
///
pub struct VfAlgorithm {}

impl SubgraphAlgorithm for VfAlgorithm {
    ///
    /// Wrapper for VfState calls. Call this method to find graphs using a _very_ specialized algorithm
    /// that only works on Star Graphs.
    ///
    /// Currently returns an empty vector.
    ///
    fn find_subgraphs<
        NodeWeight,
        EdgeWeight,
        N1RefType,
        N2RefType,
        E1RefType,
        E2RefType,
        PatternGraphType,
        BaseGraphType,
        ResultGraphType,
    >(
        pattern_graph: &PatternGraphType,
        base_graph: &BaseGraphType,
    ) -> Vec<ResultGraphType>
    where
        PatternGraphType:
            PatternGraph<NodeWeight, EdgeWeight, NodeRef = N1RefType, EdgeRef = E1RefType>,
        BaseGraphType: Graph<NodeWeight, EdgeWeight, NodeRef = N2RefType, EdgeRef = E2RefType>,
        ResultGraphType: Graph<NodeWeight, EdgeWeight, NodeRef = N1RefType, EdgeRef = E1RefType>,
    {
        vec![]
    }
}
