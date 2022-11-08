use petgraph::graph::{NodeIndex};
use petgraph::stable_graph::{EdgeReference,Edges};
use petgraph::{stable_graph::DefaultIx, Directed};

use crate::graph::{Graph,Result};
/**
 * Example implementation for in memory graphs stored using the petgraph library.
 */
impl<'q, NodeWeight, EdgeWeight>
    Graph<'q, NodeWeight, EdgeWeight, &'q NodeIndex, EdgeReference<'q, EdgeWeight>>
    for &'q petgraph::stable_graph::StableGraph<NodeWeight, EdgeWeight, Directed, DefaultIx>
{
    fn adjacent_edges(
        &'q self,
        node: &NodeIndex,
    ) -> std::result::Result<Box<(dyn Iterator<Item =  petgraph::stable_graph::EdgeReference<'q, EdgeWeight>> + 'q)>, String> {
        Ok(Box::new(petgraph::stable_graph::StableGraph::edges(self, *node)))
    }
    /*fn adjacent_edges(&self,  node: &NodeIndex) -> Box<dyn Iterator<Item = & petgraph::stable_graph::EdgeReference<'a, EdgeWeight>> + 'a> {

    }*/

    fn adjacent_nodes(
        &self,
        edge: petgraph::stable_graph::EdgeReference<'q, EdgeWeight>,
    ) -> std::result::Result<(&'q NodeIndex, &'q NodeIndex), String> {
        /*self.edge_endpoints(
        edge_index(edge.id().index())).
        expect("Edge Reference invalid.")*/
        todo!();
    }

    fn node_weight(&self, node: &NodeIndex) -> std::result::Result<&'q NodeWeight, String> {
        let found_weight = petgraph::stable_graph::StableGraph::node_weight(self, *node);
        found_weight.ok_or(String::from("invalid node reference"))
    }

    fn node_weights(&self) -> Box<dyn Iterator<Item = &NodeWeight> + '_> {
        Box::new(petgraph::stable_graph::StableGraph::node_weights(self))
    }

    fn edge_weights(&self) -> Box<dyn Iterator<Item = &EdgeWeight> + '_> {
        Box::new(petgraph::stable_graph::StableGraph::edge_weights(self))
    }

    fn is_directed(&self) -> bool {
        todo!()
    }

    fn is_directed_edge(&self, edge:  EdgeReference<'q, EdgeWeight>) -> Result<bool> {
        todo!()
    }

    fn do_ref_same_edge(&'q self, edge1: EdgeReference<'q, EdgeWeight>, edge2:  EdgeReference<'q, EdgeWeight>) -> Result<bool> {
        todo!()
    }

    fn do_ref_same_node(&'q self, node1: &'q NodeIndex, node2: &'q NodeIndex) -> Result<bool> {
        todo!()
    }

    fn edge_weight(&'q self, edge:  EdgeReference<'q, EdgeWeight>) -> Result<&EdgeWeight> {
        todo!()
    }

    fn nodes(&'q self) -> Box<dyn Iterator<Item = &'q NodeIndex> + 'q> {
        todo!()
    }

    fn edges(&'q self) -> Box<dyn Iterator<Item = EdgeReference<'q, EdgeWeight>> + 'q> {
        todo!()
    }


}
