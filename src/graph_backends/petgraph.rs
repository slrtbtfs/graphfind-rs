use petgraph::{stable_graph::DefaultIx};

use crate::graph::Graph;
/**
 * Example implementation for in memory graphs stored using the petgraph library.
 */
impl<NodeWeight, EdgeWeight> Graph<NodeWeight,EdgeWeight,DefaultIx,DefaultIx> for petgraph::graph::Graph<NodeWeight, EdgeWeight> {

    fn node_weights(&self) -> Box<dyn Iterator<Item = &NodeWeight> + '_> {
        Box::new(self.node_weights())
    }

    fn edge_weights(&self) -> Box<dyn Iterator<Item = &EdgeWeight> + '_> {
        Box::new(self.edge_weights())
    }

    fn adjacent_edges(&self,  node: &DefaultIx) -> Box<dyn Iterator<Item = DefaultIx> + '_> {
        todo!()
    }

    fn adjacent_nodes(&self,  node: &DefaultIx) -> (DefaultIx,DefaultIx) {
        todo!()
    }

    fn node_weight(&self, node: &DefaultIx) -> NodeWeight {
        todo!()
    }

    fn edge_weight(&self, edge: &DefaultIx) -> EdgeWeight {
        todo!()
    }
}