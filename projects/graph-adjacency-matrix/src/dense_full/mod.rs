use graph_types::{
    errors::GraphError,
    placeholder::{PlaceholderEdgeIterator, PlaceholderNodeIterator},
    Edge, EdgeID, EdgeInsertID, EdgeQuery, GraphEngine, GraphKind, IndeterminateEdge, MutableGraph, NodeID, NodeQuery,
    NodeRangeVisitor, NodesVisitor,
};
use std::{
    fmt::{Debug, Display},
    mem::size_of,
    ops::Range,
};

mod display;

#[derive(Clone, Debug)]
pub struct AdjacencyMatrix {
    matrix: Vec<Cell>,
}

struct Cell {}

impl GraphEngine for AdjacencyMatrix {
    type NodeIterator = PlaceholderNodeIterator;
    type NeighborIterator = PlaceholderNodeIterator;
    type EdgeIterator = PlaceholderNodeIterator;
    type BridgeIterator = PlaceholderEdgeIterator;

    fn graph_kind(&self) -> GraphKind {
        GraphKind::Directed
    }

    fn get_node_id<Q: Into<NodeQuery>>(&self, node: Q) -> Result<NodeID, GraphError> {
        todo!()
    }

    fn count_nodes(&self) -> usize {
        self.matrix.shape()[0]
    }

    fn traverse_nodes(&self) -> Self::NodeIterator {
        todo!()
    }

    fn get_edge_id<Q: Into<EdgeQuery>>(&self, edge: Q) -> Result<EdgeID, GraphError> {
        todo!()
    }

    fn traverse_edges(&self) -> Self::EdgeIterator {
        todo!()
    }

    fn get_bridge<Q: Into<EdgeQuery>>(&self, edge: Q) -> Result<IndeterminateEdge, GraphError> {
        todo!()
    }

    fn traverse_bridges(&self) -> Self::BridgeIterator {
        todo!()
    }

    fn count_edges(&self) -> usize {
        self.edges as usize
    }

    fn size_hint(&self) -> usize {
        size_of::<u32>() * (self.matrix.len() + 3) + size_of::<Vec<u32>>()
    }
}

impl AdjacencyMatrix {
    pub fn new(nodes: usize) -> Self {
        Self { matrix: vec![] }
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
