use std::any::type_name;
use std::fmt::{Debug, Formatter};
use std::ops::{Bound, Range, RangeBounds};
use std::slice::SliceIndex;
use super::*;

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
pub struct GetNodesVisitor<'i, G: GraphEngine + ?Sized> {
    graph: &'i G,
    indexer: Box<dyn DoubleEndedIterator<Item=usize>>,
}

impl<'i, G: GraphEngine + ?Sized> Debug for GetNodesVisitor<'i, G> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = type_name::<G>();
        let nodes = self.graph.count_nodes();
        f.debug_struct("GetNodesVisitor")
            .field("graph", &name)
            .field("nodes", &nodes)
            .finish()
    }
}

impl<'i, G: GraphEngine> Iterator for GetNodesVisitor<'i, G> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.indexer.next()?;
        match self.graph.has_node(index) {
            Some(s) => {
                Some(s)
            }
            None => {
                self.next()
            }
        }
    }
}

impl<'i, G: GraphEngine> DoubleEndedIterator for GetNodesVisitor<'i, G> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let index = self.indexer.next_back()?;
        match self.graph.has_node(index) {
            Some(s) => {
                Some(s)
            }
            None => {
                self.next_back()
            }
        }
    }
}

impl<'i, G: GraphEngine + ?Sized> GetNodesVisitor<'i, G> {
    /// Create a new GetNodesVisitor
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
            Bound::Included(s) => {
                *s
            }
            Bound::Excluded(s) => {
                *s + 1
            }
            Bound::Unbounded => {
                0
            }
        };
        let end = match range.end_bound() {
            Bound::Included(s) => {
                *s + 1
            }
            Bound::Excluded(s) => {
                *s
            }
            Bound::Unbounded => {
                panic!("Upper bound must be specified")
            }
        };
        Self {
            graph,
            indexer: Box::new(Range {
                start,
                end,
            }),
        }
    }
    /// # Arguments
    pub fn slice<S>(graph: &'i G, slice: S) -> Self
        where
            S: SliceIndex<[usize]>,
    {
        todo!()
    }
}
