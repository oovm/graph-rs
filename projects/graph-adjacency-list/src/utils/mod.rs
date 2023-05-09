type NodeID = u32;

// platform dependent edge
#[repr(C)]
#[derive(Debug)]
pub struct ShortEdge {
    pub from: NodeID,
    pub goto: NodeID,
}

impl ShortEdge {
    pub const fn new(from: usize, goto: usize) -> ShortEdge {
        Self { from: from as u32, goto: goto as u32 }
    }
}
