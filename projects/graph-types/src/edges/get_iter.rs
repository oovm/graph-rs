use crate::{EdgeQuery, GraphEngine, NodeQuery};
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
pub struct EdgesVisitor<'i, G: GraphEngine + ?Sized> {
    graph: &'i G,
    indexer: Box<dyn DoubleEndedIterator<Item = usize>>,
}

impl<'i, G: GraphEngine + ?Sized> Debug for EdgesVisitor<'i, G> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = type_name::<G>();
        let nodes = self.graph.count_nodes();
        f.debug_struct("EdgeVisitor").field("graph", &name).field("nodes", &nodes).finish()
    }
}

impl<'i, G> Iterator for EdgesVisitor<'i, G>
where
    G: GraphEngine + ?Sized,
{
    type Item = EdgeQuery;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.indexer.next()?;
        if self.graph.has_node(NodeQuery::NodeID(index)).is_some() { Some(EdgeQuery::EdgeID(index)) } else { self.next() }
    }
}

impl<'i, G> DoubleEndedIterator for EdgesVisitor<'i, G>
where
    G: GraphEngine + ?Sized,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let index = self.indexer.next_back()?;
        if self.graph.has_node(NodeQuery::NodeID(index)).is_some() { Some(EdgeQuery::EdgeID(index)) } else { self.next_back() }
    }
}

impl<'i, G> EdgesVisitor<'i, G>
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
}
