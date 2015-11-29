use graph_types::{DictStorage, EntryName, Graph, GraphData, GraphResult, Query, UndirectedEdge};
use std::borrow::Cow;

#[derive(Debug)]
pub struct AdjacencyList {
    heads: Vec<Vec<usize>>,
}

impl Graph for AdjacencyList {
    type NodeIndex = usize;

    fn get_node(&self, index: Self::NodeIndex) -> Option<usize> {
        todo!()
    }

    fn count_nodes(&self) -> usize {
        self.heads.len()
    }

    fn get_edge(&self, index: usize) -> Option<usize> {
        todo!()
    }

    fn count_edges(&self) -> usize {
        todo!()
    }
}
// impl GraphData<EntryName> for AdjacencyList {
//     /// Not all node needs a name, so we use a dict storage here.
//     type Provider = DictStorage<EntryName>;
// }
//
// impl AdjacencyList {
//     pub fn get_node_name<'i>(&self, names: &'i DictStorage<EntryName>) -> GraphResult<&'i EntryName> {
//         GraphData::<EntryName>::get_data(self, names, Query::node(0))
//     }
//     pub fn set_node_name(&self, names: &mut DictStorage<EntryName>, name: EntryName) -> GraphResult<EntryName> {
//         GraphData::<EntryName>::mut_data(self, names, Query::node(0), name)
//     }
// }
