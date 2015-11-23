use super::*;

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
/// use graph_types::{Graph, GraphData, ListStorage};
/// ```
pub trait ValueProvider<'a, V>: Send + Sync {
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
    /// use graph_types::{Graph, GraphData, ListStorage};
    /// ```
    type ValueRef: Deref<Target = V>;
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
    /// use graph_types::{Graph, GraphData, ListStorage};
    /// ```
    type ValueMut: DerefMut<Target = V>;
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
    /// use graph_types::{Graph, GraphData, ListStorage};
    /// ```
    fn get_value(&'a self, query: Query) -> Result<Self::ValueRef, GraphError>;
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
    /// use graph_types::{Graph, GraphData, ListStorage};
    /// ```
    fn mut_value(&'a mut self, query: Query) -> Result<Self::ValueMut, GraphError>;
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
    /// use graph_types::{Graph, GraphData, ListStorage};
    /// ```
    fn set_value(&'a mut self, query: Query, value: V) -> Result<V, GraphError> {
        let mut new = value;
        let mut old = self.mut_value(query)?;
        std::mem::swap(&mut new, &mut old);
        Ok(new)
    }
}

/// A graph that has extra data associated with each node and edge.
///
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
/// use graph_types::{Graph, GraphData, ListStorage};
/// ```
pub trait GraphData<V>: Graph {
    type Provider: for<'p> ValueProvider<'p, V>;

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
    /// use graph_types::{Graph, GraphData, ListStorage};
    /// ```
    fn get_data<'a, 'p>(
        &'a self,
        data: &'p Self::Provider,
        query: Query,
    ) -> Result<<Self::Provider as ValueProvider<'p, V>>::ValueRef, GraphError> {
        data.get_value(query)
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
    /// use graph_types::{Graph, GraphData, ListStorage};
    /// ```
    fn mut_data(&self, data: &mut Self::Provider, query: Query, value: V) -> Result<V, GraphError> {
        data.set_value(query, value)
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
    /// use graph_types::{Graph, GraphData, ListStorage};
    /// ```
    fn async_get_data<'a, 'p, 'async_trait>(
        &'a self,
        data: &'p Self::Provider,
        query: Query,
    ) -> Pin<
        Box<dyn Future<Output = Result<<Self::Provider as ValueProvider<'p, V>>::ValueRef, GraphError>> + Send + 'async_trait>,
    >
    where
        'a: 'async_trait,
        'p: 'async_trait,
        V: Send + 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move { data.get_value(query) })
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
    /// use graph_types::{Graph, GraphData, ListStorage};
    /// ```
    fn async_mut_data<'a, 'p, 'async_trait>(
        &'a self,
        data: &'p mut Self::Provider,
        query: Query,
        value: V,
    ) -> Pin<Box<dyn Future<Output = Result<V, GraphError>> + Send + 'async_trait>>
    where
        'a: 'async_trait,
        'p: 'async_trait,
        V: Send + 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move { data.set_value(query, value) })
    }
}
