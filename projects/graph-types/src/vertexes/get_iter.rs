use super::*;
use crate::NodeQuery;

/// A double-ended iterator over the nodes of a graph.
///
/// # Examples
///
/// ```
/// use graph_theory::{graph_engines::CycleGraph, GraphEngine};
/// let graph = CycleGraph::two_way(5);
/// let mut visitor = graph.nodes();
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

impl<'i, G> Iterator for NodesVisitor<'i, G>
where
    G: GraphEngine + ?Sized,
{
    type Item = NodeQuery;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'i, G> DoubleEndedIterator for NodesVisitor<'i, G>
where
    G: GraphEngine + ?Sized,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'i, G: GraphEngine + ?Sized> NodesVisitor<'i, G> {
    /// Create a custom node visitor
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::GraphEngine;
    /// ```
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
    /// use graph_theory::{CompleteGraph, NodesVisitor};
    /// let graph = CompleteGraph::new(5);
    /// let mut visitor = NodesVisitor::range(&graph, 1..=2);
    /// assert_eq!(visitor.next(), Some(1));
    /// assert_eq!(visitor.next(), Some(2));
    /// assert_eq!(visitor.next(), None);
    /// ```
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
    /// Create a index based node visitor
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::{CompleteGraph, NodesVisitor};
    /// let graph = CompleteGraph::new(5);
    /// let mut visitor = NodesVisitor::slice(&graph, &[1, 2]);
    /// assert_eq!(visitor.next(), Some(1));
    /// assert_eq!(visitor.next(), Some(2));
    /// assert_eq!(visitor.next(), None);
    /// ```
    pub fn slice(graph: &'i G, slice: &'static [usize]) -> Self {
        // TODO: allow non static slices
        Self { graph, indexer: Box::new(slice.iter().copied()) }
    }
}
