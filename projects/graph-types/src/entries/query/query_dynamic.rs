use crate::{DirectedEdge, GraphEntry, UndirectedEdge};

/// # Arguments
///
/// * `index`:
///
/// returns: Option<Cow<Self::Node>>
///
/// # Examples
///
/// ```
/// use graph_theory::GraphEngine;
/// ```
#[derive(Clone, Copy, Debug)]
pub enum Query {
    NodeID(usize),
    /// No need to remove anything
    EdgeID(usize),
    /// Removed one node, return the node id
    Directed(DirectedEdge),
    /// Removed two nodes, return these node ids
    Undirected(UndirectedEdge),
}

impl Query {
    /// # Arguments
    ///
    /// * `index`:
    ///
    /// returns: Option<Cow<Self::Node>>
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::GraphEngine;
    /// ```
    pub fn as_entry(&self) -> GraphEntry {
        match self {
            Self::NodeID(_) => GraphEntry::Node,
            Self::EdgeID(_) => GraphEntry::Edge,
            Self::Directed(_) => GraphEntry::Edge,
            Self::Undirected(_) => GraphEntry::Edge,
        }
    }
}
