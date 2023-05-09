use graph_derive::Graph;
use graph_types::{EdgeQuery, EdgesVisitor, GraphEngine, GraphKind, NodesVisitor};
use serde::{ser::SerializeTupleStruct, Serialize, Serializer};
use std::{
    fmt::{Debug, Formatter},
    mem::size_of,
};

/// https://reference.wolfram.com/language/ref/CycleGraph.html
#[derive(Graph)]
pub struct EasyGraphTable {
    #[easy_graph]
    mask: i32,
}

#[derive(Graph)]
pub struct EasyGraphTuple(#[easy_graph] i32);

impl Debug for EasyGraphTable {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CycleGraph")
            .field("kind", &self.graph_kind())
            .field("rank", &self.rank())
            .field("nodes", &self.count_nodes())
            .field("edges", &self.count_edges())
            .finish()
    }
}

impl PartialEq for EasyGraphTable {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask
    }
}

impl EasyGraphTable {
    pub const fn one_way(rank: usize) -> Self {
        Self { mask: rank as i32 }
    }
    pub const fn two_way(rank: usize) -> Self {
        Self { mask: -(rank as i32) }
    }
    pub const fn rank(&self) -> usize {
        self.mask.abs() as usize
    }
}

impl Serialize for EasyGraphTable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ser = serializer.serialize_tuple_struct("CycleGraph", 1)?;
        ser.serialize_field(&self.mask)?;
        ser.end()
    }
}

impl GraphEngine for EasyGraphTable {
    fn graph_kind(&self) -> GraphKind {
        match self.mask < 0 {
            true => GraphKind::Undirected,
            false => GraphKind::Directed,
        }
    }

    fn has_node(&self, node_id: usize) -> Option<usize> {
        (node_id < self.rank()).then_some(node_id)
    }

    fn count_nodes(&self) -> usize {
        self.rank()
    }

    fn traverse_nodes(&self) -> NodesVisitor<Self> {
        NodesVisitor::range(self, 0..self.count_nodes())
    }

    fn has_edge<E: Into<EdgeQuery>>(&self, edge: E) -> Option<usize> {
        todo!()
    }

    fn traverse_edges(&self) -> EdgesVisitor<Self> {
        todo!()
    }

    fn count_edges(&self) -> usize {
        match self.graph_kind() {
            GraphKind::Directed => self.rank(),
            GraphKind::Undirected => self.rank() * 2,
        }
    }

    /// Takes O(1) space, in fact it's always takes 32 bits.
    ///
    /// ```
    /// use graph_theory::{graph_engines::EasyGraphTable, GraphEngine};
    /// assert_eq!(EasyGraphTable::one_way(3).size_hint(), 4);
    /// assert_eq!(EasyGraphTable::one_way(4).size_hint(), 4);
    /// assert_eq!(EasyGraphTable::two_way(5).size_hint(), 4);
    /// ```
    fn size_hint(&self) -> usize {
        size_of::<EasyGraphTable>()
    }
}
