use crate::graph::Graph;

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
/// A pattern graph uses string slices as node and edge indices, and matcher functions as weights.
///
/// With string indices, we may explicitly name the elements to be matched.
/// With matcher functions, we can test if an element semantically fits to a subgraph to be found,
/// for example, if it matches a given Rust pattern.
///
/// PatternGraph is generic with regards to node and edge weights of the graphs it should match on.
///
pub trait PatternGraph<NodeWeight, EdgeWeight, NodeMatcher, EdgeMatcher>:
    Graph<NodeMatcher, EdgeMatcher>
where
    NodeMatcher: Fn(NodeWeight) -> bool,
    EdgeMatcher: Fn(EdgeWeight) -> bool,
{
    ///
    /// Adds the node `name` to the pattern. Any node that matches `name`
    /// must fulfill the `matcher` function.
    ///
    fn add_node(&mut self, name: &str, matcher: &NodeMatcher);

    ///
    /// Adds the edge `edge` to the pattern.
    /// `edge` runs from a matched node `from` to a second node `to`, and must fulfill the
    /// `matcher` function.
    ///
    fn add_edge(&mut self, name: &str, from: &str, to: &str, matcher: &EdgeMatcher);
}

///
/// Type alias for weight iterators in pattern graphs.
///
pub type MatcherIterator<Weight> = dyn Iterator<Item = Matcher<Weight>>;
///
/// Type alias for reference iterators in pattern graphs.
///
pub type ReferenceIterator<'a> = dyn Iterator<Item = &'a str>;

///
/// The SubgraphAlgorithm trait specifies any algorithm that can solve the subgraph isomorphism problem.
/// Solving this problem lies at the core of graph pattern matching.
///
pub trait SubgraphAlgorithm {
    ///
    /// Finds all subgraphs of `base_graph` that match `pattern_graph`.
    /// `base_graph` and `pattern_graph` must share both node and edge types.
    ///
    /// # Panics:
    /// `base_graph` is a directed graph, and `pattern_graph` is not, or vice versa.
    ///
    fn find_subgraphs<'a, NodeWeight, EdgeWeight, ERefType, NRefType>(
        pattern_graph: dyn PatternGraph<
            NodeWeight,
            EdgeWeight,
            Matcher<NodeWeight>,
            Matcher<EdgeWeight>,
            NodeRef = &str,
            EdgeRef = &str,
            AdjacentEdgesIterator<'a> = ReferenceIterator,
            OutgoingEdgesIterator<'a> = ReferenceIterator,
            IncomingEdgesIterator<'a> = ReferenceIterator,
            NodeWeightsIterator<'a> = MatcherIterator<NodeWeight>,
            EdgeWeightsIterator<'a> = MatcherIterator<EdgeWeight>,
            NodesIterator<'a> = ReferenceIterator,
            EdgesIterator<'a> = ReferenceIterator,
        >, //base_graph: &dyn Graph<NodeWeight, EdgeWeight, NodeRef = NRefType, EdgeRef = ERefType>,
    );
}
