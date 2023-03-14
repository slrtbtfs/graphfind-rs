use crate::graph::Graph;

use super::Matcher;

/// Struct that holds all relevant matching information for a single node/edge.
pub struct PatternElement<Weight> {
    ///
    /// The matching function.
    ///
    condition: Box<Matcher<Weight>>,
    ///
    /// A flag that tells us if we should include the matched element in the result, or not.
    ///
    ignore: bool,
}

/// Holds the constructor for Matcher.
impl<Weight> PatternElement<Weight> {
    /// Creates a new Matcher struct. If `ignore` is true, the node/edge will be hidden from the result graph.
    pub fn new(condition: Box<Matcher<Weight>>, ignore: bool) -> Self {
        Self { condition, ignore }
    }

    /// Checks the matched node should appear in the result graph.
    pub fn should_appear(&self) -> bool {
        !self.ignore
    }

    /// Tests if the given element matches the condition this matcher.
    pub fn may_match(&self, element: &Weight) -> bool {
        (self.condition)(element)
    }
}

/// Defines a pattern graph, i.e. a specification for subgraphs that we want to find. This trait
/// extends the Graph trait, to allow for navigation & getting subgraph info.
///
/// A pattern graph uses matcher functions as weights.
/// With matcher functions, we can test if an element semantically fits to a subgraph to be found,
/// for example, if it matches a given Rust pattern.
///
/// PatternGraph is generic with regards to node and edge weights of the graphs it should match on.
pub trait PatternGraph<NodeWeight, EdgeWeight>:
    Graph<PatternElement<NodeWeight>, PatternElement<EdgeWeight>>
{
    /// Adds a new node to the pattern.
    ///
    /// A matched node appears in the result graph.
    ///
    /// ## Input:
    /// `condition`, a [Matcher] that holds a function to test if a node in a base graph matches what we want in the pattern graph.
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
    /// condition, a `Matcher` that holds a function to test if a node in a base graph matches.
    /// what we want in the pattern graph.
    ///
    /// ## Output:
    /// A node reference.
    fn add_hidden_node<C>(&mut self, condition: C) -> Self::NodeRef
    where
        C: Fn(&NodeWeight) -> bool + 'static;

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
    fn add_edge<C>(
        &mut self,
        from: Self::NodeRef,
        to: Self::NodeRef,
        condition: C,
    ) -> Self::EdgeRef
    where
        C: Fn(&EdgeWeight) -> bool + 'static;
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
    fn add_hidden_edge<C>(
        &mut self,
        from: Self::NodeRef,
        to: Self::NodeRef,
        condition: C,
    ) -> Self::EdgeRef
    where
        C: Fn(&EdgeWeight) -> bool + 'static;
}
