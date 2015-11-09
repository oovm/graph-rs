use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectedEdge {
    pub from: usize,
    pub goto: usize,
}

impl Edge for DirectedEdge {
    fn from(&self) -> NodeIndex {
        self.from
    }

    fn goto(&self) -> NodeIndex {
        self.goto
    }
}
