use std::borrow::Cow;
use crate::{Edge, Node};


pub trait Graph {
    type Node: Node + Clone;
    type Edge: Edge + Clone;
    fn get_nodes<'i, I>(&self) -> I where I: Iterator<Item=Cow<'i, Self::Node>>
}



