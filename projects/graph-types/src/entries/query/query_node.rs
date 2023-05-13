use crate::Query;

/// Represents a node in a graph
///
/// ```
/// use graph_theory::GraphEngine;
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum NodeQuery {
    NodeID(usize),
}

impl From<NodeQuery> for Query {
    fn from(value: NodeQuery) -> Self {
        match value {
            NodeQuery::NodeID(id) => Query::NodeID(id),
        }
    }
}
