use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PureNode<M> {
    index: NodeIndex,
    metadata: M,
}

impl<M> Node for PureNode<M> {
    fn index(&self) -> NodeIndex {
        self.index
    }
}

impl<M> PureNode<M> {
    pub fn new(index: NodeIndex, metadata: M) -> Self {
        Self {
            index,
            metadata,
        }
    }
}