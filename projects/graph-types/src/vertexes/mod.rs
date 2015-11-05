mod simple;
mod indexed;
mod get_iter;

pub use self::indexed::NodeIndex;
pub use self::simple::PureNode;

pub trait Node {
    fn index(&self) -> NodeIndex;
}

pub trait IntoNode {
    type Node: Node;
    fn into_node(self) -> Self::Node;
}




