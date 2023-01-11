use std::collections::HashMap;

use crate::{
    graph::Graph,
    query::{PatternGraph, SubgraphAlgorithm},
};

///
/// Implements an subgraph isomorphism algorithm based on the papers
/// "A (Sub)Graph Isomorphism Algorithm for Matching Large Graphs"
/// by Cordella, Foggia, Sansone, and Vento, published in 2004.
///
/// The paper referenced above calls this algorithm VF2.
///

///
/// VfState defines the required data structures as defined in Subsection 2.4
/// of the 2004 paper.
///
struct VfState<N1RefType, N2RefType> {
    ///
    /// size contains the number of nodes to match.
    ///
    size: usize,
    ///
    /// core1 maps a node from the graph G_1 to the matched node in G_2.
    /// From core1, we can construct a found graph, and find candidates for
    /// what nodes in the pattern we want to match next.
    ///
    core1: HashMap<N1RefType, N2RefType>,
    ///
    /// core2 maps the matched node of G_2 to its counterpart in G_1.
    ///
    core2: HashMap<N2RefType, N1RefType>,
}

///
/// Implementation of VfState.
///
impl<N1Ref, N2Ref> VfState<N1Ref, N2Ref> {
    ///
    /// Creates a new VfState for the given pattern graph and base graph.
    /// Initialized for each base_graph instance, to use its specific indices.
    ///
    /// ## Input:
    /// 1. `pattern_graph`, a PatternGraph with N1RefType node references.
    /// 2. `base_graph`, any Graph with N2FerType node references.
    ///
    /// ## Output:
    /// A VfState struct with size set, and core1 and core2 initialized.
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
        pattern_graph: &PatternGraphType,
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
            size: pattern_graph.nodes().count(),
            core1: HashMap::new(),
            core2: HashMap::new(),
        }
    }

    ///
    /// Runs the main VF2 algorithm, based on recursive depth-first search.
    ///
    /// ### Parameters:
    /// d, the current search depth.
    ///
    fn match_graphs_vf2(&mut self, d: usize) {
        // When the matcher reaches d = size, add the next graph.
        if d == self.size {
            println!("Found a Graph.")
        } else {
            self.match_graphs_vf2(d + 1);
        }
    }
}

struct VfAlgorithm {}

impl SubgraphAlgorithm for VfAlgorithm {
    ///
    /// Wrapper for VfState calls. Currently returns an empty vector.
    ///
    fn find_subgraphs<
        NodeWeight,
        EdgeWeight,
        NodeMatcher,
        EdgeMatcher,
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
        NodeMatcher: Fn(NodeWeight) -> bool,
        EdgeMatcher: Fn(EdgeWeight) -> bool,
        PatternGraphType:
            PatternGraph<NodeWeight, EdgeWeight, NodeMatcher, EdgeMatcher, NodeRef = N1RefType>,
        BaseGraphType: Graph<NodeWeight, EdgeWeight, NodeRef = N2RefType>,
        ResultGraphType: Graph<NodeWeight, EdgeWeight>,
    {
        let mut m_state = VfState::<N1RefType, N2RefType>::init(pattern_graph, base_graph);
        m_state.match_graphs_vf2(0);
        vec![]
    }
}
