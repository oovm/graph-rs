pub type GraphResult<T=()> = Result<T, GraphError>;

pub struct GraphError {
    kind: Box<GraphErrorKind>,
}

pub enum GraphErrorKind {
    NodeNotFound,
    EdgeNotFound,
    NodeAlreadyExists,
    EdgeAlreadyExists,
}