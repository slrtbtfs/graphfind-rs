use std::fmt::Debug;

use petgraph::Directed;
use petgraph::graph::{NodeIndex, EdgeIndex, DefaultIx};
use petgraph::dot::Dot;

use crate::print::VizDotGraph;

/**
 * Print implementation for petgraph-type graphs.
 * 
 * Requires NodeWeight, EdgeWeight to posess the Debug trait.
 */
impl<NodeWeight, EdgeWeight> VizDotGraph<NodeWeight, EdgeWeight, NodeIndex, EdgeIndex>
    // TODO Use petgraph::graph::Graph instead once base graph impl works.
    for petgraph::stable_graph::StableGraph<NodeWeight, EdgeWeight, Directed, DefaultIx> 
where
    NodeWeight: Debug,
    EdgeWeight: Debug
{
    /// Use the Dot struct to output the graph.
    fn print(&self) -> String {
        format!("{:?}", Dot::new(self))
    }
 }