use graph_types::{
    DictStorage, EntryName, EntryWeight, Graph, GraphData, GraphError, GraphResult, ListStorage, Query, UndirectedEdge,
};
use std::borrow::Cow;

#[derive(Debug)]
pub struct StableGraph {
    nodes: Vec<usize>,
    edges: Vec<usize>,
}

impl Graph for StableGraph {
    type Node = usize;
    type Edge = UndirectedEdge;

    fn get_node(&self, index: usize) -> Option<Cow<Self::Node>> {
        todo!()
    }

    fn count_nodes(&self) -> usize {
        todo!()
    }

    fn get_edge(&self, index: usize) -> Option<Cow<Self::Edge>> {
        todo!()
    }

    fn count_edges(&self) -> usize {
        todo!()
    }
}

impl GraphData<EntryName> for StableGraph {
    /// Not all node needs a name, so we use a dict storage here.
    type Provider = DictStorage<EntryName>;
}

impl StableGraph {
    pub fn get_node_name<'i>(&self, names: &'i DictStorage<EntryName>) -> GraphResult<&'i EntryName> {
        GraphData::<EntryName>::get_data(self, names, Query::node(0))
    }
    pub fn set_node_name(&self, names: &mut DictStorage<EntryName>, name: EntryName) -> GraphResult<EntryName> {
        GraphData::<EntryName>::mut_data(self, names, Query::node(0), name)
    }
}
