use crate::{Entry, GraphError, Query, WeightsProvider};
use dashmap::{
    mapref::one::{Ref, RefMut},
    DashMap,
};

#[derive(Clone, Debug)]
pub struct VectorStorage<T> {
    nodes: Vec<usize, T>,
    edges: Vec<usize, T>,
}

impl<'i, T: 'i> WeightsProvider<'i> for VectorStorage<T> {
    type Weight = T;
    type WeightRef = Ref<'i, usize, T>;
    type WeightMut = RefMut<'i, usize, T>;

    fn get_weight(&'i self, query: Query) -> Result<Self::WeightRef, GraphError> {
        match self.get_data(query) {
            Some(item) => Ok(item),
            None => Err(GraphError::not_found(query)),
        }
    }

    fn mut_weight(&'i mut self, query: Query) -> Result<Self::WeightMut, GraphError> {
        match self.mut_data(query) {
            Some(item) => Ok(item),
            None => Err(GraphError::not_found(query)),
        }
    }
}

impl<T> VectorStorage<T> {
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
