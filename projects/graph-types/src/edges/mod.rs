use crate::Graph;
use std::{
    cmp::{max, min},
    ops::Range,
};

pub mod directed;
pub mod undirected;

mod simple;
#[cfg(feature = "wolfram_wxf")]
mod wolfram;

pub mod get_iter;
pub mod mut_iter;

pub trait Edge {
    fn from(&self) -> usize;
    fn goto(&self) -> usize;
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
