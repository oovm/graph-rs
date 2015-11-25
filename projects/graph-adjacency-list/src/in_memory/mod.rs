use graph_types::{DictStorage, EntryName, Graph, GraphData, GraphResult, Query, UndirectedEdge};
use std::borrow::Cow;

#[derive(Debug)]
pub struct StableGraph {
    nodes: Vec<usize>,
    edges: Vec<usize>,
}

impl Graph for StableGraph {
    type NodeIndex = usize;
    type EdgeIndex = UndirectedEdge;

    fn get_node_id(&self, index: usize) -> Option<Cow<Self::NodeIndex>> {
        todo!()
    }

    fn count_nodes(&self) -> usize {
        todo!()
    }

    fn get_edge(&self, index: usize) -> Option<Cow<Self::EdgeIndex>> {
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
