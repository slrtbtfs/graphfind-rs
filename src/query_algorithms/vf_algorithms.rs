use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    vec,
};

use crate::{
    graph::Graph,
    graph_backends::adj_graphs::AdjGraph,
    query::{PatternGraph, SubgraphAlgorithm},
};

///
/// Implements an subgraph isomorphism algorithm based on the papers
/// "A (Sub)Graph Isomorphism Algorithm for Matching Large Graphs"
/// by Cordella, Foggia, Sansone, and Vento, published in 2004,
/// as well as
/// "Performance Evaluation of the VF Graph Matching Algorithm"
/// by the same authors in 1999.
/// The paper referenced above calls this algorithm VF and VF2.
///

///
/// VfState defines the required data structures as defined in Subsection 2.4
/// of the 2004 paper, as well as the algorithms to run them.
///
pub struct VfState<
    'a,
    NodeWeight,
    EdgeWeight,
    NRef,
    ERef,
    N2Ref,
    E2Ref,
    P: PatternGraph<NodeWeight, EdgeWeight, NodeRef = NRef, EdgeRef = ERef>,
    B: Graph<NodeWeight, EdgeWeight, NodeRef = N2Ref, EdgeRef = E2Ref>,
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
    results: Vec<AdjGraph<'a, NodeWeight, EdgeWeight, NRef, ERef>>,

    ///
    /// Matching of nodes in `pattern_graph` to suitable nodes in `base_graph`.
    /// `core_1[n] = m` says that the node `n` could be matched to node `m`.
    ///
    /// We use this map to find possible candidates for next nodes, and to
    /// construct the result graph.
    ///
    core_1: HashMap<NRef, N2Ref>,
    ///
    /// Reverse matching for core_2.
    ///
    core_2: HashMap<N2Ref, NRef>,
    ///
    /// out_1 is a matching between node references from `pattern_graph`,
    /// and the search depth at which they were inserted. We use this mapping
    /// to find possible successor nodes to insert into `core_1`.
    ///
    out_1: HashMap<NRef, usize>,
    ///
    /// Analog matching for outgoing nodes of `base_graph`.
    ///
    out_2: HashMap<N2Ref, usize>,
}

///
/// Implementation of VfState. This contains the actual parts related to subgraph isomorphism.
///
impl<'a, NodeWeight, EdgeWeight, NRef, ERef, N2Ref, E2Ref, P, B>
    VfState<'a, NodeWeight, EdgeWeight, NRef, ERef, N2Ref, E2Ref, P, B>
where
    NRef: Copy + Hash + Ord,
    N2Ref: Copy + Hash + Eq,
    P: PatternGraph<NodeWeight, EdgeWeight, NodeRef = NRef, EdgeRef = ERef>,
    B: Graph<NodeWeight, EdgeWeight, NodeRef = N2Ref, EdgeRef = E2Ref>,
{
    ///
    /// Returns a tuple (N, N2) of node references.
    /// N contains the smallest unmatched node within the pattern graph,
    /// and N2 unmatched nodes within the base graph.
    /// When matched nodes contain a successor, we use another method.
    ///
    /// This ordering is described in the 1999 first paper.
    ///
    fn find_unmatched_nodes(&'a self) -> (Option<NRef>, Vec<N2Ref>) {
        let n = self
            .pattern_graph
            .nodes()
            .filter(|n| !self.core_1.contains_key(n))
            .min();

        let base_nodes: Vec<_> = self
            .base_graph
            .nodes()
            .filter(|n| !self.core_2.contains_key(n))
            .collect();

        (n, base_nodes)
    }

    ///
    /// Returns a tuple (N, N2) of node references.
    /// N is the smallest node reference from `pattern_graph` that is yet unmatched,
    /// and the destination of an outgoing matched node.
    ///
    /// N2 is the set of unmatched nodes from `base_graph` that are the destination of a
    /// previously matched node.
    ///
    fn find_unmatched_outgoing_neighbors(&'a self) -> (Option<NRef>, Vec<N2Ref>) {
        // From out_1, i.e. outgoing neighbors, only select those
        // where no entry is in core_1.
        let n = self
            .out_1
            .keys()
            .filter(|n| !self.core_1.contains_key(n))
            .min()
            .cloned();
        let n2: Vec<_> = self
            .out_2
            .keys()
            .filter(|n| !self.core_2.contains_key(n))
            .cloned()
            .collect();
        (n, n2)
    }

    ///
    /// Matches node n to node m, where n is from the pattern, and m is from the base graph.
    /// Update out_1 and out_2 to hold the insertion depths.
    ///
    fn assign(&mut self, n: NRef, m: N2Ref, depth: usize) {
        // Update core and out sets.
        self.core_1.insert(n, m);
        self.core_2.insert(m, n);
        self.out_1.entry(n).or_insert(depth);
        self.out_2.entry(m).or_insert(depth);

        // Iterate over the neighbors of n, and add them to the out_1 set/map.
        self.pattern_graph
            .outgoing_edges(n)
            .map(|e| self.pattern_graph.adjacent_nodes(e).1)
            .for_each(|n_out| {
                self.out_1.entry(n_out).or_insert(depth);
            });
        // Repeat the process for the outgoing neighbors of m.
        self.base_graph
            .outgoing_edges(m)
            .map(|m| self.base_graph.adjacent_nodes(m).1)
            .for_each(|m_out| {
                self.out_2.entry(m_out).or_insert(depth);
            });
    }

    ///
    /// Tests if matching node n to node m is allowed. This is a shorthand for
    /// these conditions:
    ///
    /// ### Syntactic:
    /// 1. `check_successor_relation`
    ///
    /// ### Semantic:
    /// 1. `check_node_semantics`
    ///
    fn is_valid_matching(&self, n: NRef, m: N2Ref) -> bool {
        self.check_successor_relation(n, m) && self.check_node_semantics(n, m)
    }

    ///
    /// Test that assigning n to m leaves the successor relations intact:
    ///
    /// 1. We may map any matched successor n' of n in `pattern_graph` to
    /// another matched node m' that succeeds m in `base_graph`.
    ///
    /// 2. We may map any matched successor `m` of m in `base_graph` to
    /// another matched node n' that succeeds n in `pattern_graph`.
    ///
    fn check_successor_relation(&self, n: NRef, m: N2Ref) -> bool {
        // M_1(s) intersected with Succ(G_1, n)
        let n_succs: HashSet<_> = self
            .pattern_graph
            .outgoing_edges(n)
            .map(|e| self.pattern_graph.adjacent_nodes(e).1)
            .filter(|n_succ| self.core_1.contains_key(n_succ))
            .collect();

        // M_2(s) intersected with Succ(G_2, m).
        let m_succs: HashSet<_> = self
            .base_graph
            .outgoing_edges(m)
            .map(|e| self.base_graph.adjacent_nodes(e).1)
            .filter(|m_succ| self.core_2.contains_key(m_succ))
            .collect();

        // n2 should be mapped to another node m2, and that node is a successor of m.
        n_succs
            .iter()
            .all(|n2| self.core_1.get(n2).is_some_and(|m2| m_succs.contains(m2)))
            && m_succs
                .iter()
                .all(|m2| self.core_2.get(m2).is_some_and(|n2| n_succs.contains(n2)))
    }

    ///
    /// Test whether node n in the pattern may be matched to node m
    /// in the graph. That means that the matcher function for n
    /// must return true for the node referred to by m.
    ///
    fn check_node_semantics(&self, n: NRef, m: N2Ref) -> bool {
        let matcher = self.pattern_graph.node_weight(n);
        let refed_node = self.base_graph.node_weight(m);
        matcher(refed_node)
    }

    ///
    /// Undoes the matching between nodes n and m.
    ///
    fn unassign(&mut self, n: &NRef, m: &N2Ref, depth: usize) {
        // Remove from core set
        self.core_1.remove(n);
        self.core_2.remove(m);
        // Remove from out set + neighbors.
        Self::remove(n, depth, &mut self.out_1);
        Self::remove(m, depth, &mut self.out_2);

        // out_1/Pattern Graph
        self.pattern_graph
            .outgoing_edges(*n)
            .map(|n| self.pattern_graph.adjacent_nodes(n).1)
            .for_each(|n_out| Self::remove(&n_out, depth, &mut self.out_1));
        // out_2/Base Graph
        self.base_graph
            .outgoing_edges(*m)
            .map(|m| self.base_graph.adjacent_nodes(m).1)
            .for_each(|m_out| Self::remove(&m_out, depth, &mut self.out_2));
    }

    ///
    /// Removes index from map if its insertion depth is equal to
    /// its insertion depth. Removes thus nodes from the out set.
    ///
    fn remove<N>(index: &N, depth: usize, map: &mut HashMap<N, usize>)
    where
        N: Eq + Hash,
    {
        if let Some(insert_depth) = map.get(index) {
            if insert_depth == &depth {
                map.remove(index);
            }
        }
    }

    ///
    /// Produces a new AdjGraph for the current graph state.
    ///
    /// Copy the keys from pattern_graph along with the weights referred
    /// to by the depths from base_graph.
    ///
    fn produce_graph(&mut self) {
        let node_list: HashMap<NRef, &NodeWeight> = self
            .core_1
            .iter()
            .map(|(n, m)| (*n, self.base_graph.node_weight(*m)))
            .collect();

        let result: AdjGraph<'a, NodeWeight, EdgeWeight, NRef, ERef> = AdjGraph::new(node_list);
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
            // Find unmatched nodes that are neighbors of matched nodes.
            let (mut pat_node, mut base_nodes) = self.find_unmatched_outgoing_neighbors();
            // Failing that, try unmatched and unconnected nodes.
            if pat_node.is_none() || base_nodes.is_empty() {
                (pat_node, base_nodes) = self.find_unmatched_nodes();
            }

            // Assert we always will have a node in the pattern.
            let n = pat_node.unwrap();
            for m in base_nodes {
                self.assign(n, m, depth);
                // Test compatibility.
                if self.is_valid_matching(n, m) {
                    self.find_subgraphs(depth + 1);
                }
                self.unassign(&n, &m, depth);
            }
        }
    }
}

impl<'a, NodeWeight, EdgeWeight, NRef, ERef, N2Ref, E2Ref, P, B>
    SubgraphAlgorithm<'a, NodeWeight, EdgeWeight, NRef, ERef, N2Ref, E2Ref, P, B>
    for VfState<'a, NodeWeight, EdgeWeight, NRef, ERef, N2Ref, E2Ref, P, B>
where
    NRef: Copy + Hash + Ord,
    N2Ref: Copy + Hash + Eq,
    P: PatternGraph<NodeWeight, EdgeWeight, NodeRef = NRef, EdgeRef = ERef>,
    B: Graph<NodeWeight, EdgeWeight, NodeRef = N2Ref, EdgeRef = E2Ref>,
{
    ///
    /// Creates a new VfState for the given pattern graph and base graph.
    /// Initialized for each base_graph instance, to use its specific indices.
    ///
    /// ## Input:
    /// 1. `pattern_graph`, a PatternGraph with NRef node references.
    /// 2. `base_graph`, any Graph with N2Ref node references.
    ///
    /// ## Output:
    /// A VfState struct.
    ///
    fn init(
        pattern_graph: &'a P,
        base_graph: &'a B,
    ) -> VfState<'a, NodeWeight, EdgeWeight, NRef, ERef, N2Ref, E2Ref, P, B> {
        VfState {
            pattern_graph,
            base_graph,
            results: vec![],
            core_1: HashMap::new(),
            core_2: HashMap::new(),
            out_1: HashMap::new(),
            out_2: HashMap::new(),
        }
    }

    ///
    /// Handles empty patterns and otherwise calls the
    /// predefined search function.
    ///
    fn run_query(&mut self) {
        if self.pattern_graph.is_empty_pattern() {
            return;
        }
        self.find_subgraphs(0);
    }

    ///
    /// Returns a reference to results.
    ///
    fn get_results(&self) -> &Vec<AdjGraph<NodeWeight, EdgeWeight, NRef, ERef>> {
        &self.results
    }
}
