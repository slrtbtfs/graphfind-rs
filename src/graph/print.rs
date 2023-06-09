use crate::graph::Graph;

/// The VizDotGraph trait allows a given Graph to be printed to the GraphViz format.
pub trait VizDotGraph<NodeWeight, EdgeWeight>: Graph<NodeWeight, EdgeWeight> {
    /// Prints the given graph. This function returns a String.
    fn print(&self) -> String;

    /// Displays the given graph as a picture (.svg file).
    /// "path" file specifies the file path to save the picture into.
    ///
    /// The Result contains an error description, if something goes wrong.
    ///
    /// Requires the `svg` feature of this crate to be enabled.
    #[cfg(feature = "svg")]
    fn print_to_svg(&self, path: &str) -> Result<String, std::io::Error>;
}
