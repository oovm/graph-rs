
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
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EdgeInsertAction {
    tagged: InsertActions,
}



#[derive(Clone, Debug, PartialEq, Eq)]
enum InsertActions {
    DynamicOne(DynamicEdge),
    DirectedOne(DirectedEdge),
    UndirectedOne(UndirectedEdge),
    DynamicMany(Vec<DynamicEdge>),
    DirectedMany(Vec<DirectedEdge>),
    UndirectedMany(Vec<UndirectedEdge>),
}

impl From<UndirectedEdge> for EdgeInsertAction {
    fn from(edge: UndirectedEdge) -> Self {
        Self { bidi: Some(true), from: edge.from, goto: edge.goto }
    }
}

impl From<DirectedEdge> for EdgeInsertAction {
    fn from(edge: DirectedEdge) -> Self {
        Self { bidi: Some(false), from: edge.from, goto: edge.goto }
    }
}


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
pub enum EdgeRemoveAction {
    EdgeID(usize),
    Directed(DirectedEdge),
    Undirected(UndirectedEdge),
}

impl From<usize> for EdgeRemoveAction {
    fn from(value: usize) -> Self {
        Self::EdgeID(value)
    }
}

impl From<UndirectedEdge> for EdgeRemoveAction {
    fn from(edge: UndirectedEdge) -> Self {
        Self::Undirected(edge)
    }
}

impl From<DirectedEdge> for EdgeRemoveAction {
    fn from(edge: DirectedEdge) -> Self {
        Self::Directed(edge)
    }
}
