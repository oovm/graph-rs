use crate::{Edge, EdgeInsertID, EdgeQuery, EdgesVisitor, GraphError, GraphKind, NodesVisitor, Query};

use std::{
    future::Future,
    mem::size_of,
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
pub trait GraphEngine
where
    Self: Sized,
{
    /// Check the graph kind, it can be directed or undirected.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::{graph_engines::CompleteGraph, GraphEngine};
    /// assert_eq!(CompleteGraph::one_way(5).has_node(5), true);
    /// assert_eq!(CompleteGraph::one_way(5).has_node(6), false);
    /// ```
    fn graph_kind(&self) -> GraphKind;

    /// Check if the node exists, return the node id if exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::{graph_engines::CompleteGraph, GraphEngine};
    /// assert_eq!(CompleteGraph::one_way(5).has_node(5), true);
    /// assert_eq!(CompleteGraph::one_way(5).has_node(6), false);
    /// ```
    fn has_node(&self, node_id: usize) -> Option<usize>;

    /// Check if the node exists, return the node id if exists.
    ///
    /// Return [None] if the node does not exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::{graph_engines::CompleteGraph, GraphEngine};
    /// assert_eq!(CompleteGraph::one_way(5).count_nodes(), 5);
    /// ```
    fn get_node_degree(&self, node_id: usize) -> Result<usize, GraphError> {
        todo!()
    }
    /// Count the number of nodes in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::{graph_engines::CompleteGraph, GraphEngine};
    /// assert_eq!(CompleteGraph::one_way(5).count_nodes(), 5);
    /// ```
    fn count_nodes(&self) -> usize;
    /// Traverse all nodes in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::{graph_engines::CompleteGraph, GraphEngine};
    /// let mut graph = CompleteGraph::one_way(5);
    /// assert_eq!(graph.traverse_nodes().count(), 20)
    /// ```
    fn traverse_nodes(&self) -> NodesVisitor<Self>;
    /// Check if the edge exists, return the node id if exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::{graph_engines::CompleteGraph, GraphEngine};
    /// assert_eq!(CompleteGraph::one_way(5).has_node(5), true);
    /// assert_eq!(CompleteGraph::one_way(5).has_node(6), false);
    /// ```
    fn has_edge<E: Into<EdgeQuery>>(&self, edge: E) -> Option<usize>;

    /// Get the edges of the graph.
    ///
    ///
    /// ```
    /// use graph_theory::{graph_engines::CompleteGraph, GraphEngine};
    /// let mut graph = CompleteGraph::one_way(5);
    /// assert_eq!(graph.traverse_nodes().count(), 20)
    /// ```
    fn traverse_edges(&self) -> EdgesVisitor<Self>;

    /// Count the number of edges in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// # use graph_theory::{GraphEngine};
    /// # use graph_theory::graph_engines::{EasyGraphTable, StarGraph, CompleteGraph};
    /// assert_eq!(EasyGraphTable::one_way(5).count_edges(), 5);
    /// assert_eq!(EasyGraphTable::two_way(5).count_edges(), 10);
    /// assert_eq!(StarGraph::one_way(5).count_edges(), 5);
    /// assert_eq!(StarGraph::two_way(5).count_edges(), 10);
    /// assert_eq!(CompleteGraph::one_way(5).count_edges(), 5);
    /// assert_eq!(CompleteGraph::one_way(5).count_edges(), 10);
    /// ```
    fn count_edges(&self) -> usize;

    /// Query the total space occupied by the structure, return 0 if failed to query
    ///
    /// Note that this volume contains garbage data, call [GraphEngine::shrink] at the right time to perform garbage collection.
    fn size_hint(&self) -> usize {
        size_of::<Self>()
    }
}

/// Mark a graph engine that can add and delete edges or points
pub trait MutableGraph: GraphEngine {
    /// Insert a node without any neighbors (edges).
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::{graph_engines::AdjacencyNodeList, GraphEngine};
    /// let mut graph = AdjacencyNodeList::default();
    /// assert_eq!(graph.count_nodes(), 0);
    /// graph.insert_node(5);
    /// assert_eq!(graph.count_nodes(), 1);
    /// ```
    fn insert_node(&mut self, node_id: usize) -> usize;
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
    /// use graph_theory::{graph_engines::AdjacencyNodeList, GraphEngine};
    /// let mut graph = AdjacencyNodeList::default();
    /// assert_eq!(graph.count_nodes(), 0);
    /// graph.insert_node(5);
    /// assert_eq!(graph.count_nodes(), 1);
    /// ```
    fn remove_node(&mut self, node_id: usize) {
        self.remove_node_with_edges(node_id)
    }
    /// Remove the given node and all edges connected to it.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::{graph_engines::AdjacencyNodeList, GraphEngine};
    /// let mut graph = AdjacencyNodeList::default();
    /// assert_eq!(graph.count_nodes(), 0);
    /// graph.insert_node(5);
    /// assert_eq!(graph.count_nodes(), 1);
    /// ```
    fn remove_node_with_edges(&mut self, node_id: usize);
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
    fn insert_edge<E: Edge>(&mut self, edge: E) -> EdgeInsertID {
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
    fn insert_edge_with_nodes<E: Edge>(&mut self, edge: E) -> EdgeInsertID;
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
    fn remove_edge<E>(&mut self, edge: E)
    where
        E: Into<EdgeQuery>;

    /// Remove invalid edges and nodes to improve the efficiency of subsequent queries.
    fn shrink(&mut self) {}
}
