use crate::Graph;

pub struct GetEdgesVisitor<'a, G: Graph + ?Sized> {
    graph: &'a G,
    index: usize,
}

impl<'a, G> GetEdgesVisitor<'a, G>
where
    G: Graph + ?Sized,
{
    pub fn new(graph: &'a G) -> Self {
        Self { graph, index: 0 }
    }
}
