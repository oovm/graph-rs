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
/// use graph_theory::Graph;
/// ```
#[derive(Debug)]
pub struct MutEdgesVisitor<'a, G: Graph + ?Sized> {
    graph: &'a mut G,
    index: usize,
}

impl<'a, G> MutEdgesVisitor<'a, G>
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
    pub fn new(graph: &'a mut G) -> Self {
        Self { graph, index: 0 }
    }
}
