#![feature(type_alias_impl_trait)]
#![feature(is_some_and)]


// Public Interface defined within the root.
pub mod file_io;

/// Defines the Graph trait for basic navigation in typed graphs.
pub mod graph;
pub mod print;
pub mod query;

///
/// Contains implementations for query algorithms.
///
pub mod query_algorithms;

///
/// Implements the traits defined in this crate for [``::petgraph::graph::Graph``].
///
mod petgraph;
///
/// Implementation of filter + map graph transformations of node/edge weights.
///
pub mod filter_map;
