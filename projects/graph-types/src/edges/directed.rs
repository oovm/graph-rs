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

impl DirectedEdge {
    pub fn max_index(&self) -> usize {
        max(self.from, self.goto)
    }
    pub fn min_index(&self) -> usize {
        min(self.from, self.goto)
    }
    pub fn as_range(&self) -> Range<usize> {
        self.min_index()..self.max_index()
    }
}
