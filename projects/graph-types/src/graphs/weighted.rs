use super::*;
use crate::{GraphError, Query};
use std::ops::{Deref, DerefMut};

pub trait WeightsProvider<'a> {
    type Weight;
    type WeightRef: Deref<Target = Self::Weight>;
    type WeightMut: DerefMut<Target = Self::Weight>;
    fn get_weight(&'a self, query: Query) -> Result<Self::WeightRef, GraphError>;
    fn mut_weight(&'a mut self, query: Query) -> Result<Self::WeightMut, GraphError>;
}

pub trait WeightedGraph: Graph {
    type Provider: for<'p> WeightsProvider<'p>;

    fn get_node_weight<'a, 'p>(
        &'a self,
        index: usize,
        data: &'p Self::Provider,
    ) -> Result<<Self::Provider as WeightsProvider<'p>>::WeightRef, GraphError> {
        data.get_weight(Query::node(index))
    }

    fn set_node_weight<'a, 'p>(
        &'a mut self,
        index: usize,
        data: &'p mut Self::Provider,
        weight: <Self::Provider as WeightsProvider<'p>>::Weight,
    ) -> Result<(), GraphError> {
        let mut item = data.mut_weight(Query::node(index))?;
        *item.deref_mut() = weight;
        Ok(())
    }
}
