use crate::{Edge, GetEdgesVisitor, GetNodesVisitor, GraphError, MutEdgesVisitor, Node, Query};
use std::{
    borrow::Cow,
    future::Future,
    ops::{Deref, DerefMut},
    pin::Pin,
};
pub mod weighted;

/// Get basic information about the graph
#[allow(unused_variables)]
pub trait Graph {
    /// # Arguments
    ///
    /// * `index`:
    ///
    /// returns: Option<Cow<Self::Node>>
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::Graph;
    /// ```
    type Node: Node + Clone;
    /// # Arguments
    ///
    /// * `index`:
    ///
    /// returns: Option<Cow<Self::Node>>
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::Graph;
    /// ```
    type Edge: Edge + Clone;

    /// # Arguments
    ///
    /// * `index`:
    ///
    /// returns: Option<Cow<Self::Node>>
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::Graph;
    /// ```
    fn get_node(&self, index: usize) -> Option<Cow<Self::Node>>;
    /// # Arguments
    ///
    /// * `index`:
    ///
    /// returns: Option<Cow<Self::Node>>
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::Graph;
    /// ```
    fn mut_node(&mut self, index: usize) -> Option<&mut Self::Node> {
        unreachable!("The {} graph does not support mutating node `{}`", std::any::type_name::<Self>(), index)
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
    /// use graph_theory::Graph;
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
    /// use graph_theory::Graph;
    /// ```
    fn count_nodes(&self) -> usize;
    /// # Arguments
    ///
    /// * `index`:
    ///
    /// returns: Option<Cow<Self::Node>>
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::Graph;
    /// ```
    fn insert_node(&mut self, node: Self::Node) -> usize {
        unreachable!("The {} graph does not support adding node `{}`", std::any::type_name::<Self>(), node.index())
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
    /// use graph_theory::Graph;
    /// ```
    fn remove_node(&mut self, index: usize) -> Option<Self::Node> {
        unreachable!("The {} graph does not support removing node `{}`", std::any::type_name::<Self>(), index)
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
    /// use graph_theory::Graph;
    /// ```
    fn get_edge(&self, index: usize) -> Option<Cow<Self::Edge>>;
    /// # Arguments
    ///
    /// * `index`:
    ///
    /// returns: Option<Cow<Self::Node>>
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::Graph;
    /// ```
    fn mut_edge(&mut self, index: usize) -> Option<&mut Self::Edge> {
        unreachable!("The {} graph does not support mutating edge `{}`", std::any::type_name::<Self>(), index)
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
    /// use graph_theory::Graph;
    /// ```
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
    /// use graph_theory::Graph;
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
    /// use graph_theory::Graph;
    /// ```
    fn insert_edge(&mut self, edge: Self::Edge) -> Self::Edge {
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
    /// use graph_theory::Graph;
    /// ```
    fn remove_edge(&mut self, index: Self::Edge) -> Option<Self::Edge> {
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
    /// use graph_theory::Graph;
    /// ```
    fn count_edges(&self) -> usize;
}
