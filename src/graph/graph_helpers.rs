use super::Graph;

///
/// Convenience Method to directly access the predecessor nodes of node `n` in Graph `g`.
/// Returns an iterator over these nodes.
///
pub fn incoming_nodes<'a, G, N, NW: 'a, EW: 'a>(g: &'a G, n: N) -> impl Iterator<Item = N> + 'a
where
    G: Graph<NW, EW, NodeRef = N>,
{
    g.incoming_edges(n).map(|e| g.adjacent_nodes(e).0)
}

///
/// Convenience Method to directly access the successor nodes of node `n` in Graph `g`.
/// Returns an iterator over these nodes.
///
pub fn outgoing_nodes<'a, G, N, NW: 'a, EW: 'a>(g: &'a G, n: N) -> impl Iterator<Item = N> + 'a
where
    G: Graph<NW, EW, NodeRef = N>,
{
    g.outgoing_edges(n).map(|e| g.adjacent_nodes(e).1)
}
