use crate::AdjacencyEdge;
use graph_types::{DirectedEdge, GraphError, GraphResult};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct StaticDirected<V = (), E = ()> {
    /// metadata of vertexes
    vertexes: Vec<V>,
    /// edges, lower triangular matrix
    edges: Vec<AdjacencyEdge<E>>,
}

impl<V, E> Display for StaticDirected<V, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let size = self.min_degree().to_string().len();
        let max = self.vertexes.len();
        for i in 0..max {
            for j in 0..max {
                let index = i * max + j;
                let edge = self.edges.get(index).unwrap();
                write!(f, "{:width$} ", edge.degree, width = size)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<V, E> StaticDirected<V, E> {
    pub fn max_degree(&self) -> usize {
        self.edges.iter().map(|edge| edge.degree).max().unwrap_or(0)
    }
    pub fn min_degree(&self) -> usize {
        self.edges.iter().map(|edge| edge.degree).min().unwrap_or(0)
    }
}

impl<V, E> StaticDirected<V, E>
where
    V: Default,
    E: Default,
{
    pub fn new(size: usize) -> Self {
        let mut vertexes = Vec::with_capacity(size);
        vertexes.resize_with(size, V::default);
        let mut edges = Vec::with_capacity((size + 1) * size / 2);
        edges.resize_with((size + 1) * size / 2, AdjacencyEdge::default);
        Self { vertexes, edges }
    }
    pub fn get_node(&self, index: usize) -> GraphResult<&V> {
        match self.vertexes.get(index) {
            Some(s) => Ok(s),
            None => Err(GraphError::node_out_of_range(index, self.vertexes.len())),
        }
    }
    pub fn set_node<F>(&mut self, index: usize, edit: F) -> GraphResult<()>
    where
        F: Fn(&mut V),
    {
        match self.vertexes.get_mut(index) {
            Some(s) => {
                edit(s);
                Ok(())
            }
            None => Err(GraphError::node_out_of_range(index, self.vertexes.len())),
        }
    }
    pub fn get_edge<T>(&self, directed: T) -> GraphResult<&E>
    where
        T: Into<DirectedEdge>,
    {
        let edge = directed.into();
        let index = edge_index(&edge);
        match self.edges.get(index) {
            Some(s) => Ok(&s.metadata),
            None => Err(GraphError::node_out_of_range(edge.max_index(), self.vertexes.len())),
        }
    }
    pub fn set_edge<T, F>(&mut self, directed: T, edit: F) -> GraphResult<usize>
    where
        T: Into<DirectedEdge>,
        F: Fn(&mut AdjacencyEdge<E>),
    {
        let edge = directed.into();
        let index = edge_index(&edge);
        match self.edges.get_mut(index) {
            Some(s) => {
                edit(s);
                Ok(s.degree)
            }
            None => Err(GraphError::node_out_of_range(edge.max_index(), self.vertexes.len())),
        }
    }
    pub fn connect<T>(&mut self, edge: T) -> GraphResult<usize>
    where
        T: Into<DirectedEdge>,
    {
        self.set_edge(edge, |e| e.degree = e.degree.saturating_add(1))
    }
    pub fn disconnect<T>(&mut self, edge: T) -> GraphResult<usize>
    where
        T: Into<DirectedEdge>,
    {
        self.set_edge(edge, |e| e.degree = e.degree.saturating_sub(1))
    }
}

fn edge_index(edge: &DirectedEdge) -> usize {
    let min_index = edge.min_index();
    let max_index = edge.max_index();
    max_index * (max_index + 1) / 2 + min_index
}
