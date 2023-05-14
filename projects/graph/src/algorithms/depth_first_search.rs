use super::*;
pub type DepthFirstSearcher<'a, G> = ConnectedNodes<'a, G, { VisitOrder::DepthFirst as u8 }>;

pub fn dfs<'a, G>(graph: &'a G, start_node: usize) -> DepthFirstSearcher<'a, G>
where
    G: GraphEngine<'a>,
{
    DepthFirstSearcher::new(graph, start_node)
}

impl<'a, G> DepthFirstSearcher<'a, G>
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

impl<'a, G> Iterator for DepthFirstSearcher<'a, G>
where
    G: GraphEngine<'a>,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.queue.pop_back()?;
        for neighbor in self.graph.all_outgoing(node as usize) {
            if !self.visited.contains(neighbor) {
                self.visited.insert(neighbor);
                self.queue.push_back(neighbor as u32);
            }
        }
        Some(node as usize)
    }
}
