use crate::GraphEngine;

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
pub struct GetEdgesVisitor<'a, G: GraphEngine + ?Sized> {
    graph: &'a G,
    index: usize,
}

impl<'a, G> GetEdgesVisitor<'a, G>
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
    pub fn new(graph: &'a G) -> Self {
        Self { graph, index: 0 }
    }
}
