use crate::AdjacencyEdge;
use graph_types::{GraphError, GraphResult, UndirectedEdge};
use num_traits::{One, Zero};
use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
    ops::AddAssign,
};

#[derive(Clone, Debug)]
pub struct StaticUndirected {
    /// edges, lower triangular matrix
    edges: Vec<usize>,
}

impl Display for StaticUndirected {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = self.max_degree().to_string().len();
        let max = self.count_vertexes();
        for i in 0..max {
            for j in 0..max {
                if j > i {
                    write!(f, "{:width$} ", ".", width = size)?;
                }
                else {
                    let index = i * (i + 1) / 2 + j;
                    let edge = unsafe { self.edges.get_unchecked(index) };
                    write!(f, "{:width$} ", edge, width = size)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn max_degree_length<S>(edges: &[S]) -> usize
where
    S: Display,
{
    edges.iter().map(|edge| edge.to_string().len()).max().unwrap_or(0)
}

impl<S> StaticUndirected<S> {
    pub fn new(size: usize) -> Self
    where
        S: Zero,
    {
        let mut edges = Vec::with_capacity((size + 1) * size / 2);
        edges.resize_with((size + 1) * size / 2, S::zero);

        Self { edges }
    }
    pub fn count_vertexes(&self) -> usize {
        // edges == (size + 1) * size / 2
        ((self.edges.len() * 8 + 1) as f64).sqrt().floor() as usize
    }
    pub fn max_degree(&self) -> &S
    where
        S: PartialOrd + Zero,
    {
        self.edges.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal)).unwrap_or(&S::zero())
    }
    pub fn min_degree(&self) -> &S
    where
        S: PartialOrd + Zero,
    {
        self.edges.iter().min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal)).unwrap_or(&S::zero())
    }
    pub fn get_edge<T>(&self, undirected: T) -> GraphResult<&S>
    where
        T: Into<UndirectedEdge>,
    {
        let edge = undirected.into();
        let index = edge_index(&edge);
        match self.edges.get(index) {
            Some(s) => Ok(s),
            None => Err(GraphError::node_out_of_range(edge.max_index(), self.count_vertexes())),
        }
    }
    pub fn mut_edge<T>(&mut self, undirected: T) -> GraphResult<&mut S>
    where
        T: Into<UndirectedEdge>,
    {
        let edge = undirected.into();
        let index = edge_index(&edge);
        match self.edges.get_mut(index) {
            Some(s) => Ok(s),
            None => Err(GraphError::node_out_of_range(edge.max_index(), self.count_vertexes())),
        }
    }
    pub fn set_edge<T>(&mut self, undirected: T, connection: S) -> GraphResult<S>
    where
        T: Into<UndirectedEdge>,
    {
        let mut new = connection;
        let old = self.mut_edge(undirected)?;
        std::mem::swap(&mut new, old);
        Ok(new)
    }

    pub fn connect<T>(&mut self, edge: T) -> GraphResult<S>
    where
        T: Into<UndirectedEdge>,
        S: One + AddAssign,
    {
        let edge = self.mut_edge(edge)?;
        edge.add_assign(S::one());
        self.get_edge(edge)
    }
    pub fn disconnect<T>(&mut self, edge: T) -> GraphResult<usize>
    where
        T: Into<UndirectedEdge>,
    {
        let edge = self.mut_edge(edge)?;
        let old = *edge;
        *edge = S::zero();
        Ok(old)
    }
}

fn edge_index(edge: &UndirectedEdge) -> usize {
    let min_index = edge.min_index();
    let max_index = edge.max_index();
    max_index * (max_index + 1) / 2 + min_index
}
