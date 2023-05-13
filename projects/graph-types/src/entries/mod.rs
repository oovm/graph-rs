use crate::{errors::GraphError, EdgeQuery, Query};
use std::ops::{Deref, DerefMut};

pub mod query;

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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl GraphKind {
    pub const fn is_one_way(&self) -> bool {
        matches!(self, GraphKind::Directed)
    }
    pub const fn is_two_way(&self) -> bool {
        matches!(self, GraphKind::Undirected)
    }
}

pub trait EntryEngine<'i, V> {
    type EntryRef: Deref<Target = V>;
    type EntryMut: DerefMut<Target = V>;

    /// Get the reference of data by given query from the storage.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::GraphEngine;
    /// ```
    fn get_entry<Q: Into<Query>>(&'i self, query: Q) -> V;
    fn try_entry<Q: Into<Query>>(&'i self, query: Q) -> Result<Self::EntryRef, GraphError>;
    fn mut_entry<Q: Into<Query>>(&'i mut self, query: Q) -> Result<Self::EntryMut, GraphError>;
    fn set_entry<Q: Into<Query>>(&'i mut self, query: Q, entry: V) -> Result<(), GraphError> {
        let mut entry_ref = self.mut_entry(query)?;
        *entry_ref.deref_mut() = entry;
        Ok(())
    }
    fn get_node_data(&'i self, node: usize) -> V {
        self.get_entry(Query::NodeID(node))
    }
    fn try_node_data(&'i self, node: usize) -> Option<Self::EntryRef> {
        self.try_entry(Query::NodeID(node)).ok()
    }
    fn mut_node_data(&'i mut self, node: usize) -> Option<Self::EntryMut> {
        self.mut_entry(Query::NodeID(node)).ok()
    }
    fn set_node_data(&'i mut self, node: usize, data: V) {
        self.set_entry(Query::NodeID(node), data).ok();
    }
    fn get_edge_data<Q: Into<EdgeQuery>>(&'i self, edge: Q) -> V {
        self.get_entry(Query::from(edge.into()))
    }
    fn try_edge_data<Q: Into<EdgeQuery>>(&'i self, edge: Q) -> Option<Self::EntryRef> {
        self.try_entry(Query::from(edge.into())).ok()
    }
    fn mut_edge_data<Q: Into<EdgeQuery>>(&'i mut self, edge: Q) -> Option<Self::EntryMut> {
        self.mut_entry(Query::from(edge.into())).ok()
    }
    fn set_edge_data<Q: Into<EdgeQuery>>(&'i mut self, edge: Q, data: V) {
        self.set_entry(Query::from(edge.into()), data).ok();
    }
}
