use std::fmt::Pointer;
use std::ops::Deref;
use petgraph::{Directed, stable_graph::DefaultIx};
use petgraph::csr::EdgeIndex;
use petgraph::graph::{edge_index, NodeIndex};
use petgraph::prelude::EdgeRef;
use petgraph::stable_graph::EdgeReference;

use crate::graph::Graph;
/**
 * Example implementation for in memory graphs stored using the petgraph library.
 */
impl<'a, NodeWeight, EdgeWeight> Graph<NodeWeight,EdgeWeight,NodeIndex,EdgeReference<'a, EdgeWeight>> for &'a petgraph::stable_graph::StableGraph<NodeWeight, EdgeWeight, Directed, DefaultIx> {
    fn adjacent_edges(&self,  node: &NodeIndex) -> Box<dyn Iterator<Item = EdgeReference<'a, EdgeWeight>> + '_> {
        Box::new( self.edges(*node))
    }

    fn adjacent_nodes(&self,  edge: &petgraph::stable_graph::EdgeReference<'a, EdgeWeight>) -> (NodeIndex,NodeIndex) {
        self.edge_endpoints(
            edge_index(edge.id().index())).
            expect("Edge Reference invalid.")
    }

    fn node_weight(&self, node: &NodeIndex) -> NodeWeight {
        let found_weight = petgraph::stable_graph::StableGraph::node_weight(self,*node);
        found_weight.expect("Node Index invalid.");
        todo!()
    }

    fn edge_weight(&self, edge: &petgraph::stable_graph::EdgeReference<'a, EdgeWeight>) -> EdgeWeight {
        let x =
            petgraph::stable_graph::StableGraph::edge_weight(self, edge_index(edge.id().index()));
        x.expect("Edge Reference invalid");
        todo!()
    }

    fn node_weights(&self) -> Box<dyn Iterator<Item = &NodeWeight> + '_> {
        Box::new(petgraph::stable_graph::StableGraph::node_weights(self))
    }

    fn edge_weights(&self) -> Box<dyn Iterator<Item = &EdgeWeight> + '_> {
        Box::new(petgraph::stable_graph::StableGraph::edge_weights(self))
    }
}