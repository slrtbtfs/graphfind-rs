use std::fmt::Debug;
use std::vec;

use graphviz_rust::cmd::{CommandArg, Format};
use petgraph::dot::Dot;
use petgraph::graph::{DefaultIx};
use petgraph::Directed;

use crate::print::VizDotGraph;

/**
 * Print implementation for petgraph-type graphs.
 *
 * Requires NodeWeight, EdgeWeight to posess the Debug trait.
 */
impl<NodeWeight, EdgeWeight> VizDotGraph<NodeWeight, EdgeWeight>
    for petgraph::graph::Graph<NodeWeight, EdgeWeight, Directed, DefaultIx>
where
    NodeWeight: Debug,
    EdgeWeight: Debug,
{
    ///
    /// Use petgraph's Dot struct to output the graph.
    ///
    fn print(&self) -> String {
        format!("{:?}", Dot::new(self))
    }

    ///
    /// Use the graphviz-rust to print the given graph into a .svg file.
    ///
    /// Requires a graphviz engine to be installed on the machine that runs
    /// this function.
    ///
    fn print_to_svg(&self, path: &str) -> Result<String, std::io::Error> {
        graphviz_rust::exec_dot(
            self.print(),
            vec![
                CommandArg::Format(Format::Svg),
                CommandArg::Output(path.to_string()),
            ],
        )
    }
}
