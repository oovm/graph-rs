use crate::{Entry, GraphError, Query, WeightsProvider};
use dashmap::{
    mapref::one::{Ref, RefMut},
    DashMap,
};

#[derive(Clone, Debug)]
pub struct SharedStorage<T> {
    nodes: DashMap<usize, T>,
    edges: DashMap<usize, T>,
}

impl<'i, T> WeightsProvider for &'i SharedStorage<T> {
    type WeightRef = Ref<'i, usize, T>;
    type WeightMut = RefMut<'i, usize, T>;

    fn get_weight(&self, query: Query) -> Result<Self::WeightRef, GraphError> {
        match self.get_data(query) {
            Some(item) => Ok(item),
            None => Err(GraphError::not_found(query)),
        }
    }

    fn mut_weight(&mut self, query: Query) -> Result<Self::WeightMut, GraphError> {
        match self.mut_data(query) {
            Some(item) => Ok(item),
            None => Err(GraphError::not_found(query)),
        }
    }
}

impl<T> SharedStorage<T> {
    pub fn get_data(&self, query: Query) -> Option<Ref<usize, T>> {
        let item = match query.entry {
            Entry::Node => self.nodes.get(&query.index)?,
            Entry::Edge => self.edges.get(&query.index)?,
        };
        Some(item)
    }
    pub fn mut_data(&self, query: Query) -> Option<RefMut<usize, T>> {
        let item = match query.entry {
            Entry::Node => self.nodes.get_mut(&query.index)?,
            Entry::Edge => self.edges.get_mut(&query.index)?,
        };
        Some(item)
    }
}
