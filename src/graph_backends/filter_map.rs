use std::collections::HashMap;

use crate::graph::{self};

struct FilterMap<
    'g,
    BaseNodeWeight,
    BaseEdgeWeight,
    NodeWeight,
    EdgeWeight,
    Graph: graph::Graph<BaseNodeWeight, BaseEdgeWeight>,
> {
    base_graph: &'g Graph,
    node_map: HashMap<Graph::NodeRef, NodeWeight>,
    edge_map: HashMap<Graph::EdgeRef, EdgeWeight>,
}

impl<
        'g,
        BaseNodeWeight,
        BaseEdgeWeight,
        NodeWeight,
        EdgeWeight,
        Graph: graph::Graph<BaseNodeWeight, BaseEdgeWeight>,
    > FilterMap<'g, BaseNodeWeight, BaseEdgeWeight, NodeWeight, EdgeWeight, Graph>
{
    fn new<NodeFn, EdgeFn>(graph: &'g Graph, node_fn: NodeFn, edge_fn: EdgeFn) -> Self
    where
        NodeFn: Fn(&'g Graph, Graph::NodeRef) -> Option<NodeWeight>,
        EdgeFn: Fn(&'g Graph, Graph::EdgeRef) -> Option<EdgeWeight>,
    {
        let node_map: HashMap<Graph::NodeRef, NodeWeight> = graph
            .nodes()
            .filter_map(|n| {
                let weight = node_fn(graph, n)?;
                Some((n, weight))
            })
            .collect();

        let edge_map: HashMap<Graph::EdgeRef, EdgeWeight> = graph
            .edges()
            .filter(|e| {
                let (a, b) = graph.adjacent_nodes(*e);
                node_map.contains_key(&a) && node_map.contains_key(&b)
            })
            .filter_map(|e| {
                let weight = edge_fn(graph, e)?;
                Some((e, weight))
            })
            .collect();

        Self {
            base_graph: graph,
            node_map,
            edge_map,
        }
    }
}

impl<
        'g,
        BaseNodeWeight,
        BaseEdgeWeight,
        NodeWeight,
        EdgeWeight,
        Graph: graph::Graph<BaseNodeWeight, BaseEdgeWeight>,
    > graph::Graph<NodeWeight, EdgeWeight>
    for FilterMap<'g, BaseNodeWeight, BaseEdgeWeight, NodeWeight, EdgeWeight, Graph>
{
    type NodeRef = Graph::NodeRef;

    type EdgeRef = Graph::EdgeRef;

    fn is_directed(&self) -> bool {
        self.base_graph.is_directed()
    }

    fn is_directed_edge(&self, edge: Self::EdgeRef) -> bool {
        assert!(self.edge_map.contains_key(&edge));
        self.base_graph.is_directed_edge(edge)
    }

    type AdjacentEdgesIterator<'a>
     = impl Iterator<Item = Graph::EdgeRef>  + 'a where Self: 'a;

    fn adjacent_edges(&self, node: Self::NodeRef) -> Self::AdjacentEdgesIterator<'_> {
        assert!(self.node_map.contains_key(&node));
        self.base_graph
            .adjacent_edges(node)
            .filter(|e| self.edge_map.contains_key(e))
    }

    type IncomingEdgesIterator<'a> = impl Iterator<Item = Graph::EdgeRef>  + 'a where Self: 'a;

    fn incoming_edges(&self, node: Self::NodeRef) -> Self::IncomingEdgesIterator<'_> {
        assert!(self.node_map.contains_key(&node));
        self.base_graph
            .incoming_edges(node)
            .filter(|e| self.edge_map.contains_key(e))
    }

    type OutgoingEdgesIterator<'a> = impl Iterator<Item = Graph::EdgeRef>  + 'a where Self: 'a;

    fn outgoing_edges(&self, node: Self::NodeRef) -> Self::OutgoingEdgesIterator<'_> {
        assert!(self.node_map.contains_key(&node));
        self.base_graph
            .outgoing_edges(node)
            .filter(|e| self.edge_map.contains_key(e))
    }

    fn adjacent_nodes(&self, edge: Self::EdgeRef) -> (Self::NodeRef, Self::NodeRef) {
        let (a, b) = self.base_graph.adjacent_nodes(edge);

        assert!(self.node_map.contains_key(&a));
        assert!(self.node_map.contains_key(&a));

        (a, b)
    }

    fn node_weight(&self, node: Self::NodeRef) -> &NodeWeight {
        &self.node_map[&node]
    }

    fn edge_weight(&self, edge: Self::EdgeRef) -> &EdgeWeight {
        &self.edge_map[&edge]
    }

    type NodeWeightsIterator<'a> = impl Iterator<Item =&'a NodeWeight>
    where
        Self: 'a,
        NodeWeight: 'a;

    fn node_weights(&self) -> Self::NodeWeightsIterator<'_> {
        self.node_map.values()
    }

    type EdgeWeightsIterator<'a> = impl Iterator<Item =&'a EdgeWeight>
    where
        Self: 'a,
        EdgeWeight: 'a;

    fn edge_weights(&self) -> Self::EdgeWeightsIterator<'_> {
        self.edge_map.values()
    }

    type NodesIterator<'a> = impl Iterator<Item = Graph::NodeRef> + 'a
    where
        Self: 'a;

    fn nodes(&self) -> Self::NodesIterator<'_> {
        self.node_map.keys().copied()
    }

    type EdgesIterator<'a> = impl Iterator<Item = Graph::EdgeRef> + 'a
    where
        Self: 'a;

    fn edges(&self) -> Self::EdgesIterator<'_> {
        self.edge_map.keys().copied()
    }
}
