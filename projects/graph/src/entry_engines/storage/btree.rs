use crate::{GraphEntry, GraphError, Query, ValueProvider};
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

// noinspection DuplicatedCode
impl<'i, T> ValueProvider<'i, T> for DictStorage<T>
where
    T: 'i + Send + Sync,
{
    type ValueRef = &'i T;
    type ValueMut = &'i mut T;

    fn get_value(&'i self, query: Query) -> Result<Self::ValueRef, GraphError> {
        match self.get_data(query) {
            Some(item) => Ok(item),
            None => Err(GraphError::not_found(query)),
        }
    }

    fn mut_value(&'i mut self, query: Query) -> Result<Self::ValueMut, GraphError> {
        match self.mut_data(query) {
            Some(item) => Ok(item),
            None => Err(GraphError::not_found(query)),
        }
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
        let item = match query.entry {
            GraphEntry::Node => self.nodes.get(&query.index)?,
            GraphEntry::Edge => self.edges.get(&query.index)?,
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
        let item = match query.entry {
            GraphEntry::Node => self.nodes.get_mut(&query.index)?,
            GraphEntry::Edge => self.edges.get_mut(&query.index)?,
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
    pub fn set_data(&mut self, query: Query, data: T) {
        match query.entry {
            GraphEntry::Node => {
                self.nodes.insert(query.index, data);
            }
            GraphEntry::Edge => {
                self.edges.insert(query.index, data);
            }
        };
    }
}
