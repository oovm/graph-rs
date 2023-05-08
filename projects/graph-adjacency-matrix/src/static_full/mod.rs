use graph_types::{DirectedEdge, EdgeQuery, GetEdgesVisitor, GraphEngine, GraphError, GraphKind, GraphResult, NodesVisitor};
use std::fmt::{Debug, Display};
mod display;

#[derive(Clone, Debug)]
pub struct AdjacencyMatrix {
    /// edges, full matrix
    edges: Vec<usize>,
}

impl GraphEngine for AdjacencyMatrix {
    fn graph_kind(&self) -> GraphKind {
        GraphKind::Directed
    }

    fn has_node(&self, node_id: usize) -> Option<usize> {
        todo!()
    }

    fn count_nodes(&self) -> usize {
        todo!()
    }

    fn traverse_nodes(&self) -> NodesVisitor<Self> {
        todo!()
    }

    fn has_edge<E: Into<EdgeQuery>>(&self, edge: E) -> Option<usize> {
        todo!()
    }

    fn traverse_edges(&self) -> GetEdgesVisitor<Self> {
        todo!()
    }

    fn count_edges(&self) -> usize {
        todo!()
    }
}

impl AdjacencyMatrix {
    pub fn new(nodes: usize) -> Self {
        Self { edges: vec![0; nodes * nodes] }
    }
    pub fn nodes(&self) -> usize {
        // edges == nodes * nodes
        (self.edges.len() as f64).sqrt() as usize
    }
    pub fn edges(&self) -> usize {
        self.edges.len()
    }
    pub fn max_degree(&self) -> usize {
        self.edges.iter().max().copied().unwrap_or(0)
    }
    pub fn min_degree(&self) -> usize {
        self.edges.iter().min().copied().unwrap_or(0)
    }
    pub fn get_edge<T>(&self, undirected: T) -> GraphResult<usize>
    where
        T: Into<DirectedEdge>,
    {
        let edge = undirected.into();
        let index = edge_index(&edge, self.nodes());
        match self.edges.get(index) {
            Some(s) => Ok(*s),
            None => Err(GraphError::edge_out_of_range(index, self.edges.len())),
        }
    }
    pub fn mut_edge<T>(&mut self, undirected: T) -> GraphResult<&mut usize>
    where
        T: Into<DirectedEdge>,
    {
        let edge = undirected.into();
        let index = edge_index(&edge, self.nodes());
        let count = self.edges.len();
        match self.edges.get_mut(index) {
            Some(s) => Ok(s),
            None => Err(GraphError::edge_out_of_range(index, count)),
        }
    }
    pub fn set_edge<T>(&mut self, undirected: T, mut connection: usize) -> GraphResult<usize>
    where
        T: Into<DirectedEdge>,
    {
        let new = &mut connection;
        let old = self.mut_edge(undirected)?;
        std::mem::swap(new, old);
        Ok(*new)
    }

    pub fn connect<T>(&mut self, edge: T) -> GraphResult<usize>
    where
        T: Into<DirectedEdge>,
    {
        let edge = self.mut_edge(edge)?;
        *edge = edge.saturating_add(1);
        Ok(*edge)
    }
    pub fn disconnect<T>(&mut self, edge: T) -> GraphResult<usize>
    where
        T: Into<DirectedEdge>,
    {
        let edge = self.mut_edge(edge)?;
        *edge = edge.saturating_sub(1);
        Ok(*edge)
    }
}

fn edge_index(edge: &DirectedEdge, rank: usize) -> usize {
    edge.from * rank + edge.goto
}
