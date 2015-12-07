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
    /// Mark the graph engine does not support the ability.
    ///
    /// Currently, we can not detect the ability at compile time, so we use this method to mark the ability is not supported.
    fn exception(&self, ability: &'static str) -> ! {
        unreachable!("Graph engine {} does not support {ability}", type_name::<Self>())
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
    /// use graph_theory::CompleteGraph;
    /// assert_eq!(CompleteGraph::new(5).count_nodes(), 10);
    /// ```
    fn count_nodes(&self) -> usize;
    /// Insert a node without any neighbors (edges).
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::GraphEngine;
    /// use graph_theory::adjacency_list::UnGraph;
    /// let mut graph = UnGraph::default();
    /// assert_eq!(graph.count(), 0);
    /// graph.insert_node(5);
    /// assert_eq!(graph.count(), 1);
    /// ```
    fn insert_node(&mut self, node_id: usize) -> usize {
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
    /// use graph_theory::adjacency_list::UnGraph;
    /// let mut graph = UnGraph::default();
    /// assert_eq!(graph.count(), 0);
    /// graph.insert_node(5);
    /// assert_eq!(graph.count(), 1);
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
    /// use graph_theory::adjacency_list::UnGraph;
    /// let mut graph = UnGraph::default();
    /// assert_eq!(graph.count(), 0);
    /// graph.insert_node(5);
    /// assert_eq!(graph.count(), 1);
    /// ```
    fn remove_node_with_edges(&mut self, node_id: usize);
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
    /// Insert a edge between two nodes.
    ///
    /// # Undefined Behaviors
    ///
    /// - If the nodes does not exist, the behavior is undefined.
    ///
    /// It is recommended to check the existence of the nodes before inserting the edge, see [`GraphEngine::insert_edge_with_nodes`].
    ///
    /// - Insert undirected edge to directed graph.
    ///
    /// Two edges will be inserted, but only return last edge's id.
    ///
    /// # Panics
    ///
    /// - No such ability
    ///
    /// Not all graph engine supports insert edge.
    ///
    /// - Insert disconnected edge
    ///
    /// Meaningless, don't do that.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::GraphEngine;
    /// ```
    fn insert_edge<E: Edge>(&mut self, edge: E) -> EdgeInsertResult {
        self.insert_edge_with_nodes(edge)
    }
    /// Insert edge to graph, if the nodes does not exist, also insert them.
    ///
    /// # Panics
    ///
    /// - No such ability
    ///
    /// Not all graph engine supports insert edge.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::GraphEngine;
    /// ```
    fn insert_edge_with_nodes<E: Edge>(&mut self, edge: E) -> EdgeInsertResult;
    /// Remove edge by given edge-id or start and end node-id.
    ///
    /// # Panics
    ///
    /// - No such ability
    ///
    /// Not all graph engine supports insert edge.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::GraphEngine;
    /// ```
    fn remove_edge<E>(&mut self, edge: E) where E: Into<EdgeRemoveAction>;
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
