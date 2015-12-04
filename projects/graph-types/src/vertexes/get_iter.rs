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
#[derive(Debug)]
pub struct GetNodesVisitor<'i, G: GraphEngine + ?Sized> {
    graph: &'i G,
    index: usize,
}

impl<'i, G: GraphEngine> Iterator for GetNodesVisitor<'i, G> {
    type Item = Cow<'i, usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.graph.count_nodes() {
            return None;
        }
        let index = self.index;
        self.index += 1;
        // self.graph.get_node_id(index);
        todo!()
    }
}

impl<'i, G: GraphEngine> DoubleEndedIterator for GetNodesVisitor<'i, G> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index >= self.graph.count_nodes() {
            return None;
        }
        let index = self.graph.count_nodes() - self.index - 1;
        self.index += 1;
        // self.graph.get_node_id(index);
        todo!()
    }
}

impl<'i, G: GraphEngine + ?Sized> GetNodesVisitor<'i, G> {
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
    pub fn new(graph: &'i G) -> Self {
        Self { graph, index: 0 }
    }
}
