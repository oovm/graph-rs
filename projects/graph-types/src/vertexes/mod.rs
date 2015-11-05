pub mod simple;

pub trait Node {
    fn index(&self) -> NodeIndex;
}

pub trait IntoNode {
    type Node: Node;
    fn into_node(self) -> Self::Node;
}


pub type NodeIndex = usize;

