#![feature(type_alias_impl_trait)]
#![feature(is_some_and)]

/// Generic graph traits used as abstractions within this library.
pub mod graph;
pub mod query;

/// Contains implementations for query algorithms.
pub mod query_algorithms;

/// Implementation of filter + map graph transformations of node/edge weights.
pub mod filter_map;

/// Pattern matching on graphs.
pub mod pattern_matching;

/// Implements the traits defined in this crate for [``::petgraph::graph::Graph``].
mod petgraph;
