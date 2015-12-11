use graph_types::{DictStorage, Edge, EntryName, GraphEngine, GraphData, GraphResult, Query, UndirectedEdge, EdgeRemoveAction, DirectedEdge, DynamicEdge, EdgeDirection, EdgeInsertID};
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

#[doc = include_str!("AdjacencyNodeList.html")]
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
