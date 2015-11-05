use std::borrow::Cow;
use crate::Graph;
use super::*;

pub struct GetNodeIterator<'a, G: Graph> {
    graph: &'a G,
    index: usize,
}

impl <'a, G: Graph> Iterator for GetNodeIterator<'a, G> {
    type Item = Cow<'a, G::Node>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.graph.count_nodes() {
            let index = self.index;
            self.index += 1;
            self.graph.get_node(index)
        } else {
            None
        }
    }
}

pub struct MutNodeIterator<'a, G: Graph> {
    graph: &'a mut G,
    index: usize,
}

impl <'a, G: Graph> Iterator for MutNodeIterator<'a, G> {
    type Item = &'a mut G::Node;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.graph.count_nodes() {
            let index = self.index;
            self.index += 1;
            self.graph.mut_node(index)
        } else {
            None
        }
    }
}