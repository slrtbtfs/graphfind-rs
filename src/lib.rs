//! This library implements two approaches to searching graphs, one based on
//! pattern matching, the other based on filter map transformations.
//!
//! Additionally it includes traits and infrastructure to make those
//! functionalities generic and extensible.
//!
//! It was created with the aim to explore graph query languages that
//! that are written in general purpose programming languages and
//! integrate well with compilers and existing development environments.
//!
//! The create currently relies on unstable Rust features.

#![feature(type_alias_impl_trait)]
#![feature(is_some_and)]

/// Generic graph traits used as abstractions within this library.
pub mod graph;

/// Implementation of filter + map graph transformations of node/edge weights.
pub mod filter_map;

/// Pattern matching on graphs.
pub mod pattern_matching;

/// Implements the traits defined in this crate for [``::petgraph::graph::Graph``].
mod petgraph;
