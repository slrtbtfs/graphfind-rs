use crate::query::{Matcher, PatternGraph};

///
/// Defines an PatternGraph over an directed petgraph. Guarantees that
/// our graph should always be directed.
///
impl<NodeWeight, EdgeWeight> PatternGraph<NodeWeight, EdgeWeight>
    for petgraph::graph::Graph<Box<Matcher<NodeWeight>>, Box<Matcher<EdgeWeight>>>
{
    fn count_nodes(&self) -> usize {
        self.node_count()
    }

    ///
    /// Adds the node to match, and returns the reference.
    ///
    fn add_node_to_match(
        &mut self,
        name: &str,
        matcher: Box<Matcher<NodeWeight>>,
    ) -> Self::NodeRef {
        self.add_node(matcher)
    }

    ///
    /// Adds the edge to match, and returns the reference.
    ///
    fn add_edge_to_match(
        &mut self,
        name: &str,
        from: Self::NodeRef,
        to: Self::NodeRef,
        matcher: Box<Matcher<EdgeWeight>>,
    ) -> Self::EdgeRef {
        self.add_edge(from, to, matcher)
    }
}