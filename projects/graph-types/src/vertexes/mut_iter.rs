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
pub struct MutNodesVisitor<'a, G: Graph> {
    graph: &'a mut G,
    index: usize,
}
