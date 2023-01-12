use crate::{graph::Graph, graph_backends::adj_graphs::AdjGraph};

///
/// The Matcher type stands for any function that evaluates, given an element
/// in the base graph with the type Weight, if the pattern graph accepts this element.
///
/// As an example, we may define a function that tests if a node matches a Rust pattern.
///
pub type Matcher<Weight> = dyn Fn(Weight) -> bool;

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
    fn add_node_to_match(&mut self, name: &str, matcher: Box<Matcher<NodeWeight>>)
        -> Self::NodeRef;

    ///
    /// Adds the named `edge` to the pattern.
    ///
    /// `edge` is directed, and runs from the node referenced by `from` to the node referenced by `to`.
    /// The matched edge must then fulfill the `matcher` function.
    ///
    /// Returns an `EdgeRef` to the newly added edge.
    ///
    fn add_edge_to_match(
        &mut self,
        name: &str,
        from: Self::NodeRef,
        to: Self::NodeRef,
        matcher: Box<Matcher<EdgeWeight>>,
    ) -> Self::EdgeRef;
}

///
/// The SubgraphAlgorithm trait specifies any algorithm that can solve the subgraph isomorphism problem.
/// Solving this problem lies at the core of graph pattern matching.
///
pub trait SubgraphAlgorithm {
    ///
    /// Finds all subgraphs of `base_graph` that match `pattern_graph`.
    ///
    /// # Input:
    /// Two graphs, `base_graph` (any Graph instance), and `pattern_graph` (a PatternGraph instance).
    /// `base_graph` and `pattern_graph` share both node and edge types (NodeWeight/EdgeWeight).
    ///
    /// `pattern_graph` uses N1RefType and E1RefType for references over nodes and edges.
    /// `base_graph` uses N2RefType and E2RefType, respectively.
    ///
    /// # Output:
    /// A vector of ResultGraph, whose nodes and edges have NodeWeight/EdgeWeight types,
    /// and its references N1RefType/N2RefType. We want to access matched elements of
    /// the base graph by references we set in the pattern graph.
    ///
    /// An implementation find_subgraphs should guarantee set semantics, so that every found
    /// graph pattern occurs only once.
    ///
    /// # Panics:
    /// `base_graph` is a directed graph, and `pattern_graph` is not, or vice versa.
    ///
    fn find_subgraphs<
        NodeWeight,
        EdgeWeight,
        N1RefType: Copy,
        N2RefType,
        E1RefType,
        E2RefType,
        PatternGraphType,
        BaseGraphType,
    >(
        pattern_graph: &PatternGraphType,
        base_graph: &BaseGraphType,
    ) -> Vec<AdjGraph<NodeWeight, EdgeWeight, N1RefType, E1RefType>>
    where
        PatternGraphType:
            PatternGraph<NodeWeight, EdgeWeight, NodeRef = N1RefType, EdgeRef = E1RefType>,
        BaseGraphType: Graph<NodeWeight, EdgeWeight, NodeRef = N2RefType, EdgeRef = E2RefType>;
}
