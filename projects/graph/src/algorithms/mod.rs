use fixedbitset::FixedBitSet;
use graph_types::{errors::GraphError, GraphEngine, GraphKind, VisitOrder};
use std::collections::VecDeque;

mod bread_first_search;
mod depth_first_search;
mod topological_search;

pub use self::{
    bread_first_search::{bfs, BreadthFirstSearcher},
    depth_first_search::{dfs, DepthFirstSearcher},
    topological_search::{topological_sort, TopologicalSearcher},
};

/// A connected nodes iterator, contains a large number of node queries, make sure you are using a node-first graph.
#[derive(Debug)]
pub struct ConnectedWalker<'a, G, const MODE: u8>
where
    G: GraphEngine<'a>,
{
    pub(crate) graph: &'a G,
    pub(crate) visited: FixedBitSet,
    pub(crate) queue: VecDeque<u32>,
}

pub struct DirectedAcyclicGraph<'a, G: GraphEngine<'a>> {
    graph: &'a G,
    topological_order: Vec<u32>,
}

impl<'a, G> DirectedAcyclicGraph<'a, G>
where
    G: GraphEngine<'a>,
{
    pub fn new(graph: &'a G) -> Result<Self, GraphError> {
        match graph.graph_kind() {
            GraphKind::Directed => {}
            GraphKind::Undirected => {
                return Err(GraphError::custom("Graph must be directed"));
            }
        }
        // check acyclic
        let mut visited = FixedBitSet::with_capacity(graph.count_nodes());
        let mut queue = VecDeque::new();
        let mut topological_order = Vec::new();
        for node in 0..graph.count_nodes() {
            if visited.contains(node) {
                continue;
            }
            visited.insert(node);
            queue.push_back(node as u32);
            while let Some(node) = queue.pop_front() {
                topological_order.push(node);
                for neighbor in graph.all_outgoing(node as usize) {
                    if visited.contains(neighbor) {
                        return Err(GraphError::custom("Graph must be acyclic"));
                    }
                    visited.insert(neighbor);
                    queue.push_back(neighbor as u32);
                }
            }
        }
        Ok(Self { graph, topological_order })
    }
    pub unsafe fn new_unchecked(graph: &'a G) -> Self {
        Self { graph }
    }
}
