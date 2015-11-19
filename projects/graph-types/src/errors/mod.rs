use crate::{Entry, Query};
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};
pub type GraphResult<T = ()> = Result<T, GraphError>;

#[derive(Debug)]
pub struct GraphError {
    kind: Box<GraphErrorKind>,
}

#[derive(Clone, Debug)]
pub enum GraphErrorKind {
    NotFound { entry: Entry, index: usize },
    OutOfRange { entry: Entry, index: usize, max: usize },
    Custom { message: String },
}

impl Error for GraphError {}

impl Display for GraphError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}

impl Display for GraphErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GraphErrorKind::NotFound { entry, index } => {
                write!(f, "{entry:?} index {index} not found")
            }
            GraphErrorKind::OutOfRange { entry, index, max } => {
                write!(f, "{entry:?} index {index} is out of range, max index is {max}")
            }
            GraphErrorKind::Custom { message } => f.write_str(message),
        }
    }
}

impl GraphError {
    pub fn custom<S>(message: S) -> Self
    where
        S: ToString,
    {
        Self { kind: Box::new(GraphErrorKind::Custom { message: message.to_string() }) }
    }

    pub fn not_found<Q: Into<Query>>(query: Q) -> Self {
        let query = query.into();
        Self { kind: Box::new(GraphErrorKind::NotFound { entry: query.entry, index: query.index }) }
    }

    pub fn node_out_of_range(index: usize, max: usize) -> Self {
        Self { kind: Box::new(GraphErrorKind::OutOfRange { entry: Entry::Node, index, max }) }
    }
    pub fn edge_out_of_range(index: usize, max: usize) -> Self {
        Self { kind: Box::new(GraphErrorKind::OutOfRange { entry: Entry::Edge, index, max }) }
    }
}
