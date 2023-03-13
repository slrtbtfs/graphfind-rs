#![feature(type_alias_impl_trait)]
#![feature(is_some_and)]

// Public Interface defined within the root.
pub mod file_io;

///
/// Defines the Graph trait for basic navigation in typed graphs.
///
pub mod graph;
pub mod print;
pub mod query;

///
/// Contains implementations for the Graph trait.
///
pub mod graph_backends;
///
/// Contains implementations for query algorithms.
///
pub mod query_algorithms;

mod petgraph;
