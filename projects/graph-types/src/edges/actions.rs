use super::*;

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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdgeInsertID {
    /// No need to insert anything
    Nothing,
    /// Inserted one node, return the node id
    OneEdge(usize),
    /// Inserted two nodes, return these node ids
    TwoEdges(usize, usize),
}
