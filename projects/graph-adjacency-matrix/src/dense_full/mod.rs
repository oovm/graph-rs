use crate::{utils::AdjacencyCell, DiGraphAM};
use datasize::{DataSize, MemUsageNode};
use graph_types::{
    errors::GraphError,
    placeholder::{PlaceholderEdgeIterator, PlaceholderNodeIterator},
    Edge, EdgeID, EdgeInsertID, EdgeQuery, GraphEngine, GraphKind, IndeterminateEdge, MutableGraph, NodeID, NodeQuery,
    NodeRangeVisitor, NodesVisitor, Query,
};
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    mem::size_of,
    ops::Range,
};

mod display;

#[derive(Clone, Debug)]
pub struct AdjacencyMatrix<const ONE_WAY: bool> {
    rank: usize,
    edges: Vec<IndeterminateEdge>,
    matrix: Vec<AdjacencyCell>,
}

impl<const ONE_WAY: bool> DataSize for AdjacencyMatrix<ONE_WAY> {
    const IS_DYNAMIC: bool = true;
    const STATIC_HEAP_SIZE: usize = 0;

    fn estimate_heap_size(&self) -> usize {
        self.edges.estimate_heap_size() + self.matrix.estimate_heap_size()
    }
    fn estimate_detailed_heap_size(&self) -> MemUsageNode {
        let mut mem = HashMap::new();
        mem.insert("edges", self.edges.estimate_detailed_heap_size());
        mem.insert("matrix", self.matrix.estimate_detailed_heap_size());
        MemUsageNode::Detailed(mem)
    }
}

pub struct DiGraphBridges<'a> {
    graph: &'a DiGraphAM,
    task: Task,
    index: usize,
}
pub enum Task {
    Empty,
}

impl<'a> Iterator for DiGraphBridges<'a> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a> DoubleEndedIterator for DiGraphBridges<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl GraphEngine for DiGraphAM {
    type NodeIterator = Range<usize>;
    type NeighborIterator = PlaceholderNodeIterator;
    type EdgeIterator = Range<usize>;
    type BridgeIterator = DiGraphBridges<'a>;

    fn graph_kind(&self) -> GraphKind {
        GraphKind::Directed
    }

    fn get_node_id<Q: Into<NodeQuery>>(&self, node: Q) -> Result<NodeID, GraphError> {
        match node.into() {
            NodeQuery::NodeID(v) => NodeQuery::check_range(v, self.matrix.len()),
        }
    }

    fn count_nodes(&self) -> usize {
        self.matrix.len()
    }

    fn all_node_ids(&self) -> Self::NodeIterator {
        0..self.rank
    }

    fn get_edge_id<Q: Into<EdgeQuery>>(&self, edge: Q) -> Result<EdgeID, GraphError> {
        match edge.into() {
            EdgeQuery::EdgeID(v) => {
                let max = self.edges.len();
                if v < max { Ok(v) } else { Err(GraphError::edge_out_of_range(v, max)) }
            }
            EdgeQuery::Dynamic(v) => {
                todo!()
            }
            EdgeQuery::Directed(v) => {
                todo!()
            }
            EdgeQuery::Undirected(v) => v.as_unsupported(),
        }
    }

    fn all_edge_ids(&self) -> Self::EdgeIterator {
        0..self.edges.len()
    }

    fn get_bridges<Q: Into<EdgeQuery>>(&self, edge: Q) -> Self::BridgeIterator {
        match edge.into() {
            EdgeQuery::EdgeID(_) => {
                todo!()
            }
            EdgeQuery::Dynamic(v) => {
                todo!()
            }
            EdgeQuery::Directed(v) => {
                todo!()
            }
            EdgeQuery::Undirected(v) => todo!(),
        }
    }

    fn all_bridges(&self) -> Self::BridgeIterator {
        todo!()
    }

    fn count_edges(&self) -> usize {
        self.edges.len()
    }

    fn size_hint(&self) -> usize {
        size_of::<Self>()
            + self.edges.capacity() * size_of::<IndeterminateEdge>()
            + self.matrix.capacity() * size_of::<AdjacencyCell>()
    }
}

impl DiGraphAM {
    pub fn new(nodes: usize, edges: usize) -> Self {
        Self {
            rank: nodes,
            edges: vec![IndeterminateEdge { from: 0, goto: 0 }; edges],
            matrix: vec![AdjacencyCell::default(); nodes * nodes],
        }
    }
    pub fn shrink_to_fit(&mut self) {
        self.edges.shrink_to_fit();
        self.matrix.shrink_to_fit();
    }
}

#[test]
fn fast_test() {
    let mut matrix = DiGraphAM::new(11, 13);
    println!("{:?}", matrix.size_hint());
    // matrix.mut_matrix().fill(1);

    println!("{:#?}", vec![0].estimate_detailed_heap_size())
}
