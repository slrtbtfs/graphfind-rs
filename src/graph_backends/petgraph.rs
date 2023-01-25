use petgraph::graph::EdgeIndex;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::Direction::Incoming;
use petgraph::Direction::Outgoing;

use crate::graph::Graph;
///
/// Example implementation for in memory graphs stored using the petgraph library.
///
/// Both undirected and directed graphs are supported.
///
/// Node and Edge indices are by default 32-bit unsigned integers. Valid indices are
/// {0, ..., n - 1} and {0, ..., m - 1}, where n is the number of nodes, and m the
/// number of edges stored in this graph implementation.
///
impl<NodeWeight, EdgeWeight, Direction, IndexType> Graph<NodeWeight, EdgeWeight>
    for petgraph::graph::Graph<NodeWeight, EdgeWeight, Direction, IndexType>
where
    IndexType: petgraph::graph::IndexType,
    Direction: petgraph::EdgeType,
{
    type NodeRef = NodeIndex<IndexType>;
    type EdgeRef = EdgeIndex<IndexType>;
    fn is_directed(&self) -> bool {
        petgraph::graph::Graph::is_directed(self)
    }

    fn is_directed_edge(&self, edge: Self::EdgeRef) -> bool {
        assert!(edge.index() < self.edge_count());
        // petgraph doesn't support mixing directed and undirected edges.
        self.is_directed()
    }

    type AdjacentEdgesIterator<'a> = impl Iterator<Item = Self::EdgeRef> + 'a where Self: 'a;
    fn adjacent_edges(&self, node: Self::NodeRef) -> Self::AdjacentEdgesIterator<'_> {
        self.edges_directed(node, Incoming)
            .chain(
                self.edges_directed(node, Outgoing)
                    .filter(|_| self.is_directed()),
            )
            .map(|e| e.id())
    }

    type IncomingEdgesIterator<'a> = impl Iterator<Item = Self::EdgeRef> + 'a where Self: 'a;
    fn incoming_edges(&self, node: Self::NodeRef) -> Self::IncomingEdgesIterator<'_> {
        Box::new(self.edges_directed(node, Incoming).map(|e| e.id()))
    }

    type OutgoingEdgesIterator<'a> = impl Iterator<Item = Self::EdgeRef> + 'a where Self: 'a;
    fn outgoing_edges(&self, node: Self::NodeRef) -> Self::OutgoingEdgesIterator<'_> {
        Box::new(self.edges_directed(node, Outgoing).map(|e| e.id()))
    }

    fn adjacent_nodes(&self, edge: Self::EdgeRef) -> (Self::NodeRef, Self::NodeRef) {
        self.edge_endpoints(edge)
            .expect("Couldn't find edge endpoint references: Edge reference invalid.")
    }

    fn node_weight(&self, node: Self::NodeRef) -> &NodeWeight {
        petgraph::graph::Graph::node_weight(self, node)
            .expect("Couldn't find node weight: Node reference invalid.")
    }

    fn edge_weight(&self, edge: Self::EdgeRef) -> &EdgeWeight {
        petgraph::graph::Graph::edge_weight(self, edge)
            .expect("Couldn't find edge weight: Edge reference invalid.")
    }

    fn node_weights(&self) -> Self::NodeWeightsIterator<'_> {
        petgraph::graph::Graph::node_weights(self)
    }

    fn edge_weights(&self) -> Self::EdgeWeightsIterator<'_> {
        petgraph::graph::Graph::edge_weights(self)
    }

    type NodesIterator<'a> = impl Iterator<Item = Self::NodeRef> + 'a where Self: 'a;
    fn nodes(&self) -> Self::NodesIterator<'_> {
        // This works with the petgraph Graph type due to implementation details of petgraph, see https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html#graph-indices
        (0..self.node_count()).map(NodeIndex::new)
    }

    type EdgesIterator<'a> = impl Iterator<Item = Self::EdgeRef> + 'a where Self: 'a;
    fn edges(&self) -> Self::EdgesIterator<'_> {
        (0..self.edge_count()).map(EdgeIndex::new)
    }

    type NodeWeightsIterator<'a>
     = impl Iterator<Item = &'a NodeWeight> + 'a where Self: 'a, Self: 'a, NodeWeight: 'a;

    type EdgeWeightsIterator<'a>
     = impl Iterator<Item = &'a EdgeWeight> + 'a where Self: 'a, Self: 'a, EdgeWeight: 'a;

    fn count_edges(&self) -> usize {
        self.edge_count()
    }

    fn count_nodes(&self) -> usize {
        self.node_count()
    }
}
