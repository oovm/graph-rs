use crate::{errors::GraphError, NodeID, Query};

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

impl NodeQuery {
    /// Returns the node as a node ID.
    pub fn check_range(index: NodeID, count: usize) -> Result<NodeID, GraphError> {
        if index < count { Ok(index) } else { Err(GraphError::node_out_of_range(index, count)) }
    }
}
