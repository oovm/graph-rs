use super::*;

/// A topological search is a depth-first search that visits each node's neighbors before visiting the node itself.
pub type TopologicalSearcher<'a, G> = ConnectedWalker<'a, G, { VisitOrder::Topological as u8 }>;

/// Returns a topological search iterator.
pub fn topological_sort<'a, G>(graph: &'a G, start_node: usize) -> TopologicalSearcher<'a, G>
where
    G: GraphEngine<'a>,
{
    TopologicalSearcher::new(graph, start_node)
}

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
