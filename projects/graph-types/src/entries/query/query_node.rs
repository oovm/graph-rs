/// Represents a node in a graph
///
/// ```
/// use graph_theory::GraphEngine;
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum NodeQuery {
    NodeID(usize),
}
