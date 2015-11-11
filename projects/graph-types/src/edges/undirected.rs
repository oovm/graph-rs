use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UndirectedEdge {
    pub from: usize,
    pub goto: usize,
}

impl Edge for UndirectedEdge {
    fn from(&self) -> NodeIndex {
        self.min_index()
    }

    fn goto(&self) -> NodeIndex {
        self.max_index()
    }
}

impl UndirectedEdge {
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