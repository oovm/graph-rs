// platform dependent edge
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ShortEdge {
    pub from: u32,
    pub goto: u32,
}

impl ShortEdge {
    pub const fn new(from: usize, goto: usize) -> ShortEdge {
        Self { from: from as u32, goto: goto as u32 }
    }
}
