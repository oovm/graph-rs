use super::*;

pub struct MutNodesVisitor<'a, G: Graph> {
    graph: &'a mut G,
    index: usize,
}
