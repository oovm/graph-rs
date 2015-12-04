use crate::{Edge, GetEdgesVisitor, GetNodesVisitor, GraphError, MutEdgesVisitor, Node, Query};
use std::{
    borrow::Cow,
    future::Future,
    ops::{Deref, DerefMut},
    pin::Pin,
};

pub mod weighted;

/// Represent a graph storage, with a set of nodes and edges.
///
/// # Examples
///
/// ```
/// use graph_theory::GraphEngine;
/// ```
#[allow(unused_variables)]
pub trait GraphEngine {
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
    fn get_nodes(&self) -> GetNodesVisitor<Self> {
        GetNodesVisitor::new(self)
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
    fn count_nodes(&self) -> usize;
    /// Insert a node without any neighbors (edges).
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::GraphEngine;
    /// ```
    fn insert_node(&mut self, node: usize) -> usize {
        todo!()
    }
    /// Remove the given node.
    ///
    /// # Undefined Behavior
    ///
    /// - If the node has any edges, the behavior is undefined.
    /// It is recommended to remove all edges before removing the node, see [`GraphEngine::remove_node_with_edges`].
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::GraphEngine;
    /// ```
    fn remove_node(&mut self, node_id: usize) {
        self.remove_node_with_edges(node_id)
    }
    /// Remove the given node and all edges connected to it.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::GraphEngine;
    /// ```
    fn remove_node_with_edges(&mut self, node_id: usize) {
        unreachable!(
    }
    fn get_edges(&self) -> GetEdgesVisitor<Self> {
        GetEdgesVisitor::new(self)
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
    fn mut_edges(&mut self) -> MutEdgesVisitor<Self> {
        MutEdgesVisitor::new(self)
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
    fn insert_edge<E: Edge>(&mut self, edge: E) -> usize {
        todo!()
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
    fn remove_edge(&mut self, index: usize) -> Option<usize> {
        todo!()
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
    fn count_edges(&self) -> usize;
}
