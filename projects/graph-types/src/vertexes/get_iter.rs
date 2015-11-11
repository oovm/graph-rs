use super::*;

#[derive(Debug)]
pub struct GetNodesVisitor<'i, G: Graph + ?Sized> {
    graph: &'i G,
    index: usize,
}

impl<'i, G: Graph> Iterator for GetNodesVisitor<'i, G> {
    type Item = Cow<'i, G::Node>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.graph.count_nodes() {
            let index = self.index;
            self.index += 1;
            self.graph.get_node(index)
        }
        else {
            None
        }
    }
}

impl<'i, G: Graph> DoubleEndedIterator for GetNodesVisitor<'i, G> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index < self.graph.count_nodes() {
            let index = self.graph.count_nodes() - self.index - 1;
            self.index += 1;
            self.graph.get_node(index)
        }
        else {
            None
        }
    }
}

impl<'i, G: Graph + ?Sized> GetNodesVisitor<'i, G> {
    pub fn new(graph: &'i G) -> Self {
        Self { graph, index: 0 }
    }
}