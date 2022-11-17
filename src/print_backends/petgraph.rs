use std::fmt::Debug;

use petgraph::dot::Dot;
use petgraph::graph::{DefaultIx, EdgeIndex, NodeIndex};
use petgraph::Directed;

use crate::print::VizDotGraph;

/**
 * Print implementation for petgraph-type graphs.
 *
 * Requires NodeWeight, EdgeWeight to posess the Debug trait.
 */
impl<NodeWeight, EdgeWeight> VizDotGraph<NodeWeight, EdgeWeight, NodeIndex, EdgeIndex>
    for petgraph::graph::Graph<NodeWeight, EdgeWeight, Directed, DefaultIx>
where
    NodeWeight: Debug,
    EdgeWeight: Debug,
{
    /// Use the Dot struct to output the graph.
    fn print(&self) -> String {
        format!("{:?}", Dot::new(self))
    }
}
