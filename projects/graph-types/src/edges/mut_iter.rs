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
pub struct MutEdgesVisitor<'a, G: GraphEngine + ?Sized> {
    graph: &'a mut G,
    index: usize,
}

impl<'a, G> MutEdgesVisitor<'a, G>
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
    pub fn new(graph: &'a mut G) -> Self {
        Self { graph, index: 0 }
    }
}
