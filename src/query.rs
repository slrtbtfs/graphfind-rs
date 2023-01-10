use petgraph::visit::NodeRef;

use crate::graph::Graph;

///
/// Type definition of a matching function, generic in its weight.
/// Applicable to both nodes and edges.
///
pub type Matcher<Weight> = dyn Fn(Weight) -> bool;

///
/// Defines a pattern graph, i.e. a specification for subgraphs that we want to find.
///
/// A pattern graph uses string slices as node and edge indices, and matcher functions as weights.
///
/// With string indices, we may explicitly name the elements to be matched.
/// With matcher functions, we can test if an element semantically fits to a subgraph to be found,
/// for example, if it matches a given Rust pattern.
///
/// PatternGraph is generic with regards to node and edge weights of the graphs it should match on.
///
pub trait PatternGraph<NodeWeight, EdgeWeight> {
    ///
    /// Adds the node `name` to the pattern. Any node that matches `name`
    /// must fulfill the `matcher` function.
    ///
    fn add_node(&mut self, name: &str, matcher: &Matcher<NodeWeight>);

    ///
    /// Adds the edge `edge` to the pattern.
    /// `edge` runs from a matched node `from` to a second node `to`, and must fulfill the
    /// `matcher` function.
    ///
    fn add_edge(&mut self, name: &str, from: &str, to: &str, matcher: &Matcher<EdgeWeight>);
}

///
/// The SubgraphAlgorithm trait specifies any algorithm that can solve the subgraph isomorphism problem.
/// Solving this problem lies at the core of graph pattern matching.
///
pub trait SubgraphAlgorithm {
    ///
    /// Finds all subgraphs of `base_graph` that match `pattern_graph`.
    /// `base_graph` and `pattern_graph` must share both node and edge types.
    ///
    fn find_subgraphs<NodeWeight, EdgeWeight>(
        pattern_graph: &dyn PatternGraph<NodeWeight, EdgeWeight>,
        base_graph: i32,
    );
}
