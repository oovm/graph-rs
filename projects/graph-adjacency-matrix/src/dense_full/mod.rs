use graph_types::{Edge, EdgeInsertID, EdgeQuery, EdgesVisitor, GraphEngine, GraphKind, MutableGraph, NodesVisitor};
use ndarray::{Array2, ArrayView2, ArrayViewMut2};
use std::{
    fmt::{Debug, Display},
    mem::size_of,
    ops::Range,
};

mod display;

#[derive(Clone, Debug)]
pub struct AdjacencyMatrix {
    edges: u32,
    degree: Range<u32>,
    matrix: Array2<u32>,
}

impl GraphEngine for AdjacencyMatrix {
    fn graph_kind(&self) -> GraphKind {
        GraphKind::Directed
    }

    fn has_node(&self, node_id: usize) -> Option<usize> {
        if node_id < self.matrix.shape()[0] { Some(node_id) } else { None }
    }

    fn count_nodes(&self) -> usize {
        self.matrix.shape()[0]
    }

    fn traverse_nodes(&self) -> NodesVisitor<Self> {
        NodesVisitor::range(self, 0..self.count_nodes())
    }

    /// It will return id but id is meaningless
    fn has_edge<E: Into<EdgeQuery>>(&self, edge: E) -> Option<usize> {
        match edge.into() {
            EdgeQuery::EdgeID(_) => {
                panic!("Cannot query edge id on adjacency matrix")
            }
            EdgeQuery::Directed(e) => {
                if e.from < self.matrix.shape()[0] && e.goto < self.matrix.shape()[1] {
                    Some(0)
                }
                else {
                    None
                }
            }
            EdgeQuery::Undirected(e) => {
                if e.from < self.matrix.shape()[0] && e.goto < self.matrix.shape()[1] {
                    Some(0)
                }
                else {
                    None
                }
            }
        }
    }

    fn traverse_edges(&self) -> EdgesVisitor<Self> {
        todo!()
    }
    fn count_edges(&self) -> usize {
        self.edges as usize
    }

    fn size_hint(&self) -> usize {
        size_of::<u32>() * (self.matrix.len() + 3) + size_of::<Vec<u32>>()
    }
}

impl MutableGraph for AdjacencyMatrix {
    /// It will return id but id is meaningless
    fn insert_node(&mut self, _: usize) -> usize {
        panic!("Cannot insert node on adjacency matrix")
    }

    fn remove_node(&mut self, _node_id: usize) {
        todo!()
    }

    /// It means clean all edges
    fn remove_node_with_edges(&mut self, _node_id: usize) {
        todo!()
    }
    fn insert_edge<E: Edge>(&mut self, _edge: E) -> EdgeInsertID {
        todo!()
    }

    fn insert_edge_with_nodes<E: Edge>(&mut self, _edge: E) -> EdgeInsertID {
        todo!()
    }

    fn remove_edge<E>(&mut self, edge: E)
    where
        E: Into<EdgeQuery>,
    {
        match edge.into() {
            EdgeQuery::EdgeID(_) => {
                panic!("Cannot query edge id on adjacency matrix")
            }
            EdgeQuery::Directed(_) => {}
            EdgeQuery::Undirected(_) => {}
        }
    }
}

impl AdjacencyMatrix {
    pub fn new(nodes: usize) -> Self {
        Self { edges: 0, degree: 0..0, matrix: Array2::zeros((nodes, nodes)) }
    }
    pub fn get_matrix(&self) -> ArrayView2<u32> {
        self.matrix.view()
    }
    pub fn mut_matrix(&mut self) -> ArrayViewMut2<u32> {
        self.matrix.view_mut()
    }
}

#[test]
fn fast_test() {
    let mut matrix = AdjacencyMatrix::new(10);
    println!("{:?}", matrix.size_hint());
    matrix.mut_matrix().fill(1);

    println!("{:?}", matrix.size_hint())
}

// impl AdjacencyMatrix {
//     pub fn new(nodes: usize) -> Self {
//         Self { edges: vec![0; nodes * nodes] }
//     }
//     pub fn nodes(&self) -> usize {
//         // edges == nodes * nodes
//         (self.edges.len() as f64).sqrt() as usize
//     }
//     pub fn edges(&self) -> usize {
//         self.edges.len()
//     }
//     pub fn max_degree(&self) -> usize {
//         self.edges.iter().max().copied().unwrap_or(0)
//     }
//     pub fn min_degree(&self) -> usize {
//         self.edges.iter().min().copied().unwrap_or(0)
//     }
//     pub fn get_edge<T>(&self, undirected: T) -> GraphResult<usize>
//     where
//         T: Into<DirectedEdge>,
//     {
//         let edge = undirected.into();
//         let index = edge_index(&edge, self.nodes());
//         match self.edges.get(index) {
//             Some(s) => Ok(*s),
//             None => Err(GraphError::edge_out_of_range(index, self.edges.len())),
//         }
//     }
//     pub fn mut_edge<T>(&mut self, undirected: T) -> GraphResult<&mut usize>
//     where
//         T: Into<DirectedEdge>,
//     {
//         let edge = undirected.into();
//         let index = edge_index(&edge, self.nodes());
//         let count = self.edges.len();
//         match self.edges.get_mut(index) {
//             Some(s) => Ok(s),
//             None => Err(GraphError::edge_out_of_range(index, count)),
//         }
//     }
//     pub fn set_edge<T>(&mut self, undirected: T, mut connection: usize) -> GraphResult<usize>
//     where
//         T: Into<DirectedEdge>,
//     {
//         let new = &mut connection;
//         let old = self.mut_edge(undirected)?;
//         std::mem::swap(new, old);
//         Ok(*new)
//     }
//
//     pub fn connect<T>(&mut self, edge: T) -> GraphResult<usize>
//     where
//         T: Into<DirectedEdge>,
//     {
//         let edge = self.mut_edge(edge)?;
//         *edge = edge.saturating_add(1);
//         Ok(*edge)
//     }
//     pub fn disconnect<T>(&mut self, edge: T) -> GraphResult<usize>
//     where
//         T: Into<DirectedEdge>,
//     {
//         let edge = self.mut_edge(edge)?;
//         *edge = edge.saturating_sub(1);
//         Ok(*edge)
//     }
// }
//
// fn edge_index(edge: &DirectedEdge, rank: usize) -> usize {
//     edge.from * rank + edge.goto
// }
