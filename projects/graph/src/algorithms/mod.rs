use fixedbitset::FixedBitSet;
use graph_types::GraphEngine;
use std::collections::VecDeque;

pub struct ConnectedSearcher<'a, G, const DepthFirst: bool>
where
    G: GraphEngine,
{
    graph: &'a G,
    visited: FixedBitSet,
    queue: VecDeque<u32>,
}

pub type DFSSearcher<'a, G> = ConnectedSearcher<'a, G, { SearchOrder::DepthFirst.is_dfs() }>;
pub type BFSSearcher<'a, G> = ConnectedSearcher<'a, G, { SearchOrder::BreadthFirst.is_dfs() }>;

#[derive(Clone, Copy, Debug)]
pub enum SearchOrder {
    DepthFirst,
    BreadthFirst,
}

impl SearchOrder {
    pub fn is_dfs(self) -> bool {
        matches!(self, Self::DepthFirst)
    }
    pub fn is_bfs(self) -> bool {
        matches!(self, Self::BreadthFirst)
    }
}

impl<'a, G: GraphEngine> ConnectedSearcher<'a, G> {
    pub fn dfs(graph: &'a G, start_node: usize) -> Self {
        let mut visited = FixedBitSet::with_capacity(graph.node_count());
        visited.insert(start_node);
        let mut queue = VecDeque::new();
        queue.push_back(start_node as u32);
        Self { graph, visited, queue, order: SearchOrder::DepthFirst }
    }
    pub fn bfs(graph: &'a G, start_node: usize) -> Self {
        let mut visited = FixedBitSet::with_capacity(graph.node_count());
        visited.insert(start_node);
        let mut queue = VecDeque::new();
        queue.push_back(start_node as u32);
        Self { graph, visited, queue, order: SearchOrder::BreadthFirst }
    }
}

impl<'a, G: GraphEngine> Iterator for ConnectedSearcher<'a, G> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.queue.pop_front() {
            for edge in self.graph.edges(node) {
                if !self.visited.contains(edge.target as usize) {
                    self.visited.insert(edge.target as usize);
                    self.queue.push_back(edge.target);
                }
            }
            return Some(node as usize);
        }
        None
    }
}
