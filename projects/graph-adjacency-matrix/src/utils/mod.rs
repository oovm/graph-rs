use datasize::DataSize;

#[derive(Copy, Clone, Debug, Default)]
pub struct AdjacencyCell {
    node_degree: u32,
    edge_start: u32,
}

impl DataSize for AdjacencyCell {
    const IS_DYNAMIC: bool = false;
    const STATIC_HEAP_SIZE: usize = 0;

    fn estimate_heap_size(&self) -> usize {
        self.node_degree.estimate_heap_size() + self.edge_start.estimate_heap_size()
    }
}
