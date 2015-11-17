use crate::{Entry, Graph, GraphError, GraphErrorKind, GraphResult};
use dashmap::{
    mapref::one::{Ref, RefMut},
    DashMap,
};
use std::{collections::BTreeMap, ops::Deref};

pub struct UniqueStorage<N, E> {
    vertexes: BTreeMap<usize, N>,
    edges: BTreeMap<usize, E>,
}

pub trait WeightsProvider {
    type WeightRef;
    type WeightMut;
    fn get_weight(&self, query: Query) -> Result<Self::WeightRef, GraphError>;
    fn mut_weight(&mut self, query: Query) -> Result<Self::WeightMut, GraphError>;
}

#[derive(Clone, Debug)]
pub struct DenseStorage<T> {
    nodes: Vec<T>,
    edges: Vec<T>,
}

impl<'i, T> WeightsProvider for &'i DenseStorage<T> {
    type WeightRef = &'i T;
    type WeightMut = &'i mut T;

    fn get_weight(&self, query: Query) -> Result<Self::WeightRef, GraphError> {
        let item = match query.entry {
            Entry::Node => self.nodes.get(query.index)?,
            Entry::Edge => self.edges.get(query.index)?,
        };
        Ok(item)
    }

    fn mut_weight(&mut self, query: Query) -> Result<Self::WeightMut, GraphError> {
        let item = self.nodes.get_mut(query.index)?;
        Some(item)
    }
}

#[derive(Clone, Debug)]
pub struct SharedStorage<T> {
    nodes: DashMap<usize, T>,
    edges: DashMap<usize, T>,
}

impl<'i, T> WeightsProvider for &'i SharedStorage<T> {
    type WeightRef = Ref<'i, usize, T>;
    type WeightMut = RefMut<'i, usize, T>;

    fn get_weight(&self, query: Query) -> Result<Self::WeightRef, GraphError> {
        let item = match query.entry {
            Entry::Node => self.nodes.get(&query.index)?,
            Entry::Edge => self.edges.get(&query.index)?,
        };
        Ok(item)
    }

    fn mut_weight(&mut self, query: Query) -> Result<Self::WeightRef, GraphError> {
        let item = match query.entry {
            Entry::Node => self.nodes.get_mut(&query.index)?,
            Entry::Edge => self.edges.get_mut(&query.index)?,
        };
        Ok(item)
    }
}

pub trait WeightedGraph: Graph {
    type Provider: WeightsProvider;

    fn get_weight(&self, query: Query, data: &Self::Provider) -> Option<<Self::Provider as WeightsProvider>::WeightRef> {
        data.get_weight(query)
    }
    #[rustfmt::skip]
    fn mut_weight(&mut self, query: Query, data: &mut Self::Provider) -> Option<<Self::Provider as WeightsProvider>::WeightMut> {
        data.mut_weight(query)
    }
}
