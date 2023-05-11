use crate::{GraphEntry, GraphError, Query, ValueProvider};

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
pub struct ListStorage<T> {
    nodes: Vec<T>,
    edges: Vec<T>,
}

impl<T> Default for ListStorage<T>
where
    T: Default,
{
    fn default() -> Self {
        Self { nodes: Vec::new(), edges: Vec::new() }
    }
}

// noinspection DuplicatedCode
impl<'i, T> ValueProvider<'i, T> for ListStorage<T>
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

impl<T> ListStorage<T> {
    /// Get the reference of data by given query from the storage.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::GraphEngine;
    /// ```
    pub fn get_data(&self, query: Query) -> Option<&T> {
        let item = match query.entry {
            GraphEntry::Node => self.nodes.get(query.index)?,
            GraphEntry::Edge => self.edges.get(query.index)?,
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
            GraphEntry::Node => self.nodes.get_mut(query.index)?,
            GraphEntry::Edge => self.edges.get_mut(query.index)?,
        };
        Some(item)
    }
}
