use petgraph::graph::EdgeIndex;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::Direction::Incoming;
use petgraph::Direction::Outgoing;
use petgraph::{graph::DefaultIx, Directed};

use crate::graph::Graph;
/**
 * Example implementation for in memory graphs stored using the petgraph library.
 */
impl<NodeWeight, EdgeWeight> Graph<NodeWeight, EdgeWeight>
    for petgraph::graph::Graph<NodeWeight, EdgeWeight, Directed, DefaultIx>
{
    type NodeRef = NodeIndex;
    type EdgeRef = EdgeIndex;
    fn is_directed(&self) -> bool {
        petgraph::graph::Graph::is_directed(self)
    }

    fn is_directed_edge(&self, _edge: EdgeIndex) -> Option<bool> {
        // petgraph doesn't support mixing directed and undirected edges.
        Some(self.is_directed())
    }
    type AdjacentEdgesIterator<'a> = impl Iterator<Item = EdgeIndex> + 'a where Self: 'a;
    fn adjacent_edges(&self, node: Self::NodeRef) -> Self::AdjacentEdgesIterator<'_> {
        self.edges_directed(node, Incoming)
            .chain(
                self.edges_directed(node, Outgoing)
                    .filter(|_| self.is_directed()),
            )
            .map(|e| e.id())
    }

    type IncomingEdgesIterator<'a> = impl Iterator<Item = EdgeIndex> + 'a where Self: 'a;
    fn incoming_edges(&self, node: Self::NodeRef) -> Self::IncomingEdgesIterator<'_> {
        Box::new(self.edges_directed(node, Incoming).map(|e| e.id()))
    }

    type OutgoingEdgesIterator<'a> = impl Iterator<Item = EdgeIndex> + 'a where Self: 'a;
    fn outgoing_edges(&self, node: Self::NodeRef) -> Self::OutgoingEdgesIterator<'_> {
        Box::new(self.edges_directed(node, Outgoing).map(|e| e.id()))
    }

    fn do_ref_same_edge(&self, edge1: EdgeIndex, edge2: EdgeIndex) -> bool {
        edge1 == edge2
    }

    fn do_ref_same_node(&self, node1: NodeIndex, node2: NodeIndex) -> bool {
        node1 == node2
    }

    fn adjacent_nodes(&self, edge: EdgeIndex) -> Option<(NodeIndex, NodeIndex)> {
        self.edge_endpoints(edge)
    }

    fn node_weight(&self, node: NodeIndex) -> Option<&NodeWeight> {
        petgraph::graph::Graph::node_weight(self, node)
    }

    fn edge_weight(&self, edge: EdgeIndex) -> Option<&EdgeWeight> {
        petgraph::graph::Graph::edge_weight(self, edge)
    }

    fn node_weights(&self) -> Self::NodeWeightsIterator<'_> {
        petgraph::graph::Graph::node_weights(self)
    }

    fn edge_weights(&self) -> Self::EdgeWeightsIterator<'_> {
        petgraph::graph::Graph::edge_weights(self)
    }

    type NodesIterator<'a> = impl Iterator<Item = NodeIndex> + 'a where Self: 'a;
    fn nodes(&self) -> Self::NodesIterator<'_> {
        // This works with the petgraph Graph type due to implementation details of petgraph, see https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html#graph-indices
        (0..self.node_count()).map(NodeIndex::new)
    }

    type EdgesIterator<'a> = impl Iterator<Item = EdgeIndex> + 'a where Self: 'a;
    fn edges(&self) -> Self::EdgesIterator<'_> {
        (0..self.edge_count()).map(EdgeIndex::new)
    }

    type NodeWeightsIterator<'a>
     = impl Iterator<Item = &'a NodeWeight> + 'a where Self: 'a, Self: 'a, NodeWeight: 'a;

    type EdgeWeightsIterator<'a>
     = impl Iterator<Item = &'a EdgeWeight> + 'a where Self: 'a, Self: 'a, EdgeWeight: 'a;
}
