use crate::query::{Matcher, PatternGraph};

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
    fn add_node_to_match<M>(&mut self, matcher: M) -> Self::NodeRef
    where
        M: Fn(&NodeWeight) -> bool + 'static,
    {
        self.add_node(Box::new(matcher))
    }

    ///
    /// Adds the edge to match, and returns the reference.
    ///
    fn add_edge_to_match<M>(
        &mut self,
        from: Self::NodeRef,
        to: Self::NodeRef,
        matcher: M,
    ) -> Self::EdgeRef
    where
        M: Fn(&EdgeWeight) -> bool + 'static,
    {
        self.add_edge(from, to, Box::new(matcher))
    }
}
