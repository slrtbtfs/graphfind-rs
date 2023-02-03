use crate::query::{PatternGraph, Matcher};

///
/// Defines an PatternGraph over an directed petgraph. Guarantees that
/// our graph should always be directed.
///
impl<NodeWeight, EdgeWeight> PatternGraph<NodeWeight, EdgeWeight>
    for petgraph::graph::Graph<Matcher<NodeWeight>, Matcher<EdgeWeight>>
{
    ///
    /// Adds the node to match, and returns the reference.
    ///
    fn add_node_to_match_full<C>(
        &mut self,
        condition: C,
        ignore: bool,
    ) -> Self::NodeRef
    where
        C: Fn(&NodeWeight) -> bool + 'static {
        self.add_node(Matcher::new(Box::new(condition), ignore))
    }

    ///
    /// Adds the edge to match, and returns the reference.
    ///
    fn add_edge_to_match_full<C>(
        &mut self,
        from: Self::NodeRef,
        to: Self::NodeRef,
        condition: C,
        ignore: bool,
    ) -> Self::EdgeRef
    where
        C: Fn(&EdgeWeight) -> bool + 'static {
        self.add_edge(from, to, Matcher::new(Box::new(condition), ignore))
    }
}
