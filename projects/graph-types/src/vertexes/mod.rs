pub mod simple;
pub mod indexed;
pub mod get_iter;
pub mod mut_iter;

use std::borrow::Cow;
use crate::Graph;
use crate::NodeIndex;

pub trait Node {
    fn index(&self) -> NodeIndex;
}

pub trait IntoNode {
    type Node: Node;
    fn into_node(self) -> Self::Node;
}




