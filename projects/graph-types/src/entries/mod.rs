pub mod query;

pub mod storages;

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
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum GraphEntry {
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
    Node = 0,
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
    Edge = 1,
}

/// Mark the graph as directed or undirected.
///
/// Generally speaking, a directed graph engine can insert undirected edges,
/// but a undirected graph engine cannot insert directed edges.
///
/// # Examples
///
/// ```
/// use graph_theory::{graph_engines::CycleGraph, GraphEngine, GraphKind};
/// assert_eq!(CycleGraph::one_way(5).graph_kind(), GraphKind::Directed);
/// assert_eq!(CycleGraph::two_way(5).graph_kind(), GraphKind::Undirected);
/// ```
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum GraphKind {
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
    Directed = 0,
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
    Undirected = 1,
}
