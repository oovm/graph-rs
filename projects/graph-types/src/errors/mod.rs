use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub type GraphResult<T = ()> = Result<T, GraphError>;

#[derive(Debug)]
pub struct GraphError {
    kind: Box<GraphErrorKind>,
}

#[derive(Debug)]
pub enum Entry {
    Node,
    Edge,
}

#[derive(Debug)]
pub enum GraphErrorKind {
    NodeNotFound,
    EdgeNotFound,
    NodeAlreadyExists,
    EdgeAlreadyExists,
    OutOfRange {
        entry: Entry,
        index: usize,
        max: usize,
    },
}

impl Display for GraphError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for GraphError {}

impl GraphError {
    pub fn node_out_of_range(index: usize, max: usize) -> Self {
        Self {
            kind: Box::new(GraphErrorKind::OutOfRange {
                entry: Entry::Node,
                index,
                max,
            }),
        }
    }
    pub fn edge_out_of_range(index: usize, max: usize) -> Self {
        Self {
            kind: Box::new(GraphErrorKind::OutOfRange {
                entry: Entry::Edge,
                index,
                max,
            }),
        }
    }
}