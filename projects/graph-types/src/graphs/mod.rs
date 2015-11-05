use std::borrow::Cow;
use crate::{Edge, Node};


pub trait Graph {
    type Node: Node + Clone;
    type Edge: Edge + Clone;

    fn count_nodes(&self) -> usize;
    fn get_node(&self, index: usize) -> Option<Cow<Self::Node>>;
    fn mut_node(&mut self, index: usize) -> Option<&mut Self::Node> {
        unreachable!("The {} graph does not support mutating node `{}`", std::any::type_name::<Self>(), index)
    }
    fn add_node(&mut self, node: Self::Node) -> usize {
        unreachable!("The {} graph does not support adding node `{}`", std::any::type_name::<Self>(), node.index())
    }
    fn remove_node(&mut self, index: usize) -> Option<Self::Node> {
        unreachable!("The {} graph does not support removing node `{}`", std::any::type_name::<Self>(), index)
    }
}
