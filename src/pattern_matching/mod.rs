//!
//! The problem being solved here is finding subgraphs matching
//! a given subgraph pattern in a larger base graph. The subgraph
//! pattern is a graph where each element weight is a condition.
//!  A valid match is a subgraph of the base graph that is isomorphic to the
//! pattern where each element fulfils the condition of the corresponding
//! pattern element.
//!
//! It is also possible to add hidden subgraph elements, that are removed from the matches before returning them to the caller.
//!
//! While the architecture is designed to eventually support multiple matching
//! implementations, currently only one VF based algorithm is implemented. The
//! easiest way to use it is creating a new pattern with
//! [pattern_matching::new_pattern] and passing that pattern to
//! [pattern_matching::solve_vf]. Conditions in the pattern graph can be either constructed as function closures or with the [matcher] macro.
//!
//! For examples see the unit tests for this module.

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
