use petgraph::graph::NodeIndex;
use petgraph::graph::{EdgeIndex, EdgeReference, Edges};
use petgraph::visit::EdgeRef;
use petgraph::{graph::DefaultIx, Directed};

use crate::graph::{Graph, Result};
/**
 * Example implementation for in memory graphs stored using the petgraph library.
 */
impl<NodeWeight, EdgeWeight> Graph<NodeWeight, EdgeWeight, NodeIndex, EdgeIndex>
    for petgraph::graph::Graph<NodeWeight, EdgeWeight, Directed, DefaultIx>
{
    fn adjacent_edges<'a>(
        &'a self,
        node: &NodeIndex,
    ) -> std::result::Result<Box<(dyn Iterator<Item = EdgeIndex> + 'a)>, String> {
        Ok(Box::new(
            petgraph::graph::Graph::edges(self, *node).map(|e| e.id()),
        ))
    }
    /*fn adjacent_edges(&self,  node: &NodeIndex) -> Box<dyn Iterator<Item = & petgraph::graph::EdgeReference<'a, EdgeWeight>> + 'a> {

    fn adjacent_nodes(&self, edge: EdgeIndex) -> Option<(NodeIndex, NodeIndex)> {
        self.edge_endpoints(edge)
    }

    fn node_weight(&self, node: NodeIndex) -> std::result::Result<&NodeWeight, String> {
        let found_weight = petgraph::graph::Graph::node_weight(self, node);
        found_weight.ok_or(String::from("invalid node reference"))
    }

    fn node_weights<'a>(&'a self) -> Box<dyn Iterator<Item = &'a NodeWeight> + 'a> {
        Box::new(petgraph::graph::Graph::node_weights(self))
    }

    fn edge_weights<'a>(&'a self) -> Box<dyn Iterator<Item = &'a EdgeWeight> + 'a> {
        Box::new(petgraph::graph::Graph::edge_weights(self))
    }

    fn is_directed(&self) -> bool {
        petgraph::graph::Graph::is_directed(self)
    }

    fn is_directed_edge(&self, edge: EdgeIndex) -> Result<bool> {
        petgraph::graph::Graph::is_directed_edge(self, edge)
    }

    fn do_ref_same_edge(&self, edge1: EdgeIndex, edge2: EdgeIndex) -> bool {
        edge1 == edge2
    }

    fn do_ref_same_node(&self, node1: NodeIndex, node2: NodeIndex) -> bool {
        node1 == node2
    }

    fn edge_weight(&self, edge: EdgeIndex) -> Option<&EdgeWeight> {
        petgraph::graph::Graph::edge_weight(self, edge)
    }

    fn nodes<'a>(&'a self) -> Box<dyn Iterator<Item = NodeIndex> + 'a>
    where
        NodeIndex: 'a,
    {
        // This works with the petgraph Graph type due to implementation details of petgraph, see https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html#graph-indices
        let it = (0..self.node_count()).map(NodeIndex::new);

        Box::new(it)
    }

    fn edges<'a>(&'a self) -> Box<dyn Iterator<Item = EdgeIndex> + 'a>
    where
        EdgeIndex: 'a,
    {
        let it = (0..self.edge_count()).map(EdgeIndex::new);

        Box::new(it)
    }
}
