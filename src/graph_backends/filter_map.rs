use std::collections::HashMap;

use crate::graph::{self};

///
/// `FilterMap` is a graph representation that is designed to abstractly
/// implement a wide range of possible Queries to a `Graph` object.
///
/// Given an underlying base graph object, it can:
/// * Create a subgraph, filtering out nodes and edges based on a provided
/// condition. Indices remain stable under those transformations.
/// * Apply transformations to the node and edge weights. The new weights can
/// reference the weights of the old graph.
///
/// However it is not possible to change the structure beyond that.
///
/// The most general way to create a `FilterMap` graph is to use the
/// constructor `general_filter_map`, which applies filters and transformations
/// at the same time. For most use cases the simpler, derived constructors
/// that only do part of that might be more appropriate.
///
/// Note that the base graph must outlive the generated FilerMap Graph, since
/// the graph structure is borrowed from the base graph.
///
pub struct FilterMap<
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
    /// Creates a new Graph derived from the base graph, both filtering nodes
    /// and edges and mapping their weights to new values.
    ///
    /// `node_fn` takes a function closure that can either return None, to
    /// remove that node from the derived graph, or `Some(weight)` to keep it
    /// and at the same time equip it with a possibly new value for weight.
    /// `edge_fn` works similarly but with edges.
    ///
    /// By also passing a reference to the base graph into these closures this
    /// allows quite complex graph filtering and mapping. For example the this
    /// can be used to query the adjacent graph elements of the element
    /// currently considered.
    ///
    /// For simpler cases it might be more appropriate to use one of
    /// the derived constructors.
    pub fn general_filter_map<NodeFn, EdgeFn>(
        base_graph: &'g Graph,
        node_fn: NodeFn,
        edge_fn: EdgeFn,
    ) -> Self
    where
        NodeFn: Fn(&'g Graph, Graph::NodeRef) -> Option<NodeWeight>,
        EdgeFn: Fn(&'g Graph, Graph::EdgeRef) -> Option<EdgeWeight>,
    {
        let node_map: HashMap<Graph::NodeRef, NodeWeight> = base_graph
            // take nodes from base graph
            .nodes()
            // apply filter map function
            .filter_map(|n| {
                // in case None is returned by `node_fn` we skip this node due to the `?` operator
                let weight = node_fn(base_graph, n)?;
                // otherwise we return the key, value pair used for constructing the node map
                Some((n, weight))
            })
            // collect into hash table
            .collect();

        let edge_map: HashMap<Graph::EdgeRef, EdgeWeight> = base_graph
            // take edges from base graph
            .edges()
            // filter out edges connected to nodes that don't exist in the new graph
            .filter(|e| {
                let (a, b) = base_graph.adjacent_nodes(*e);
                node_map.contains_key(&a) && node_map.contains_key(&b)
            })
            // apply edge map
            .filter_map(|e| {
                let weight = edge_fn(base_graph, e)?;
                Some((e, weight))
            })
            // collect into hash table
            .collect();

        Self {
            base_graph,
            node_map,
            edge_map,
        }
    }
    /// Creates a new graph derived from the base graph, similarly to `general_filter_map`.
    ///
    /// However, the function closures just take the respective node and edge
    /// weights as arguments, making the constructor less general but more convenient to use.
    pub fn weight_filter_map<NodeFn, EdgeFn>(
        base_graph: &'g Graph,
        node_fn: NodeFn,
        edge_fn: EdgeFn,
    ) -> Self
    where
        NodeFn: Fn(&'g BaseNodeWeight) -> Option<NodeWeight>,
        EdgeFn: Fn(&'g BaseEdgeWeight) -> Option<EdgeWeight>,
        BaseNodeWeight: 'g,
        BaseEdgeWeight: 'g,
    {
        Self::general_filter_map(
            base_graph,
            |_, n| node_fn(base_graph.node_weight(n)),
            |_, e| edge_fn(base_graph.edge_weight(e)),
        )
    }
    /// Creates a new graph derived from the case graph, applying the
    /// respective function to each node and edge weight trough the respective
    /// function, without removing any graph elements.
    pub fn weight_map<NodeFn, EdgeFn>(
        base_graph: &'g Graph,
        node_fn: NodeFn,
        edge_fn: EdgeFn,
    ) -> Self
    where
        NodeFn: Fn(&'g BaseNodeWeight) -> NodeWeight,
        EdgeFn: Fn(&'g BaseEdgeWeight) -> EdgeWeight,
        BaseNodeWeight: 'g,
        BaseEdgeWeight: 'g,
    {
        Self::general_filter_map(
            base_graph,
            |_, n| Some(node_fn(base_graph.node_weight(n))),
            |_, e| Some(edge_fn(base_graph.edge_weight(e))),
        )
    }
}

impl<'g, NodeWeight, EdgeWeight, Graph: graph::Graph<NodeWeight, EdgeWeight>>
    FilterMap<'g, NodeWeight, EdgeWeight, &'g NodeWeight, &'g EdgeWeight, Graph>
{
    /// Creates a new graph derived from the base graph, just filtering out
    /// nodes and edges based on a given condition on their weights.
    ///
    /// Note that, instead of copying the weights into the new graph it adds a
    /// layer of references into the old graph.
    pub fn weight_filter<NodeFn, EdgeFn>(
        base_graph: &'g Graph,
        node_fn: NodeFn,
        edge_fn: EdgeFn,
    ) -> Self
    where
        NodeFn: Fn(&'g NodeWeight) -> bool,
        EdgeFn: Fn(&'g EdgeWeight) -> bool,
        NodeWeight: 'g,
        EdgeWeight: 'g,
    {
        Self::weight_filter_map(
            base_graph,
            |n| {
                if node_fn(n) {
                    Some(n)
                } else {
                    None
                }
            },
            |e| {
                if edge_fn(e) {
                    Some(e)
                } else {
                    None
                }
            },
        )
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
        assert!(self.edge_map.contains_key(&edge));
        let (a, b) = self.base_graph.adjacent_nodes(edge);

        assert!(self.node_map.contains_key(&a));
        assert!(self.node_map.contains_key(&b));

        (a, b)
    }

    fn node_weight(&self, node: Self::NodeRef) -> &NodeWeight {
        assert!(self.node_map.contains_key(&node));
        &self.node_map[&node]
    }

    fn edge_weight(&self, edge: Self::EdgeRef) -> &EdgeWeight {
        assert!(self.edge_map.contains_key(&edge));
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
