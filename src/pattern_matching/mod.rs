use vf_algorithms::VfState;

use crate::graph::Graph;

/// Module that contains an implementation for subgraph algorithms.
///
/// Goal: Contain Subgraph Isomorphism Algorithms based on the VF family (VF2, VF2+, VF3...).
pub mod vf_algorithms;

/// Definition of matcher types.
mod matcher;
pub use matcher::*;

/// Definition of pattern types.
mod pattern;
pub use pattern::*;

/// trait specifying a generic algorithm for solving subgraph matching
mod algorithm;
pub use algorithm::*;

/// Creates an empty new graph pattern.
pub fn new_pattern<NodeWeight, EdgeWeight>() -> impl PatternGraph<NodeWeight, EdgeWeight> {
    petgraph::Graph::new()
}

/// Solve a graph matching problem instance using an approach based the VF algorithms.
///
/// The algorithm implementation used is provided by the [vf_algorithms] module.
///
/// See the [SubgraphAlgorithm::eval] documentation on how to use it.
pub fn solve_vf<'a, N, E, Pattern>(
    pattern_graph: &'a Pattern,
    base_graph: &'a impl Graph<N, E>,
) -> Vec<MatchedGraph<'a, N, E, Pattern>>
where
    Pattern: PatternGraph<N, E>,
{
    VfState::eval(pattern_graph, base_graph)
}
