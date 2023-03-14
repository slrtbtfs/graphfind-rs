#[cfg(feature = "svg")]
use graphviz_rust::cmd::{CommandArg, Format};
use petgraph::dot::Dot;
use std::fmt::Debug;
#[cfg(feature = "svg")]
use std::vec;

use crate::graph::print::VizDotGraph;

/// Print implementation for petgraph-type graphs.
///
/// Requires NodeWeight, EdgeWeight to posess the Debug trait.
impl<NodeWeight, EdgeWeight, IndexType, Direction> VizDotGraph<NodeWeight, EdgeWeight>
    for petgraph::graph::Graph<NodeWeight, EdgeWeight, Direction, IndexType>
where
    NodeWeight: Debug,
    EdgeWeight: Debug,
    IndexType: petgraph::graph::IndexType,
    Direction: petgraph::EdgeType,
{
    ///
    /// Use petgraph's Dot struct to output the graph.
    ///
    fn print(&self) -> String {
        format!("{:?}", Dot::new(self))
    }

    /// Use the graphviz-rust to print the given graph into a .svg file.
    ///
    /// Requires a graphviz engine to be installed on the machine that runs
    /// this function.
    ///
    /// Requires the `svg` feature of this crate to be enabled.
    #[cfg(feature = "svg")]
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
