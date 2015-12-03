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
pub struct MutNodesVisitor<'a, G: GraphEngine> {
    graph: &'a mut G,
    index: usize,
}
