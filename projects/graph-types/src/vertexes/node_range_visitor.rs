use crate::{GraphEngine, NodeQuery};
use std::{
    any::type_name,
    fmt::{Debug, Formatter},
    ops::{Bound, Range, RangeBounds},
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
pub struct NodeRangeVisitor<'i, G: GraphEngine + ?Sized> {
    graph: &'i G,
    indexer: Range<usize>,
}

impl<'i, G: GraphEngine + ?Sized> Debug for NodeRangeVisitor<'i, G> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = type_name::<G>();
        let nodes = self.graph.count_nodes();
        f.debug_struct("NodeRangeVisitor").field("graph", &name).field("nodes", &nodes).finish()
    }
}

impl<'i, G> Iterator for NodeRangeVisitor<'i, G>
where
    G: GraphEngine + ?Sized,
{
    type Item = NodeQuery;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.indexer.next()?;
        let query = NodeQuery::NodeID(index);
        match self.graph.has_node(query) {
            Some(_) => Some(query),
            None => self.next(),
        }
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let id = self.indexer.nth(n)?;
        let query = NodeQuery::NodeID(id);
        let i = self.graph.has_node(query)?;
        Some(NodeQuery::NodeID(i))
    }
}

impl<'i, G> DoubleEndedIterator for NodeRangeVisitor<'i, G>
where
    G: GraphEngine + ?Sized,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let index = self.indexer.next_back()?;
        let query = NodeQuery::NodeID(index);
        match self.graph.has_node(query) {
            Some(_) => Some(query),
            None => self.next_back(),
        }
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let id = self.indexer.nth_back(n)?;
        let query = NodeQuery::NodeID(id);
        let i = self.graph.has_node(query)?;
        Some(NodeQuery::NodeID(i))
    }
}

impl<'i, G> NodeRangeVisitor<'i, G>
where
    G: GraphEngine + ?Sized,
{
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
    pub fn new<R>(graph: &'i G, range: R) -> Self
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
        Self { graph, indexer: Range { start, end } }
    }
}
