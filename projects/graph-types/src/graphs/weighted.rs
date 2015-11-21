use super::*;
use crate::{GraphError, Query};
use std::{
    future::Future,
    ops::{Deref, DerefMut},
    pin::Pin,
};

pub trait ValueProvider<'a, V> {
    type ValueRef: Deref<Target = V>;
    type ValueMut: DerefMut<Target = V>;
    fn get_value(&'a self, query: Query) -> Result<Self::ValueRef, GraphError>;
    fn mut_value(&'a mut self, query: Query) -> Result<Self::ValueMut, GraphError>;
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
    fn get_node_data<'a, 'p>(
        &'a self,
        index: usize,
        data: &'p Self::Provider,
    ) -> Result<<Self::Provider as ValueProvider<'p, V>>::ValueRef, GraphError> {
        data.get_value(Query::node(index))
    }

    fn set_node_data(&self, index: usize, data: &mut Self::Provider, value: V) -> Result<V, GraphError> {
        data.set_value(Query::node(index), value)
    }

    fn get_data_async<'asynchronous, 'provider>(
        &'asynchronous self,
        query: Query,
        data: &'provider Self::Provider,
    ) -> Pin<
        Box<dyn Future<Output = Result<<Self::Provider as ValueProvider<'provider, V>>::ValueRef, GraphError>> + Send + 'asynchronous>,
    >
    where
        Self: Sync + 'asynchronous,
    {


        async fn run(self: &Self) {
            /* the original method body */
        }

        Box::pin(run(self))

        data.get_value(query)
    }
}
