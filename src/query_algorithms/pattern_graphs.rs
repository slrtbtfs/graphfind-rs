use crate::query::{Matcher, PatternGraph};

/// Creates an empty new pattern
pub fn new_pattern<NodeWeight, EdgeWeight>() -> impl PatternGraph<NodeWeight, EdgeWeight> {
    petgraph::Graph::new()
}

///
/// Defines an PatternGraph over an directed petgraph. Guarantees that
/// our graph should always be directed.
///
impl<NodeWeight, EdgeWeight> PatternGraph<NodeWeight, EdgeWeight>
    for petgraph::graph::Graph<Box<Matcher<NodeWeight>>, Box<Matcher<EdgeWeight>>>
{
    ///
    /// Adds the node to match, and returns the reference.
    ///
    fn add_node<M>(&mut self, matcher: M) -> Self::NodeRef
    where
        M: Fn(&NodeWeight) -> bool + 'static,
    {
        self.add_node(Box::new(matcher))
    }

    ///
    /// Adds the edge to match, and returns the reference.
    ///
    fn add_edge<M>(&mut self, from: Self::NodeRef, to: Self::NodeRef, matcher: M) -> Self::EdgeRef
    where
        M: Fn(&EdgeWeight) -> bool + 'static,
    {
        self.add_edge(from, to, Box::new(matcher))
    }
}
