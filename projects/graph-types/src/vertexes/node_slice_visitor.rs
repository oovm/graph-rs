use crate::{GraphEngine, NodeID};
use std::{
    any::type_name,
    fmt::{Debug, Formatter},
    slice::Iter,
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
pub struct NodeSliceVisitor<'i, G>
where
    G: GraphEngine<'i> + ?Sized,
{
    graph: &'i G,
    indexer: Iter<'i, usize>,
}

impl<'i, G> Debug for NodeSliceVisitor<'i, G>
where
    G: GraphEngine<'i> + ?Sized,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = type_name::<G>();
        let nodes = self.graph.count_nodes();
        f.debug_struct("NodeRangeVisitor").field("graph", &name).field("nodes", &nodes).finish()
    }
}
impl<'i, G> Iterator for NodeSliceVisitor<'i, G>
where
    G: GraphEngine<'i> + ?Sized,
{
    type Item = NodeID;

    fn next(&mut self) -> Option<Self::Item> {
        let index = *self.indexer.next()?;
        match self.graph.get_node(index) {
            Ok(_) => Some(index),
            Err(_) => self.next(),
        }
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let index = *self.indexer.nth(n)?;
        match self.graph.get_node(index) {
            Ok(_) => Some(index),
            Err(_) => None,
        }
    }
}

impl<'i, G> DoubleEndedIterator for NodeSliceVisitor<'i, G>
where
    G: GraphEngine<'i> + ?Sized,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let index = *self.indexer.next_back()?;
        match self.graph.get_node(index) {
            Ok(_) => Some(index),
            Err(_) => self.next_back(),
        }
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let index = *self.indexer.nth_back(n)?;
        match self.graph.get_node(index) {
            Ok(_) => Some(index),
            Err(_) => None,
        }
    }
}

impl<'i, G> NodeSliceVisitor<'i, G>
where
    G: GraphEngine<'i> + ?Sized,
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
    pub fn new<'a>(graph: &'i G, slice: &'a [usize]) -> Self
    where
        'a: 'i,
    {
        Self { graph, indexer: slice.iter() }
    }
}
