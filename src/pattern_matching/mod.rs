use crate::query::PatternGraph;

///
/// Creates an empty new graph pattern.
///
pub fn new_pattern<NodeWeight, EdgeWeight>() -> impl PatternGraph<NodeWeight, EdgeWeight> {
    petgraph::Graph::new()
}
