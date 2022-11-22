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
impl<NodeWeight, EdgeWeight> Graph<NodeWeight, EdgeWeight, NodeIndex, EdgeIndex>
    for petgraph::graph::Graph<NodeWeight, EdgeWeight, Directed, DefaultIx>
{
    fn is_directed(&self) -> bool {
        petgraph::graph::Graph::is_directed(self)
    }

    fn is_directed_edge(&self, _edge: EdgeIndex) -> Option<bool> {
        // petgraph doesn't support mixing directed and undirected edges.
        Some(self.is_directed())
    }

    fn adjacent_edges<'a>(
        &'a self,
        node: &'a NodeIndex,
    ) -> Box<(dyn Iterator<Item = EdgeIndex> + 'a)>
    where
        EdgeIndex: 'a,
    {
        if self.is_directed() {
            Box::new(
                self.edges_directed(*node, Incoming)
                    .chain(self.edges_directed(*node, Outgoing))
                    .map(|e| e.id()),
            )
        } else {
            self.outgoing_edges(node)
        }
    }

    fn incoming_edges<'a>(&'a self, node: &'a NodeIndex) -> Box<dyn Iterator<Item = EdgeIndex> + 'a>
    where
        EdgeIndex: 'a,
    {
        Box::new(self.edges_directed(*node, Incoming).map(|e| e.id()))
    }

    fn outgoing_edges<'a>(&'a self, node: &'a NodeIndex) -> Box<dyn Iterator<Item = EdgeIndex> + 'a>
    where
        EdgeIndex: 'a,
    {
        Box::new(self.edges_directed(*node, Outgoing).map(|e| e.id()))
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

    fn node_weights<'a>(&'a self) -> Box<dyn Iterator<Item = &'a NodeWeight> + 'a> {
        Box::new(petgraph::graph::Graph::node_weights(self))
    }

    fn edge_weights<'a>(&'a self) -> Box<dyn Iterator<Item = &'a EdgeWeight> + 'a> {
        Box::new(petgraph::graph::Graph::edge_weights(self))
    }

    type NodesIterator<'a> = impl Iterator<Item = NodeIndex> + 'a where NodeIndex: 'a, EdgeIndex: 'a, NodeWeight: 'a, EdgeWeight: 'a;
    fn nodes<'a>(&'a self) -> Self::NodesIterator<'a> {
        // This works with the petgraph Graph type due to implementation details of petgraph, see https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html#graph-indices
        let it = (0..self.node_count()).map(NodeIndex::new);

        it
    }

    fn edges<'a>(&'a self) -> Box<dyn Iterator<Item = EdgeIndex> + 'a>
    where
        EdgeIndex: 'a,
    {
        let it = (0..self.edge_count()).map(EdgeIndex::new);

        Box::new(it)
    }
}
