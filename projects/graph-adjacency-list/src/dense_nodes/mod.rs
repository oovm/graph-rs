/// Dense, node first, adjacency list
#[derive(Debug)]
pub struct AdjacencyNodeList<const ONE_WAY: bool> {
    head_nodes: Vec<NodeNeighbors>,
    last_edge: u32,
}

#[derive(Debug)]
struct NodeNeighbors {
    end_nodes: Vec<u32>,
}
