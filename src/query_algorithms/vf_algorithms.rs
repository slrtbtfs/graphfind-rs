use std::collections::HashMap;

use crate::{graph::Graph, query::PatternGraph};

///
/// Implements an subgraph isomorphism algorithm based on the papers
/// "A (Sub)Graph Isomorphism Algorithm for Matching Large Graphs"
/// by Cordella, Foggia, Sansone, and Vento.
///
/// The paper referenced above calls this algorithm VF2.
///

///
/// VfState defines the data structures as defined in Subsection 2.4.
///
struct VfState<N1RefType, N2RefType> {
    ///
    /// size contains number of nodes to match.
    ///
    size: usize,
    ///
    /// core1 maps a node from the graph G_1 to a matched node in G_2.
    ///
    core1: HashMap<N1RefType, N2RefType>,
}

///
/// Implementation of VfState.
///
impl<N1Ref, N2Ref> VfState<N1Ref, N2Ref> {
    ///
    /// Creates a new VfState for the given pattern graph and base graph.
    ///
    /// ## Input:
    /// 1. `pat_graph`, a PatternGraph with N1RefType node references.
    /// 2. `base_graph`, any Graph with N2FerType node references.
    ///
    /// ## Output:
    /// A VfState struct.
    ///
    fn init<
        NodeWeight,
        EdgeWeight,
        NodeMatcher,
        EdgeMatcher,
        N1RefType,
        N2RefType,
        PatternGraphType,
        BaseGraphType,
    >(
        pat_graph: &PatternGraphType,
        base_graph: &BaseGraphType,
    ) -> VfState<N1RefType, N2RefType>
    where
        PatternGraphType:
            PatternGraph<NodeWeight, EdgeWeight, NodeMatcher, EdgeMatcher, NodeRef = N1RefType>,
        BaseGraphType: Graph<NodeWeight, EdgeWeight, NodeRef = N2RefType>,
        NodeMatcher: Fn(NodeWeight) -> bool,
        EdgeMatcher: Fn(EdgeWeight) -> bool,
    {
        VfState {
            size: pat_graph.nodes().count(),
            core1: HashMap::new(),
        }
    }
}
