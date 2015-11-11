use super::*;

#[derive(Debug)]
pub struct MutNodesVisitor<'a, G: Graph> {
    graph: &'a mut G,
    index: usize,
}
