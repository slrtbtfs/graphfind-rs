use std::fmt::Display;

use petgraph::Directed;
use petgraph::graph::{NodeIndex, EdgeIndex, DefaultIx};
use petgraph::dot::Dot;

use crate::print::PrintToGraphVizDot;

/**
 * Print implementation for petgraph-type graphs.
 * 
 * Requires NodeWeight, EdgeWeight to posess the Display trait.
 */
impl<NodeWeight, EdgeWeight> PrintToGraphVizDot<NodeWeight, EdgeWeight, NodeIndex, EdgeIndex>
    // TODO Use petgraph::graph::Graph instead once base graph impl works.
    for petgraph::stable_graph::StableGraph<NodeWeight, EdgeWeight, Directed, DefaultIx> 
where
    NodeWeight: Display,
    EdgeWeight: Display
{
    /// Use the Dot struct to output the graph.
    fn print(&self) -> String {
        format!("{}", Dot::new(self)).to_string()
    }
 }