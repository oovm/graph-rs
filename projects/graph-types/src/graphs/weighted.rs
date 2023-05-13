use super::*;

/// Labeling a graph can provide Weight information
///
/// # Examples
///
/// ```
/// use graph_theory::GraphEngine;
/// ```
pub trait WeightedGraph<V>: GraphEngine {
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
    type WeightRef: Deref<Target = V>;
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
    type WeightMut: DerefMut<Target = V>;
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
    fn get_node_weight<Q: Into<NodeQuery>>(&self, node: Q) -> Option<Self::WeightRef>;
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
    fn mut_node_weight<Q: Into<NodeQuery>>(&self, node: Q) -> Option<Self::WeightMut>;
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
    fn set_node_weight<Q: Into<NodeQuery>>(&mut self, node: Q, weight: V);
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
    fn get_edge_weight<Q: Into<EdgeQuery>>(&self, edge: Q) -> Option<Self::WeightRef>;
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
    fn mut_edge_weight<Q: Into<EdgeQuery>>(&self, edge: Q) -> Option<Self::WeightMut>;
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
    fn set_edge_weight<Q: Into<EdgeQuery>>(&mut self, edge: Q, weight: V);
}
