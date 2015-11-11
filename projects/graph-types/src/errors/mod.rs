use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

pub type GraphResult<T = ()> = Result<T, GraphError>;

#[derive(Debug)]
pub struct GraphError {
    kind: Box<GraphErrorKind>,
}

#[derive(Copy, Clone, Debug)]
pub enum Entry {
    Node,
    Edge,
}

#[derive(Clone, Debug)]
pub enum GraphErrorKind {
    NodeNotFound,
    EdgeNotFound,
    NodeAlreadyExists,
    EdgeAlreadyExists,
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
            GraphErrorKind::NodeNotFound => {
                todo!()
            }
            GraphErrorKind::EdgeNotFound => {
                todo!()
            }
            GraphErrorKind::NodeAlreadyExists => {
                todo!()
            }
            GraphErrorKind::EdgeAlreadyExists => {
                todo!()
            }
            GraphErrorKind::OutOfRange { entry, index, max } => match entry {
                Entry::Node => {
                    write!(f, "Node index {} is out of range, max index is {}", index, max)
                }
                Entry::Edge => {
                    write!(f, "Edge index {} is out of range, max index is {}", index, max)
                }
            },
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
    pub fn node_out_of_range(index: usize, max: usize) -> Self {
        Self { kind: Box::new(GraphErrorKind::OutOfRange { entry: Entry::Node, index, max }) }
    }
    pub fn edge_out_of_range(index: usize, max: usize) -> Self {
        Self { kind: Box::new(GraphErrorKind::OutOfRange { entry: Entry::Edge, index, max }) }
    }
}
