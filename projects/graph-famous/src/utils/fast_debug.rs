use graph_types::GraphKind;
use std::fmt::{Debug, Formatter};

/// All information are O(1)
pub struct GraphFastDebug {
    name: &'static str,
    kind: GraphKind,
    rank: usize,
    nodes: usize,
    edges: usize,
}

impl Debug for GraphFastDebug {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(self.name)
            .field("kind", &self.kind)
            .field("rank", &self.rank)
            .field("node", &self.nodes)
            .field("edge", &self.edges)
            .finish()
    }
}
