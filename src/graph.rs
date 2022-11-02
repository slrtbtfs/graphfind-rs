trait  Graph<'a>  {
    type NodeType;
    type EdgeType;

    fn get_nodes(&self) -> Box<dyn Iterator<Item = &'a Self::NodeType>>;
    fn get_edges(&self) -> Box<dyn Iterator<Item = &'a Self::EdgeType>>;
}
