pub mod get_iter;
pub mod indexed;
pub mod mut_iter;
pub mod simple;

use crate::Graph;
use std::borrow::Cow;

pub trait Node {
    fn index(&self) -> usize;
}

pub trait IntoNode {
    type Node: Node;
    fn into_node(self) -> Self::Node;
}
