use crate::Query;
use std::collections::BTreeMap;

/// A sparse entry engine that uses a [BTreeMap] to store data.
///
/// # Examples
///
/// ```
/// use graph_theory::GraphEngine;
/// ```
#[derive(Clone, Debug)]
pub struct DictStorage<T> {
    nodes: BTreeMap<usize, T>,
    edges: BTreeMap<usize, T>,
}

impl<T> Default for DictStorage<T> {
    fn default() -> Self {
        Self { nodes: BTreeMap::default(), edges: BTreeMap::default() }
    }
}

impl<T> DictStorage<T> {
    /// Get the reference of data by given query from the storage.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::GraphEngine;
    /// ```
    pub fn get_data(&self, query: Query) -> Option<&T> {
        let item = match query {
            Query::NodeID(id) => self.nodes.get(&id)?,
            Query::EdgeID(id) => self.edges.get(&id)?,
            _ => return None,
        };
        Some(item)
    }
    /// Get the mutable reference of data by given query from the storage.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::GraphEngine;
    /// ```
    pub fn mut_data(&mut self, query: Query) -> Option<&mut T> {
        let item = match query {
            Query::NodeID(id) => self.nodes.get_mut(&id)?,
            Query::EdgeID(id) => self.edges.get_mut(&id)?,
            _ => return None,
        };
        Some(item)
    }
    /// Set the data with given query to the storage.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::GraphEngine;
    /// ```
    pub fn set_data(&mut self, query: Query, data: T) -> Option<T> {
        match query {
            Query::NodeID(id) => self.nodes.insert(id, data),
            Query::EdgeID(id) => self.edges.insert(id, data),
            _ => None,
        }
    }
}
