use std::hash::Hash;

use crate::{graph::Graph, graph_backends::filter_map::FilterMap};

///
/// The Matcher type stands for any function that evaluates, given an element
/// in the base graph with the type Weight, if the pattern graph accepts this element.
///
/// As an example, we may define a function that tests if a node matches a Rust pattern.
///
pub type Matcher<Weight> = dyn Fn(&Weight) -> bool;

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
/// use rustgql::query::*;
///
/// # // This line is hidden in the docs but required to pass the docstest, see https://users.rust-lang.org/t/how-to-use-a-macro-in-the-doc-test/3664/5?u=slrtbtfs
/// # fn main() {
///
///
/// let student = matcher!('a' ..= 'c' |  'd' ..= 'f');
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
    Graph<Box<Matcher<NodeWeight>>, Box<Matcher<EdgeWeight>>>
{
    ///
    /// Adds the node identified by `name` to the pattern. Any node that matches `name`
    /// must fulfill the `matcher` function.
    ///
    /// Returns a NodeRef to the added node.
    ///
    fn add_node<M>(&mut self, matcher: M) -> Self::NodeRef
    where
        M: Fn(&NodeWeight) -> bool + 'static;

    ///
    /// Adds the named `edge` to the pattern.
    ///
    /// `edge` is directed, and runs from the node referenced by `from` to the node referenced by `to`.
    /// The matched edge must then fulfill the `matcher` function.
    ///
    /// Returns an `EdgeRef` to the newly added edge.
    ///
    fn add_edge<M>(&mut self, from: Self::NodeRef, to: Self::NodeRef, matcher: M) -> Self::EdgeRef
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
    /// A reference to a vector of AdjGraph, whose nodes and edges have NodeWeight/EdgeWeight types,
    /// and its references N1Ref/E1RefType. We want to access matched elements of
    /// the base graph by references we set in the pattern graph.
    ///
    /// An implementation find_subgraphs should guarantee set semantics, so that every found
    /// graph pattern occurs only once.
    ///
    /// If `pattern_graph` is an empty graph without nodes (or edges), or if no subgraph of `base_graph`
    /// can be matched to it, then we return an empty vector.
    ///
    ///
    /// # Panics:
    /// `base_graph` is a directed graph, and `pattern_graph` is not, or vice versa.
    ///
    fn eval(
        pattern_graph: &'a PatternGraphType,
        base_graph: &'a BaseGraphType,
    ) -> Vec<MatchedGraph<'a, NodeWeight, EdgeWeight, PatternGraphType>>;
}

pub type MatchedGraph<'a, N, E, P> =
    FilterMap<'a, Box<Matcher<N>>, Box<Matcher<E>>, &'a N, &'a E, P>;
