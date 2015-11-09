use crate::NodeIndex;
use std::{
    cmp::{max, min},
    ops::Range,
};

pub mod directed;
pub mod undirected;

mod simple;

pub trait Edge {
    fn from(&self) -> NodeIndex;
    fn goto(&self) -> NodeIndex;
    fn bidirectional(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeDirection {
    TwoWay,
    Forward,
    Reverse,
}
