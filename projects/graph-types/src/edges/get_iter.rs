use crate::Graph;

/// # Arguments
///
/// * `index`:
///
/// returns: Option<Cow<Self::Node>>
///
/// # Examples
///
/// ```
/// use graph_theory::Graph;
/// ```
#[derive(Debug)]
pub struct GetEdgesVisitor<'a, G: Graph + ?Sized> {
    graph: &'a G,
    index: usize,
}

impl<'a, G> GetEdgesVisitor<'a, G>
where
    G: Graph + ?Sized,
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
    /// use graph_theory::Graph;
    /// ```
    pub fn new(graph: &'a G) -> Self {
        Self { graph, index: 0 }
    }
}
