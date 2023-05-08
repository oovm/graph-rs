use crate::{Entry, GraphError, Query, ValueProvider};
use std::collections::BTreeMap;

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
#[derive(Clone, Debug)]
pub struct DictStorage<T> {
    nodes: BTreeMap<usize, T>,
    edges: BTreeMap<usize, T>,
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
    pub fn get_data(&self, query: Query) -> Option<&T> {
        let item = match query.entry {
            Entry::Node => self.nodes.get(&query.index)?,
            Entry::Edge => self.edges.get(&query.index)?,
        };
        Some(item)
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
    pub fn mut_data(&mut self, query: Query) -> Option<&mut T> {
        let item = match query.entry {
            Entry::Node => self.nodes.get_mut(&query.index)?,
            Entry::Edge => self.edges.get_mut(&query.index)?,
        };
        Some(item)
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
    pub fn set_data(&mut self, query: Query, data: T) {
        match query.entry {
            Entry::Node => {
                self.nodes.insert(query.index, data);
            }
            Entry::Edge => {
                self.edges.insert(query.index, data);
            }
        };
    }
}
