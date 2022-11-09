/**
 * Graph is a generic trait specifying the functionality that must be implemented by Graph storage backends used for Querying.
 *
 * Lifetimes:
 * + Graph lifetime is `'g`.
 * + query lifetime is `'q`.
 */

pub trait Graph<NodeWeight, EdgeWeight, NodeRef, EdgeRef>     where NodeWeight : Copy, EdgeWeight: Copy {
    /**
     * Checks if the edges of this graph are directed.
     */
    fn is_directed(&self) -> bool;

    /**
     * Checks if the given edge is directed.
     */
    fn is_directed_edge(&self, edge: EdgeRef) -> Result<bool>;

    /**
     * Gets a readonly handle of all adjacent edges of a node.
     */
    fn adjacent_edges<'a>(&'a self, node: &'a NodeRef) -> Result<Box<dyn Iterator<Item = EdgeRef> + 'a>>;

    /**
     * Checks whether two references refer to the same edge.
     */
    fn do_ref_same_edge(& self, edge1: EdgeRef, edge2: EdgeRef) -> Result<bool>;

    /**
     * Checks whether two references refer to the same node.
     */
    fn do_ref_same_node(& self, node1: NodeRef, node2: NodeRef) -> Result<bool>;

    /**
     * Gets a readonly handle of the nodes an edge connects.
     * If the edge is directed, the first node is its source, and the second node its destination.
     */
    fn adjacent_nodes(& self, node: EdgeRef) -> Result<(NodeRef, NodeRef)>;

    /**
     * Retrieve weight from a node reference.
     */
    fn node_weight(& self, node: NodeRef) -> Result<&NodeWeight>;

    /**
     * Retrieve weight from an edge reference.
     */
    fn edge_weight(& self, edge: EdgeRef) -> Result<&EdgeWeight>;

    /**
     * Returns an Iterator over all node weights.
     */
    fn node_weights<'a>(&'a self) -> Box<dyn Iterator<Item = NodeWeight> + 'a> where NodeWeight: 'a{
        let it= self
            .nodes()
            .map(|x| *self.node_weight(x).unwrap());

        //Box::new(it)
        todo!()
    }

    /**
     * Returns an Iterator over all edge weights.
     */
    fn edge_weights<'a>(&'a self) -> Box<dyn Iterator<Item = EdgeWeight> + 'a> where EdgeWeight: 'a{
        let it = self.edges().map(|x| *self.edge_weight(x).unwrap());
        //Box::new(it)
        todo!()
    }

    /**
     * Returns an Iterator over all nodes.
     */
    fn nodes<'a>(&'a self) -> Box<dyn Iterator<Item = NodeRef> + 'a> where NodeRef: 'a;

    /**
     * Returns an Iterator over all edges.
     */
    fn edges<'a>(&'a self) -> Box<dyn Iterator<Item = EdgeRef> + 'a> where EdgeRef: 'a;
}

pub type Result<T> = std::result::Result<T, String>;
