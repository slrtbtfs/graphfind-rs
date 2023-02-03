use crate::query::{Condition, Matcher, PatternGraph};

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
    fn add_node_to_match_full(
        &mut self,
        condition: Box<Condition<NodeWeight>>,
        ignore: bool,
    ) -> Self::NodeRef {
        self.add_node(Matcher::new(condition, ignore))
    }

    ///
    /// Adds the edge to match, and returns the reference.
    ///
    fn add_edge_to_match_full(
        &mut self,
        from: Self::NodeRef,
        to: Self::NodeRef,
        condition: Box<Condition<EdgeWeight>>,
        ignore: bool,
    ) -> Self::EdgeRef {
        // Test logical conditions
        if !ignore
            && (!self.node_weight(from).unwrap().should_appear()
                || !self.node_weight(to).unwrap().should_appear())
        {
            panic!("Must not refer to an edge that refers to nodes that cannot be referred!")
        }
        self.add_edge(from, to, Matcher::new(condition, ignore))
    }
}
