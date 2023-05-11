use super::*;
use crate::GraphErrorKind;
use std::{any::type_name, borrow::Cow};

/// Extend the ability to get a value from a graph
///
/// # Examples
///
/// ```
/// use graph_theory::{entry_engines::ListStorage, EntryEngine, GraphEngine};
/// ```
#[allow(unused_variables)]
pub trait ValueProvider<'a, V>: Send + Sync {
    /// The reference type of [V].
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::{entry_engines::ListStorage, EntryEngine, GraphEngine};
    /// ```
    type ValueRef: Deref<Target = V>;
    /// The mutable reference type of [V].
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::{entry_engines::ListStorage, EntryEngine, GraphEngine};
    /// ```
    type ValueMut: DerefMut<Target = V>;
    /// Get the value reference from the graph by [Query].
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::{entry_engines::ListStorage, EntryEngine, GraphEngine};
    /// ```
    fn try_get_value(&'a self, query: Query) -> Result<Self::ValueRef, GraphError>;
    /// Get the value reference from the graph by [Query].
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::{entry_engines::ListStorage, EntryEngine, GraphEngine};
    /// ```
    fn get_value(&'a self, query: Query) -> Self::ValueRef;
    /// Get the mutable value reference from the graph by [Query].
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::{entry_engines::ListStorage, EntryEngine, GraphEngine};
    /// ```
    fn try_mut_value(&'a mut self, query: Query) -> Result<Self::ValueMut, GraphError>;

    fn mut_value(&'a mut self, query: Query) -> Self::ValueMut;

    /// Set the owned value to the graph by [Query], return the old value if it exists,
    /// return [NotFound](GraphErrorKind::NotFound) error missing this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::{entry_engines::ListStorage, EntryEngine, GraphEngine};
    /// ```
    fn set_value(&'a mut self, query: Query, value: V) -> Result<V, GraphError> {
        let mut new = value;
        let mut old = self.try_mut_value(query)?;
        std::mem::swap(&mut new, &mut old);
        Ok(new)
    }
}

/// A graph that has extra data associated with each node and edge.
//
/// # Examples
///
/// ```
/// use graph_types::{EntryEngine, GraphEngine, ListStorage};
/// ```
pub trait EntryEngine<V>: GraphEngine {
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
    /// use graph_types::{EntryEngine, GraphEngine, ListStorage};
    /// ```
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
    /// use graph_types::{EntryEngine, GraphEngine, ListStorage};
    /// ```
    fn get_data<'a, 'p>(
        &'a self,
        data: &'p Self::Provider,
        query: Query,
    ) -> Result<<Self::Provider as ValueProvider<'p, V>>::ValueRef, GraphError> {
        data.try_get_value(query)
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
    /// use graph_types::{EntryEngine, GraphEngine, ListStorage};
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
    /// use graph_types::{EntryEngine, GraphEngine, ListStorage};
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
        Box::pin(async move { data.try_get_value(query) })
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
    /// use graph_types::{EntryEngine, GraphEngine, ListStorage};
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
