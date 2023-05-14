use crate::utils::ShortEdge;

/// Dense, edge first, adjacency list
#[derive(Debug)]
pub struct AdjacencyEdgeList<const ONE_WAY: bool> {
    nodes: Vec<u32>,
    edges: Vec<ShortEdge>,
}
