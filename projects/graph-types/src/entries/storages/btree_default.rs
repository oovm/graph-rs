use crate::entry_engines::DictStorage;
use graph_types::{GraphEntry, Query};

/// A sparse entry engine that uses a [BTreeMap] to store data.
///
/// # Examples
///
/// ```
/// use graph_theory::GraphEngine;
/// ```
#[derive(Clone, Debug)]
pub struct DictDefault<T> {
    dict: DictStorage<T>,
    node_default: T,
    edge_default: T,
}

impl<T> DictDefault<T> {
    pub fn new(node: T, edge: T) -> Self {
        Self { dict: DictStorage::default(), node_default: node, edge_default: edge }
    }
}

impl<T> DictDefault<T> {
    /// Get the reference of data by given query from the storage.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::GraphEngine;
    /// ```
    pub fn get_data(&self, query: Query) -> &T {
        match self.dict.get_data(query) {
            None => match query.as_entry() {
                GraphEntry::Node => &self.node_default,
                GraphEntry::Edge => &self.edge_default,
            },
            Some(s) => s,
        }
    }
    /// Get the mutable reference of data by given query from the storage.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::GraphEngine;
    /// ```
    pub fn mut_data(&mut self, query: Query) -> Option<&mut T> {
        self.mut_data(query)
    }
    /// Set the data with given query to the storage.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::GraphEngine;
    /// ```
    pub fn set_data(&mut self, query: Query, data: T) {
        self.dict.set_data(query, data);
    }
    /// Set the data with given query to the storage.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::GraphEngine;
    /// ```
    pub fn set_default(&mut self, entry: GraphEntry, data: T) {
        match entry {
            GraphEntry::Node => self.node_default = data,
            GraphEntry::Edge => self.edge_default = data,
        }
    }
}
