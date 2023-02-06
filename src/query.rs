use std::hash::Hash;

use crate::{graph::Graph, graph_backends::filter_map::FilterMap};

///
/// The Condition type stands for any function that evaluates, given an element
/// in the base graph with the type Weight, if the pattern graph accepts this element.
///
/// As an example, we may define a function that tests if a node matches a Rust pattern.
///
pub type Condition<Weight> = dyn Fn(&Weight) -> bool;

///
/// Defines a struct that holds all relevant node/edge matching information.
///
pub struct Matcher<Weight> {
    ///
    /// The matching function.
    ///
    condition: Box<Condition<Weight>>,
    ///
    /// A flag that tells us if we should include the matched element in the result, or not.
    ///
    ignore: bool,
}

///
/// Holds the constructor for Matcher.
///
impl<Weight> Matcher<Weight> {
    ///
    /// Creates a new Matcher struct.
    ///
    pub fn new(condition: Box<Condition<Weight>>, ignore: bool) -> Self {
        Self { condition, ignore }
    }

    ///
    /// Returns true if and only if the matched node should appear in the result.
    ///
    pub fn should_appear(&self) -> bool {
        !self.ignore
    }

    ///
    /// Tests if the given element may be matched.
    ///
    pub fn may_match(&self, element: &Weight) -> bool {
        (self.condition)(element)
    }
}

///
/// Defines a pattern graph, i.e. a specification for subgraphs that we want to find. This trait
/// extends the Graph trait, to allow for navigation & getting subgraph info.
///
/// A pattern graph uses matcher functions as weights.
/// With matcher functions, we can test if an element semantically fits to a subgraph to be found,
/// for example, if it matches a given Rust pattern.
///
/// PatternGraph is generic with regards to node and edge weights of the graphs it should match on.
///
pub trait PatternGraph<NodeWeight, EdgeWeight>:
    Graph<Matcher<NodeWeight>, Matcher<EdgeWeight>>
{
    ///
    /// Adds a new node to the pattern.
    ///
    /// ## Input:
    /// 1. condition, a `Box` that holds a function to test if a node in a base graph matches
    /// what we want in the pattern graph.
    /// 2. ignore, a flag that tells us if the matched node should appear in the result.
    ///
    /// ## Output:
    /// A NodeRef.
    ///
    fn add_node_to_match_full(
        &mut self,
        condition: Box<Condition<NodeWeight>>,
        ignore: bool,
    ) -> Self::NodeRef;

    ///
    /// Adds a new node to the pattern. Matched nodes appear in the result.
    ///
    /// Returns a NodeRef to the added node.
    ///
    fn add_node_to_match<M>(&mut self, matcher: M) -> Self::NodeRef
    where
        M: Fn(&NodeWeight) -> bool + 'static;

    ///
    /// Adds a new, directed edge to the pattern.
    ///
    /// ## Input:
    /// 1. from, the source node of the new edge.
    /// 2. to, the destination node.
    /// 3. condition, a `Box` that holds a function to test if an edge in a base graph matches
    /// that we want in the pattern graph.
    /// 4. ignore, a flag that tells us if the matched edge should appear in the result.
    ///
    /// ## Output:
    /// An EdgeRef.
    ///
    /// ## Panics:
    /// `ignore` is set to false, and either one of the nodes under `from` and `to` is ignored.
    /// We could then refer to a node in the result that we do not want to refer to.
    ///
    fn add_edge_to_match_full(
        &mut self,
        from: Self::NodeRef,
        to: Self::NodeRef,
        condition: Box<Condition<EdgeWeight>>,
        ignore: bool,
    ) -> Self::EdgeRef;

    ///
    /// Adds a new edge to the pattern. This edge will appear in the result graphs.
    ///
    /// Returns an `EdgeRef` to the edge.
    ///
    fn add_edge_to_match<M>(
        &mut self,
        from: Self::NodeRef,
        to: Self::NodeRef,
        matcher: M,
    ) -> Self::EdgeRef
    where
        M: Fn(&EdgeWeight) -> bool + 'static;
}

///
/// The SubgraphAlgorithm trait specifies any algorithm that can solve the subgraph isomorphism problem.
/// Solving this problem lies at the core of graph pattern matching.
///
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
    ///
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
    ///
    fn eval(
        pattern_graph: &'a PatternGraphType,
        base_graph: &'a BaseGraphType,
    ) -> Vec<MatchedGraph<'a, NodeWeight, EdgeWeight, PatternGraphType>>;
}

///
/// Type definition of MatchedGraph.
///
pub type MatchedGraph<'a, N, E, P> = FilterMap<'a, Matcher<N>, Matcher<E>, &'a N, &'a E, P>;
