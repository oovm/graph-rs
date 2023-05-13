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
        let item = match query {
            Query::NodeID(id) => self.nodes.get(id)?,
            Query::EdgeID(id) => self.edges.get(id)?,
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
            Query::NodeID(id) => self.nodes.get_mut(id)?,
            Query::EdgeID(id) => self.edges.get_mut(id)?,
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
    pub fn set_data(&mut self, query: Query, data: T) -> Option<T> {
        let mut new = data;
        let old = self.mut_data(query)?;
        std::mem::swap(old, &mut new);
        return Some(new);
    }
}
