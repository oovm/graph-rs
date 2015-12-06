use graph_types::{DictStorage, Edge, EntryName, GraphEngine, GraphData, GraphResult, Query, UndirectedEdge, EdgeRemoveAction, EdgeInsertAction, DirectedEdge};
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

#[derive(Debug)]
struct NodeNeighbors {
    end_nodes: BTreeMap<EdgeID, EndNodeID>,
}

impl<const TwoWay: bool> Default for AdjacencyNodeList<TwoWay> {
    fn default() -> Self {
        Self {
            head_nodes: BTreeMap::new(),
            last_edge: 0,
        }
    }
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

    fn insert_edge_with_nodes<E>(&mut self, edge: E) -> Vec<usize> where E: Into<EdgeInsertAction> {
        let mut new_edge_ids = Vec::with_capacity(2);
        match edge.into() {
            EdgeInsertAction::Directed(edge) => {
                self.insert_directed_edge(edge, &mut new_edge_ids);
            }
            EdgeInsertAction::Undirected(edge) => {
                self.insert_undirected_edge(edge, &mut new_edge_ids)
            }
            EdgeInsertAction::Grouped(v) => {
                for edge in v {
                    self.insert_edge_with_nodes(edge);
                }
            }
        }
        new_edge_ids
    }

    fn remove_edge<E>(&mut self, edge: E) where E: Into<EdgeRemoveAction> {
        match edge.into() {
            EdgeRemoveAction::EdgeID(v) => {
                let edge_id = v as u32;
                for (_, node) in self.head_nodes.iter_mut() {
                    node.end_nodes.remove(&edge_id);
                    // edge id is unique in the graph
                    break;
                }
            }
            EdgeRemoveAction::Directed(v) => {
                let start_node_id = v.lhs() as u32;
                let end_node_id = v.rhs() as u32;
                if let Some(node) = self.head_nodes.get_mut(&start_node_id) {
                    // notice that there are multiple edges between two nodes
                    node.end_nodes.retain(|_, &mut v| v != end_node_id);
                }
            }
            EdgeRemoveAction::Undirected(v) => {
                panic!("remove undirected edge {v} is not supported in directed graph");
            }
        }
    }
    fn count_edges(&self) -> usize {
        self.head_nodes.iter().map(|(_, v)| v.end_nodes.len()).sum()
    }
}

impl DiGraph {
    pub fn insert_directed_edge(&mut self, edge: DirectedEdge, news: &mut Vec<usize>) {
        let edge_id = self.last_edge;
        self.last_edge += 1;
        let start_node_id = edge.from as u32;
        let end_node_id = edge.goto as u32;
        news.push(edge_id as usize);
        if let Some(node) = self.head_nodes.get_mut(&start_node_id) {
            node.end_nodes.insert(edge_id, end_node_id);
        } else {
            let mut end_nodes = BTreeMap::new();
            end_nodes.insert(edge_id, end_node_id);
            self.head_nodes.insert(start_node_id, NodeNeighbors { end_nodes });
        }
    }
    pub fn insert_undirected_edge(&mut self, edge: UndirectedEdge, news: &mut Vec<usize>) {
        self.insert_directed_edge(DirectedEdge::new(edge.from, edge.goto), news);
        self.insert_directed_edge(DirectedEdge::new(edge.goto, edge.from), news);
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
