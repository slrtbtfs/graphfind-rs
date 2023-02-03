use crate::query::{Condition, PatternGraph};

///
/// Defines an PatternGraph over an directed petgraph. Guarantees that
/// our graph should always be directed.
///
impl<NodeWeight, EdgeWeight> PatternGraph<NodeWeight, EdgeWeight>
    for petgraph::graph::Graph<Box<Condition<NodeWeight>>, Box<Condition<EdgeWeight>>>
{
    ///
    /// Adds the node to match, and returns the reference.
    ///
    fn add_node_to_match_full(
        &mut self,
        condition: Box<Condition<NodeWeight>>,
        ignore: bool,
    ) -> Self::NodeRef {
        self.add_node(condition)
    }

    ///
    /// Adds the edge to match, and returns the reference.
    ///
    fn add_edge_to_match_full(
        &mut self,
        from: Self::NodeRef,
        to: Self::NodeRef,
        matcher: Box<Condition<EdgeWeight>>,
        ignore: bool,
    ) -> Self::EdgeRef {
        self.add_edge(from, to, matcher)
    }
}
