use std::borrow::Cow;
use crate::{Edge, Node};


pub trait Graph {
    type Node: Node + Clone;
    type Edge: Edge + Clone;
    fn get_nodes<I>(&self) -> I where I: Iterator<Item=Self::Node>;
    fn mut_nodes<I>(&mut self) -> I where I: Iterator<Item=Self::Node>;
}



