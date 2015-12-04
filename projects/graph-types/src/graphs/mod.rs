use crate::{Edge, EdgeRemoveAction, GetEdgesVisitor, GetNodesVisitor, GraphError, MutEdgesVisitor, Node, Query};
use std::{
    borrow::Cow,
    future::Future,
    ops::{Deref, DerefMut},
    pin::Pin,
};
use std::any::type_name;

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
    ///
    /// It is recommended to remove all edges before removing the node, see [`GraphEngine::remove_node_with_edges`].
    ///
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
        unreachable!("Graph engine {} does not support removing nodes", type_name::<Self>())
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
    /// Remove edge by given edge-id or start and end node-id.
    ///
    /// Returns `true` if the edge was removed, `false` if the edge was not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::GraphEngine;
    /// ```
    fn remove_edge<E>(&mut self, edge: E) -> bool where E: Into<EdgeRemoveAction> {
        unreachable!("Graph engine {} does not support removing edges", type_name::<Self>())
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
