use super::*;

pub type NodeIndex = usize;

impl Node for NodeIndex {
    fn index(&self) -> NodeIndex {
        *self
    }
}