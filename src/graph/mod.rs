//! Everywhere a graph is used as a function argument or return type within this crate, it is required to implement the [graph::Graph] trait and possibly some of its trait extensions.
//!
//! This approach theoretically allows swapping out storage backends for graphs, and allows more implementation flexibility for the return type of graph operations.
//!
//! However, petgraphs Graph type is currently the only storage backend supported by the implementations in this crate itself.

/// Serializing graphs to files.
mod file_io;
pub use file_io::GraphReadWriter;
/// Printing graph visualizations in graphviz dot format.
mod print;
pub use print::VizDotGraph;

/// Helper functions for acessing graph attributes.
mod graph_helpers;
pub use self::graph_helpers::*;

/// Generic graph trait specification
mod graph_trait;
pub use graph_trait::Graph;
