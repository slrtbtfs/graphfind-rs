use petgraph::graph::NodeIndex;
use petgraph::stable_graph::{EdgeIndex, EdgeReference, Edges};
use petgraph::visit::EdgeRef;
use petgraph::{stable_graph::DefaultIx, Directed};

use crate::graph::{Graph, Result};
/**
 * Example implementation for in memory graphs stored using the petgraph library.
 */
impl<NodeWeight, EdgeWeight> Graph<NodeWeight, EdgeWeight, NodeIndex, EdgeIndex>
    for petgraph::stable_graph::StableGraph<NodeWeight, EdgeWeight, Directed, DefaultIx>
where
    NodeWeight: Copy,
    EdgeWeight: Copy,
{
    fn adjacent_edges<'a>(
        &'a self,
        node: &NodeIndex,
    ) -> std::result::Result<Box<(dyn Iterator<Item = EdgeIndex> + 'a)>, String> {
        Ok(Box::new(
            petgraph::stable_graph::StableGraph::edges(self, *node).map(|e| e.id()),
        ))
    }
    /*fn adjacent_edges(&self,  node: &NodeIndex) -> Box<dyn Iterator<Item = & petgraph::stable_graph::EdgeReference<'a, EdgeWeight>> + 'a> {

    }*/

    fn adjacent_nodes(
        &self,
        edge: EdgeIndex,
    ) -> std::result::Result<(NodeIndex, NodeIndex), String> {
        /*self.edge_endpoints(
        edge_index(edge.id().index())).
        expect("Edge Reference invalid.")*/
        todo!();
    }

    fn node_weight(&self, node: NodeIndex) -> std::result::Result<&NodeWeight, String> {
        let found_weight = petgraph::stable_graph::StableGraph::node_weight(self, node);
        found_weight.ok_or(String::from("invalid node reference"))
    }

    fn node_weights<'a>(&'a self) -> Box<dyn Iterator<Item = &'a NodeWeight> + 'a> {
        Box::new(petgraph::stable_graph::StableGraph::node_weights(self))
    }

    fn edge_weights<'a>(&'a self) -> Box<dyn Iterator<Item = &'a EdgeWeight> + 'a> {
        Box::new(petgraph::stable_graph::StableGraph::edge_weights(self))
    }

    fn is_directed(&self) -> bool {
        todo!()
    }

    fn is_directed_edge(&self, edge: EdgeIndex) -> Result<bool> {
        todo!()
    }

    fn do_ref_same_edge(&self, edge1: EdgeIndex, edge2: EdgeIndex) -> Result<bool> {
        todo!()
    }

    fn do_ref_same_node(&self, node1: NodeIndex, node2: NodeIndex) -> Result<bool> {
        todo!()
    }

    fn edge_weight(&self, edge: EdgeIndex) -> Result<&EdgeWeight> {
        todo!()
    }

    fn nodes<'a>(&'a self) -> Box<dyn Iterator<Item = NodeIndex> + 'a>
    where
        NodeIndex: 'a,
    {
        todo!()
    }

    fn edges<'a>(&'a self) -> Box<dyn Iterator<Item = EdgeIndex> + 'a>
    where
        EdgeIndex: 'a,
    {
        todo!()
    }
}
