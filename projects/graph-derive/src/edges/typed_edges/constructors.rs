use super::*;


impl From<(usize, usize)> for UndirectedEdge {
    /// Creates a new edge from the given ordinals (start from 1).
    fn from(ordinal: (usize, usize)) -> Self {
        Self { from: ordinal.0 - 1, goto: ordinal.1 - 1 }
    }
}

impl DirectedEdge {
    /// Creates a new edge from the given indices (start from 0).
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::GraphEngine;
    /// ```
    pub fn new(from: usize, goto: usize) -> Self {
        Self { from, goto }
    }
}

impl From<(usize, usize)> for DirectedEdge {
    fn from(ordinal: (usize, usize)) -> Self {
        Self { from: ordinal.0 - 1, goto: ordinal.1 - 1 }
    }
}



impl UndirectedEdge {
    /// Creates a new edge from the given indices (start from 0).
    pub fn new(from: usize, goto: usize) -> Self {
        Self { from, goto }
    }
    /// Use the edge as a range.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::UndirectedEdge;
    /// let range = UndirectedEdge::new(0, 3).as_range();
    /// assert_eq!(range, 0..3);
    /// ```
    pub fn as_range(&self) -> Range<usize> {
        self.min_index()..self.max_index()
    }
}
