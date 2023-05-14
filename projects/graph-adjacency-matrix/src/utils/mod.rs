#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct AdjacencyCell {
    pub node_degree: u32,
    pub edge_start: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct AdjacencyEdge {
    pub from: u32,
    pub goto: u32,
}
