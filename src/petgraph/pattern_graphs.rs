use crate::pattern_matching::{PatternElement, PatternGraph};

///
/// Defines an PatternGraph over an directed petgraph. Guarantees that
/// our graph should always be directed.
///
impl<NodeWeight, EdgeWeight> PatternGraph<NodeWeight, EdgeWeight>
    for petgraph::graph::Graph<PatternElement<NodeWeight>, PatternElement<EdgeWeight>>
{
    ///
    /// Adds a hidden node to match, and returns the reference.
    ///
    fn add_hidden_node<C>(&mut self, condition: C) -> Self::NodeRef
    where
        C: Fn(&NodeWeight) -> bool + 'static,
    {
        self.add_node(PatternElement::new(Box::new(condition), true))
    }

    ///
    /// Adds a visible node to match, and returns the reference.
    ///
    fn add_node<C>(&mut self, condition: C) -> Self::NodeRef
    where
        C: Fn(&NodeWeight) -> bool + 'static,
    {
        self.add_node(PatternElement::new(Box::new(condition), false))
    }

    ///
    /// Adds a hidden/ignored edge to match, and returns the reference.
    ///
    fn add_hidden_edge<C>(
        &mut self,
        from: Self::NodeRef,
        to: Self::NodeRef,
        condition: C,
    ) -> Self::EdgeRef
    where
        C: Fn(&EdgeWeight) -> bool + 'static,
    {
        self.add_edge(from, to, PatternElement::new(Box::new(condition), true))
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
        self.add_edge(from, to, PatternElement::new(Box::new(condition), false))
    }
}
