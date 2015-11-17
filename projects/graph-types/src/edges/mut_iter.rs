use super::*;

#[derive(Debug)]
pub struct MutEdgesVisitor<'a, G: Graph + ?Sized> {
    graph: &'a mut G,
    index: usize,
}

impl<'a, G> MutEdgesVisitor<'a, G>
where
    G: Graph + ?Sized,
{
    pub fn new(graph: &'a mut G) -> Self {
        Self { graph, index: 0 }
    }
}
