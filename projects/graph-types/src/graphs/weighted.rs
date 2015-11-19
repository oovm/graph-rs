use super::*;
use crate::{GraphError, Query};

pub trait WeightsProvider<'a> {
    type WeightRef;
    type WeightMut;
    fn get_weight(&'a self, query: Query) -> Result<Self::WeightRef, GraphError>;
    fn mut_weight(&'a mut self, query: Query) -> Result<Self::WeightMut, GraphError>;
}

pub trait WeightedGraph: Graph {
    type Provider: WeightsProvider;

    fn get_node_weight(
        &self,
        index: usize,
        data: &Self::Provider,
    ) -> Result<<Self::Provider as WeightsProvider>::WeightRef, GraphError> {
        data.get_weight(Query::node(index))
    }

    fn set_node_weight(
        &mut self,
        index: usize,
        data: &mut Self::Provider,
        weight: <Self::Provider as WeightsProvider>::WeightMut,
    ) -> Result<(), GraphError> {
        *data.mut_weight(Query::node(index))? = weight;
        Ok(())
    }

    fn get_edge_weight(
        &mut self,
        index: usize,
        data: &mut Self::Provider,
    ) -> Result<<Self::Provider as WeightsProvider>::WeightMut, GraphError> {
        data.mut_weight(Query::edge(index))
    }
}
