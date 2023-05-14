use fixedbitset::FixedBitSet;
use graph_types::{GraphEngine, VisitOrder};
use std::collections::VecDeque;
mod breadth_first_search;
mod depth_first_search;
mod topological_sort;

pub use self::{
    breadth_first_search::{bfs, BreadthFirstSearcher},
    depth_first_search::{dfs, DepthFirstSearcher},
};

#[derive(Debug)]
pub struct ConnectedNodes<'a, G, const MODE: u8>
where
    G: GraphEngine<'a>,
{
    graph: &'a G,
    visited: FixedBitSet,
    queue: VecDeque<u32>,
}

pub type TopologicalSearcher<'a, G> = ConnectedNodes<'a, G, { VisitOrder::Topological as u8 }>;

impl<'a, G> Iterator for TopologicalSearcher<'a, G>
where
    G: GraphEngine<'a>,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.queue.pop_front() {
            if !self.visited.put(node as usize) {
                self.queue.extend(self.graph.neighbors(node));
                return Some(node as usize);
            }
        }
        None
    }
}
