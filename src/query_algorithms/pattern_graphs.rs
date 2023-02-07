use crate::query::{Matcher, PatternGraph};

///
/// Creates an empty new pattern.
///
pub fn new_pattern<NodeWeight, EdgeWeight>() -> impl PatternGraph<NodeWeight, EdgeWeight> {
    petgraph::Graph::new()
}

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
    fn add_node_to_match_full<C>(&mut self, condition: C, ignore: bool) -> Self::NodeRef
    where
        C: Fn(&NodeWeight) -> bool + 'static,
    {
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
        C: Fn(&EdgeWeight) -> bool + 'static,
    {
        // Test logical conditions
        if !ignore
            && (!self.node_weight(from).unwrap().should_appear()
                || !self.node_weight(to).unwrap().should_appear())
        {
            panic!("Must not refer to an edge that refers to nodes that cannot be referred!")
        }
        self.add_edge(from, to, Matcher::new(Box::new(condition), ignore))
    }
}
