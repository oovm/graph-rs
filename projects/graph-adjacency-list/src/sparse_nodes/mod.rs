use graph_types::{DictStorage, Edge, EntryName, GraphEngine, GraphData, GraphResult, Query, UndirectedEdge};
use std::{borrow::Cow, marker::PhantomData};
use std::collections::BTreeMap;

mod di_graph;
mod un_graph;

type EdgeID = u32;
type StartNodeID = u32;
type EndNodeID = u32;

/// Sparse, node first undirected adjacency list based graph
///
/// # Space Complexity
///
/// - O(|V| + |E|)
///
/// # Time Complexity
///
/// - Insert node: O(1)
/// - Insert edge: O(1)
/// - Query node: O(1)
/// - Query edge: O(V)
pub type UnGraph = AdjacencyNodeList<true>;

/// Sparse, node first directed adjacency list based graph
///
/// # Space Complexity
///
/// - O(|V| + 2|E|)
///
/// # Time Complexity
///
/// - Insert node: O(1)
/// - Insert edge: O(1)
/// - Query node: O(1)
/// - Query edge: O(V)
pub type DiGraph = AdjacencyNodeList<false>;

/// Sparse, node first, adjacency list
///
/// # Space Complexity
///
/// - O(|V| + |E|) for undirected graph
/// - O(|V| + 2|E|) for directed graph
///
/// # Node Time Complexity
///
/// This structure has very good performance for nodes
///
/// - Insert: O(1)
/// - Query: O(1)
/// - Removal: O(1)
/// - Count: O(1)
/// - Neighbors: O(1)
///
/// # Edge Time Complexity
///
/// This structure has linear complexity across the edges
///
/// - Insert edge: O(1)
/// - Query edge: O(|V|)
/// - Removal edge: O(|V|)
/// - Count edges: O(|V|)
#[derive(Debug)]
pub struct AdjacencyNodeList<const TwoWay: bool> {
    head_nodes: BTreeMap<StartNodeID, NodeNeighbors>,
    last_edge: EdgeID,
}

struct NodeNeighbors {
    end_nodes: BTreeMap<EdgeID, EndNodeID>,
}

impl GraphEngine for DiGraph {
    fn count_nodes(&self) -> usize {
        self.head_nodes.len()
    }

    fn insert_node(&mut self, node: usize) -> usize {
        let node_id = node as u32;
        self.head_nodes.entry(node_id).or_insert_with(|| NodeNeighbors {
            end_nodes: BTreeMap::new(),
        });
        node
    }
    fn remove_node_with_edges(&mut self, node_id: usize) {
        let node_id = node_id as u32;
        self.head_nodes.remove(&node_id);
    }
    fn insert_edge<E: Edge>(&mut self, edge: E) -> usize {
        let new_edge_id = self.last_edge.saturating_add(1);
        self.last_edge = new_edge_id;
        let start_node_id = edge.from() as u32;
        let end_node_id = edge.goto() as u32;
        let end_nodes = self.head_nodes.entry(start_node_id).or_insert_with(|| NodeNeighbors {
            end_nodes: BTreeMap::new(),
        });
        end_nodes.end_nodes.insert(new_edge_id, end_node_id);
        new_edge_id as usize
    }
    fn count_edges(&self) -> usize {
        self.head_nodes.iter().map(|v| v.len()).sum()
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
