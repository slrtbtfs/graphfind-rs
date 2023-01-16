use std::{collections::HashMap, hash::Hash};

use crate::{
    graph::Graph,
    graph_backends::adj_graphs::AdjGraph,
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
struct VfState<
    'a,
    NodeWeight,
    EdgeWeight,
    NRef,
    ERef,
    N2Ref,
    P: PatternGraph<NodeWeight, EdgeWeight, NodeRef = NRef, EdgeRef = ERef>,
    B: Graph<NodeWeight, EdgeWeight, NodeRef = N2Ref>,
> {
    ///
    /// Reference to the pattern graph.
    ///
    pattern_graph: &'a P,
    ///
    /// Reference to the base graph.
    ///
    base_graph: &'a B,
    ///
    /// Vec of found graphs we can return.
    ///
    results: Vec<AdjGraph<NodeWeight, EdgeWeight, NRef, ERef>>,

    ///
    /// Matching of nodes in `pattern_graph` to suitable nodes in `base_graph`.
    /// `core_1[n] = m` says that the node `n` could be matched to node `m`.
    ///
    core_1: HashMap<NRef, N2Ref>,
    ///
    /// Reverse matching for core_2.
    ///
    core_2: HashMap<N2Ref, NRef>,
}

///
/// Implementation of VfState. This contains the actual parts related to subgraph isomorphism.
///
impl<'a, NodeWeight, EdgeWeight, NRef, ERef, N2Ref, P, B>
    VfState<'a, NodeWeight, EdgeWeight, NRef, ERef, N2Ref, P, B>
where
    NRef: Copy + Eq + Hash,
    N2Ref: Copy + Eq + Hash,
    P: PatternGraph<NodeWeight, EdgeWeight, NodeRef = NRef, EdgeRef = ERef>,
    B: Graph<NodeWeight, EdgeWeight, NodeRef = N2Ref>,
{
    ///
    /// Creates a new VfState for the given pattern graph and base graph.
    /// Initialized for each base_graph instance, to use its specific indices.
    ///
    /// ## Input:
    /// 1. `pattern_graph`, a PatternGraph with NRef node references.
    /// 2. `base_graph`, any Graph with N2FerType node references.
    ///
    /// ## Output:
    /// A VfState struct.
    ///
    fn new<E2RefType, PatternGraphType, BaseGraphType>(
        pattern_graph: &'a PatternGraphType,
        base_graph: &'a BaseGraphType,
    ) -> VfState<'a, NodeWeight, EdgeWeight, NRef, ERef, N2Ref, PatternGraphType, BaseGraphType>
    where
        PatternGraphType: PatternGraph<NodeWeight, EdgeWeight, NodeRef = NRef, EdgeRef = ERef>,
        BaseGraphType: Graph<NodeWeight, EdgeWeight, NodeRef = N2Ref, EdgeRef = E2RefType>,
    {
        VfState {
            pattern_graph,
            base_graph,
            results: vec![],
            core_1: HashMap::new(),
            core_2: HashMap::new(),
        }
    }

    ///
    /// Returns a tuple (N1, N2) of node references.
    /// N1 contains unmatched nodes within the pattern graph, and N2 unmatched nodes within the base graph.
    ///
    fn find_unmatched_nodes(&'a self) -> (Vec<NRef>, Vec<N2Ref>) {
        let n1: Vec<_> = self
            .pattern_graph
            .nodes()
            .filter(|n| !self.core_1.contains_key(n))
            .collect();
        let n2: Vec<_> = self
            .base_graph
            .nodes()
            .filter(|n| !self.core_2.contains_key(n))
            .collect();

        (n1, n2)
    }

    ///
    /// Matches node n to node m, where n is from the pattern,
    /// and m is from the base graph.
    ///
    fn assign(&mut self, n: NRef, m: N2Ref) {
        self.core_1.insert(n, m);
        self.core_2.insert(m, n);
    }

    ///
    /// Undoes the matching between nodes n and m.
    ///
    fn unassign(&mut self, n: &NRef, m: &N2Ref) {
        self.core_1.remove(n);
        self.core_2.remove(m);
    }

    ///
    /// Produces a new AdjGraph for the current graph state.
    ///
    fn produce_graph(&mut self) {
        let nodes: Vec<NRef> = self.core_1.keys().cloned().collect();
        let result: AdjGraph<NodeWeight, EdgeWeight, NRef, ERef> = AdjGraph::new(nodes);
        self.results.push(result);
    }

    ///
    /// Looks up subgraphs and puts them into results.
    ///
    fn find_subgraphs(&mut self, depth: usize) {
        // Full match may now be added.
        if depth == self.pattern_graph.count_nodes() {
            self.produce_graph();
        } else {
            // Find unmatched nodes.
            let (nodes_1, nodes_2) = self.find_unmatched_nodes();
            for n in nodes_1 {
                for m in &nodes_2 {
                    self.assign(n, *m);
                    self.find_subgraphs(depth + 1);
                    self.unassign(&n, m);
                }
            }
        }
    }

    fn get_results(&self) -> &Vec<AdjGraph<NodeWeight, EdgeWeight, NRef, ERef>> {
        &self.results
    }
}

///
/// The VfAlgorithm struct that is accessible to the user.
///
pub struct VfAlgorithm {}

impl SubgraphAlgorithm for VfAlgorithm {
    ///
    /// Wrapper for VfState calls. Call this method to find graphs using a _very_ specialized algorithm
    /// that is only correct for finding single nodes, without types.
    ///
    fn find_subgraphs<'a, NodeWeight, EdgeWeight, NRef, N2Ref, ERef, E2RefType, P, B>(
        pattern_graph: &'a P,
        base_graph: &'a B,
    ) -> Vec<AdjGraph<NodeWeight, EdgeWeight, NRef, ERef>>
    where
        NRef: Copy + Eq + Hash,
        N2Ref: Copy + Eq + Hash,
        P: PatternGraph<NodeWeight, EdgeWeight, NodeRef = NRef, EdgeRef = ERef>,
        B: Graph<NodeWeight, EdgeWeight, NodeRef = N2Ref, EdgeRef = E2RefType>,
    {
        if pattern_graph.is_empty_pattern() {
            return vec![];
        }
        let mut solver = VfState::<NodeWeight, EdgeWeight, NRef, ERef, N2Ref, P, B>::new(
            pattern_graph,
            base_graph,
        );
        solver.find_subgraphs(0);
        vec![]
    }
}
