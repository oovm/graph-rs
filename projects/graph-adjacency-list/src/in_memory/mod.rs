use graph_types::{DictStorage, Edge, EntryName, Graph, GraphData, GraphResult, Query, UndirectedEdge};
use std::{borrow::Cow, marker::PhantomData};

/// A vertex first adjacency list
#[derive(Debug)]
pub struct AdjacencyList {
    heads: Vec<AdjacencyNeighbor>,
}

/// start node id is the same as the index in the vector
pub struct AdjacencyNeighbor {
    neighbors: u32,
    end_node_ids: Vec<EdgeID>,
}

/// u64::MAX is reserved for the empty
#[repr(C)]
pub struct EdgeID {
    edge_id: u32,
    end_node: u32,
}

impl EdgeID {
    const EMPTY: Self = Self { edge_id: u32::MAX, end_node: u32::MAX };
}

impl Graph for AdjacencyList {
    type NodeIndex = usize;

    fn count_nodes(&self) -> usize {
        self.heads.len()
    }

    fn insert_edge<E: Edge>(&mut self, edge: E) -> usize {
        let max = edge.max_index();
        if max >= self.heads.len() {
            self.heads.resize(max + 1, Vec::new());
        }
        let lhs = edge.from();
        let rhs = edge.goto();
        if edge.bidirectional() {
            unsafe {
                self.heads.get_unchecked_mut(lhs).push(rhs);
                self.heads.get_unchecked_mut(rhs).push(lhs);
            }
        }
        else {
            unsafe {
                self.heads.get_unchecked_mut(lhs).push(rhs);
            }
        }
        0
    }
    fn count_edges(&self) -> usize {
        self.heads.iter().map(|v| v.len()).sum()
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
