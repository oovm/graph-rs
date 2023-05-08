use super::*;
use std::{
    any::type_name,
    fmt::{Debug, Formatter},
    ops::{Bound, Range, RangeBounds},
    slice::SliceIndex,
};

/// # Arguments
///
/// * `index`:
///
/// returns: Option<Cow<Self::Node>>
///
/// # Examples
///
/// ```
/// use graph_theory::GraphEngine;
/// ```
pub struct NodesVisitor<'i, G: GraphEngine + ?Sized> {
    graph: &'i G,
    indexer: Box<dyn DoubleEndedIterator<Item = usize>>,
}

impl<'i, G: GraphEngine + ?Sized> Debug for NodesVisitor<'i, G> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = type_name::<G>();
        let nodes = self.graph.count_nodes();
        f.debug_struct("NodesVisitor").field("graph", &name).field("nodes", &nodes).finish()
    }
}

impl<'i, G: GraphEngine> Iterator for NodesVisitor<'i, G> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.indexer.next()?;
        match self.graph.has_node(index) {
            Some(s) => Some(s),
            None => self.next(),
        }
    }
}

impl<'i, G: GraphEngine> DoubleEndedIterator for NodesVisitor<'i, G> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let index = self.indexer.next_back()?;
        match self.graph.has_node(index) {
            Some(s) => Some(s),
            None => self.next_back(),
        }
    }
}

impl<'i, G: GraphEngine + ?Sized> NodesVisitor<'i, G> {
    /// Create a custom node visitor
    pub fn new<I>(graph: &'i G, indexer: I) -> Self
    where
        I: DoubleEndedIterator<Item = usize> + 'static,
    {
        Self { graph, indexer: Box::new(indexer) }
    }
    /// Create a range based node visitor
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::GraphEngine;
    /// ```
    /// # Arguments
    pub fn range<R>(graph: &'i G, range: R) -> Self
    where
        R: RangeBounds<usize>,
    {
        let start = match range.start_bound() {
            Bound::Included(s) => *s,
            Bound::Excluded(s) => *s + 1,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(s) => *s + 1,
            Bound::Excluded(s) => *s,
            Bound::Unbounded => {
                panic!("Upper bound must be specified")
            }
        };
        Self { graph, indexer: Box::new(Range { start, end }) }
    }
    /// # Arguments
    pub fn slice<S>(graph: &'i G, slice: S) -> Self
    where
        S: SliceIndex<[usize]>,
    {
        todo!()
    }
}
