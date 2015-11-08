use std::borrow::Cow;
use crate::{Edge, GetNodesVisitor, Node, NodeIndex};


pub trait Graph {
    type Node: Node + Clone;
    type Edge: Edge + Clone;

    fn count_nodes(&self) -> usize;
    fn get_node(&self, index: NodeIndex) -> Option<Cow<Self::Node>>;
    fn get_nodes(&self) -> GetNodesVisitor<Self> {
        GetNodesVisitor::new(self)
    }
    fn mut_node(&mut self, index: NodeIndex) -> Option<&mut Self::Node> {
        unreachable!("The {} graph does not support mutating node `{}`", std::any::type_name::<Self>(), index)
    }
    fn insert_node(&mut self, node: Self::Node) -> NodeIndex {
        unreachable!("The {} graph does not support adding node `{}`", std::any::type_name::<Self>(), node.index())
    }
    fn remove_node(&mut self, index: NodeIndex) -> Option<Self::Node> {
        unreachable!("The {} graph does not support removing node `{}`", std::any::type_name::<Self>(), index)
    }
}
