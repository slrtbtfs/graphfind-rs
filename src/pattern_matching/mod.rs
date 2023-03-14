///
/// Creates an empty new graph pattern.
///
pub fn new_pattern<NodeWeight, EdgeWeight>() -> impl PatternGraph<NodeWeight, EdgeWeight> {
    petgraph::Graph::new()
}

/// Solve a graph matching problem instance using an approach based the VF algorithms.
///
/// The algorithm implementation used is provided by the [vf_algorithms] module.
///
/// See the [SubgraphAlgorithm::eval] documentation on how to use it.
pub fn solve_vf<'a, N, E, Pattern>(
    pattern_graph: &'a Pattern,
    base_graph: &'a impl Graph<N, E>,
) -> Vec<MatchedGraph<'a, N, E, Pattern>>
where
    Pattern: PatternGraph<N, E>,
{
    VfState::eval(pattern_graph, base_graph)
}

use std::hash::Hash;
use vf_algorithms::VfState;

use crate::filter_map::FilterMap;
use crate::graph::Graph;

///
/// Module that contains an implementation for subgraph algorithms.
///
/// Goal: Contain Subgraph Isomorphism Algorithms based on the VF family (VF2, VF2+, VF3...).
///
pub mod vf_algorithms;

///
/// The Condition type stands for any function that evaluates, given an element
/// in the base graph with the type Weight, if the pattern graph accepts this element.
///
/// As an example, we may define a function that tests if a node matches a Rust pattern.
///
/// Recommended ways to create conditions are using either the lambda functions or the [crate::matcher] macro.
///
pub type Condition<Weight> = dyn Fn(&Weight) -> bool;

///
/// Struct that holds all relevant matching information for a single node/edge.
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
impl<Weight> Matcher<Weight> {
    ///
    /// Creates a new Matcher struct. If `ignore` is true, the node/edge will be hidden from the result graph.
    ///
    pub fn new(condition: Box<Condition<Weight>>, ignore: bool) -> Self {
        Self { condition, ignore }
    }

    ///
    /// Checks the matched node should appear in the result graph.
    ///
    pub fn should_appear(&self) -> bool {
        !self.ignore
    }

    ///
    /// Tests if the given element matches the condition this matcher.
    ///
    pub fn may_match(&self, element: &Weight) -> bool {
        (self.condition)(element)
    }
}

///
/// Creates a `Matcher` function from a given pattern
///
/// The syntax is similar to the `std::matches` macro.
/// Calling matcher with no arguments will match anything.
///
/// # Examples
/// ```
/// #[macro_use]
/// extern crate rustgql;
/// use rustgql::pattern_matching::*;
///
/// # // This line is hidden in the docs but required to pass the docstest, see https://users.rust-lang.org/t/how-to-use-a-macro-in-the-doc-test/3664/5?u=slrtbtfs
///
/// # fn main() {
///
/// enum Person {
///     Student {name: String},
///     Prof {},
/// }
///
/// let student = matcher!(Person::Student{..});
///
/// let even = matcher!(i if i % 2 == 0);
/// # }
#[macro_export]
macro_rules! matcher {
    () => {matcher!(_)};
    ($(|)? $( $pattern:pat_param )|+ $( if $guard: expr )? $(,)?) => {
        |__weight__: &_|
        match __weight__ {
            $( $pattern )|+ $( if $guard )? => true,
            _ => false
        }
    };
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
    /// Adds a new node to the pattern.
    ///
    /// A matched node appears in the result graph.
    ///
    /// ## Input:
    /// `condition`, a [Condition] that holds a function to test if a node in a base graph matches what we want in the pattern graph.
    ///
    /// ## Output:
    /// A node reference.
    fn add_node<C>(&mut self, condition: C) -> Self::NodeRef
    where
        C: Fn(&NodeWeight) -> bool + 'static;
    /// Adds a new hidden node to the pattern.
    /// A node that matches does not appear in the result graph, but is required to exist in the searched graph.
    ///
    /// ## Input:
    /// condition, a `Condition` that holds a function to test if a node in a base graph matches.
    /// what we want in the pattern graph.
    ///
    /// ## Output:
    /// A node reference.
    fn add_hidden_node<C>(&mut self, condition: C) -> Self::NodeRef
    where
        C: Fn(&NodeWeight) -> bool + 'static;

    ///
    /// Adds a new edge to the pattern. This edge will appear in the result graphs.
    ///
    /// ## Input:
    /// 1. `from`, the source node of the new edge.
    /// 2. `to`, the destination node.
    /// 3. `condition`, a function to test if an edge in a base graph matches
    /// what we want in the pattern graph.
    ///
    /// ## Output:
    /// An edge reference.
    ///
    /// ## Panics:
    /// Panics if one of the adjacent nodes is a hidden node.
    ///
    fn add_edge<C>(
        &mut self,
        from: Self::NodeRef,
        to: Self::NodeRef,
        condition: C,
    ) -> Self::EdgeRef
    where
        C: Fn(&EdgeWeight) -> bool + 'static;
    ///
    /// Adds a new, directed, hidden edge to the pattern.
    ///
    /// An edge that matches does not appear in the result graph, but is required to exist in the searched graph.
    ///
    /// ## Input:
    /// 1. `from`, the source node of the new edge.
    /// 2. `to`, the destination node.
    /// 3. `condition`, a function to test if an edge in a base graph matches
    /// what we want in the pattern graph.
    ///
    /// ## Output:
    /// An edge reference.
    ///
    ///
    fn add_hidden_edge<C>(
        &mut self,
        from: Self::NodeRef,
        to: Self::NodeRef,
        condition: C,
    ) -> Self::EdgeRef
    where
        C: Fn(&EdgeWeight) -> bool + 'static;
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
