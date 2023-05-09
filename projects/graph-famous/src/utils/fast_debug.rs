use crate::{CompleteGraph, CycleGraph, StarGraph, WheelGraph};
use graph_types::{GraphEngine, GraphKind};
use std::fmt::{Debug, Display, Formatter};

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

// impl Display for GraphFastDebug {
//     #[inline]
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         Debug::fmt(self, f)
//     }
// }

macro_rules! impl_debug {
    ($($t:ty),* $(,)?) => {
        $(
            impl Debug for $t {
                fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                    Debug::fmt(&GraphFastDebug {
                        name: stringify!($t),
                        kind: self.graph_kind(),
                        rank: self.rank(),
                        nodes: self.count_nodes(),
                        edges: self.count_edges(),
                    }, f)
                }
            }
        )*
    };
}

// impl_debug![StarGraph, CycleGraph, WheelGraph, CompleteGraph];
