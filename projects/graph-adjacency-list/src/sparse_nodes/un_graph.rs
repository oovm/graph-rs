use super::*;


impl GraphEngine for UnGraph {
    fn count_nodes(&self) -> usize {
        self.head_nodes.len()
    }

    fn insert_node(&mut self, node: usize) -> usize {
        let node_id = node as u32;
        self.head_nodes.entry(node_id).or_insert_with(|| NodeNeighbors {
            end_nodes: BTreeMap::new(),
        });
        node
    }
    fn remove_node_with_edges(&mut self, node_id: usize) {
        let node_id = node_id as u32;
        self.head_nodes.remove(&node_id);
    }

    fn insert_edge_with_nodes<E: Edge>(&mut self, edge: E) -> EdgeInsertResult {
        let lhs = edge.lhs() as u32;
        let rhs = edge.rhs() as u32;
        match edge.direction() {
            EdgeDirection::Disconnect => {
                EdgeInsertResult::Nothing
            }
            EdgeDirection::Dynamic | EdgeDirection::Forward => {
                let e1 = self.insert_directed_edge(lhs, rhs);
                EdgeInsertResult::OneEdge(e1)
            }
            EdgeDirection::Reverse => {
                let e1 = self.insert_directed_edge(rhs, lhs);
                EdgeInsertResult::OneEdge(e1)
            }
            EdgeDirection::TwoWay => {
                let e1 = self.insert_directed_edge(lhs, rhs);
                let e2 = self.insert_directed_edge(rhs, lhs);
                EdgeInsertResult::TwoEdges(e1, e2)
            }
        }
    }


    fn remove_edge<E>(&mut self, edge: E) where E: Into<EdgeRemoveAction> {
        match edge.into() {
            EdgeRemoveAction::EdgeID(v) => {
                let edge_id = v as u32;
                for (_, node) in self.head_nodes.iter_mut() {
                    node.end_nodes.remove(&edge_id);
                    // edge id is unique in the graph
                    break;
                }
            }
            EdgeRemoveAction::Directed(v) => {
                let start_node_id = v.lhs() as u32;
                let end_node_id = v.rhs() as u32;
                if let Some(node) = self.head_nodes.get_mut(&start_node_id) {
                    // notice that there are multiple edges between two nodes
                    node.end_nodes.retain(|_, &mut v| v != end_node_id);
                }
            }
            EdgeRemoveAction::Undirected(v) => {
                panic!("remove undirected edge {v} is not supported in directed graph");
            }
        }
    }
    fn count_edges(&self) -> usize {
        self.head_nodes.iter().map(|(_, v)| v.end_nodes.len()).sum()
    }
}

impl UnGraph {
    fn new_edge_id(&mut self) -> u32 {
        let new = self.last_edge.saturating_add(1);
        self.last_edge = new;
        new
    }
    /// The low level interface for inserting a undirected edge
    ///
    /// Always from min to max, so just insert one
    pub fn insert_undirected_edge(&mut self, min: u32, max: u32) -> usize {
        debug_assert!(min <= max, "min index {} is larger than max index {}", min, max);
        let new_edge_id = self.new_edge_id();
        let min_node = self.head_nodes.entry(min).or_insert_with(|| NodeNeighbors {
            end_nodes: BTreeMap::new(),
        });
        min_node.end_nodes.insert(new_edge_id, max);
        new_edge_id as usize
    }
}
