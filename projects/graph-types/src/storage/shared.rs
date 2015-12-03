use crate::{Entry, GraphError, Query, ValueProvider};
use dashmap::{
    mapref::one::{Ref, RefMut},
    DashMap,
};

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
#[derive(Clone, Debug, Default)]
pub struct SharedStorage<T> {
    nodes: DashMap<usize, T>,
    edges: DashMap<usize, T>,
}

// noinspection DuplicatedCode
impl<'i, T> ValueProvider<'i, T> for SharedStorage<T>
where
    T: 'i + Send + Sync,
{
    type ValueRef = Ref<'i, usize, T>;
    type ValueMut = RefMut<'i, usize, T>;

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

impl<T> SharedStorage<T> {
    /// # Arguments
    ///
    /// * `index`:
    /// * `data`:
    ///
    /// returns: Result<<Self::Provider as ValueProvider>::ValueRef, GraphError>
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_types::{GraphEngine, GraphData, ListStorage};
    /// ```
    pub fn get_data(&self, query: Query) -> Option<Ref<usize, T>> {
        let item = match query.entry {
            Entry::Node => self.nodes.get(&query.index)?,
            Entry::Edge => self.edges.get(&query.index)?,
        };
        Some(item)
    }
    /// # Arguments
    ///
    /// * `index`:
    /// * `data`:
    ///
    /// returns: Result<<Self::Provider as ValueProvider>::ValueRef, GraphError>
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_types::{GraphEngine, GraphData, ListStorage};
    /// ```
    pub fn mut_data(&self, query: Query) -> Option<RefMut<usize, T>> {
        let item = match query.entry {
            Entry::Node => self.nodes.get_mut(&query.index)?,
            Entry::Edge => self.edges.get_mut(&query.index)?,
        };
        Some(item)
    }
}
