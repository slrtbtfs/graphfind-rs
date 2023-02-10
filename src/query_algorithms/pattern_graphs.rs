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
    /// Adds a hidden node to match, and returns the reference.
    ///
    fn hide_node<C>(&mut self, condition: C) -> Self::NodeRef
    where
        C: Fn(&NodeWeight) -> bool + 'static,
    {
        self.add_node(Matcher::new(Box::new(condition), true))
    }

    ///
    /// Adds a visible node to match, and returns the reference.
    ///
    fn add_node<C>(&mut self, condition: C) -> Self::NodeRef
    where
        C: Fn(&NodeWeight) -> bool + 'static,
    {
        self.add_node(Matcher::new(Box::new(condition), false))
    }

    ///
    /// Adds a hidden/ignored edge to match, and returns the reference.
    ///
    fn hide_edge<C>(
        &mut self,
        from: Self::NodeRef,
        to: Self::NodeRef,
        condition: C,
    ) -> Self::EdgeRef
    where
        C: Fn(&EdgeWeight) -> bool + 'static,
    {
        self.add_edge(from, to, Matcher::new(Box::new(condition), true))
    }

    ///
    /// Adds an edge to match, and returns the reference.
    ///
    fn add_edge<C>(&mut self, from: Self::NodeRef, to: Self::NodeRef, condition: C) -> Self::EdgeRef
    where
        C: Fn(&EdgeWeight) -> bool + 'static,
    {
        if !self.node_weight(from).unwrap().should_appear()
            || !self.node_weight(to).unwrap().should_appear()
        {
            panic!("Must not refer to an edge that refers to nodes that cannot be referred!")
        }
        self.add_edge(from, to, Matcher::new(Box::new(condition), false))
    }
}
