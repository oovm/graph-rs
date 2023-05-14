use super::*;

pub type BreadthFirstSearcher<'a, G> = ConnectedNodes<'a, G, { VisitOrder::BreadthFirst as u8 }>;
pub fn bfs<'a, G>(graph: &'a G, start_node: usize) -> BreadthFirstSearcher<'a, G>
where
    G: GraphEngine<'a>,
{
    BreadthFirstSearcher::new(graph, start_node)
}

impl<'a, G> BreadthFirstSearcher<'a, G>
where
    G: GraphEngine<'a>,
{
    pub fn new(graph: &'a G, start_node: usize) -> Self {
        let mut visited = FixedBitSet::with_capacity(graph.count_nodes());
        visited.insert(start_node);
        let mut queue = VecDeque::new();
        queue.push_back(start_node as u32);
        Self { graph, visited, queue }
    }
}

impl<'a, G> Iterator for BreadthFirstSearcher<'a, G>
where
    G: GraphEngine<'a>,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.queue.pop_front()?;
        for neighbor in self.graph.all_outgoing(node as usize) {
            if !self.visited.contains(neighbor) {
                self.visited.insert(neighbor);
                self.queue.push_back(neighbor as u32);
            }
        }
        Some(node as usize)
    }
}
