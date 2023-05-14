pub mod get_iter;
pub mod indexed;
pub mod mut_iter;
pub mod node_range_visitor;
pub mod node_slice_visitor;

use crate::GraphEngine;
use std::{
    any::type_name,
    fmt::{Debug, Formatter},
    ops::{Bound, Range, RangeBounds},
};

/// used to determine the direction of an edge
pub type NodeID = usize;

/// used to determine the direction of an edge
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NodeNeighborFilter {
    Both = 0,
    /// used to determine the direction of an edge
    InComing = 1,
    /// used to determine the direction of an edge
    OutGoing = 2,
}

/// used to determine the direction of an edge
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VisitOrder {
    /// used to determine the direction of an edge
    Topological = 0,
    /// used to determine the direction of an edge
    DepthFirst = 1,
    /// used to determine the direction of an edge
    BreadthFirst = 2,
    /// used to determine the direction of an edge
    PreOrder = 3,
    /// used to determine the direction of an edge
    PostOrder = 4,
}

#[derive(Copy, Clone, Debug)]
pub enum NodeDegree {
    Directed {
        /// used to determine the direction of an edge
        in_coming: usize,
        /// used to determine the direction of an edge
        out_going: usize,
    },
    Undirected {
        /// used to determine the direction of an edge
        total: usize,
    },
}

impl NodeDegree {
    /// used to determine the direction of an edge
    pub fn total(&self) -> usize {
        match self {
            NodeDegree::Directed { in_coming, out_going } => in_coming + out_going,
            NodeDegree::Undirected { total } => *total,
        }
    }
}

/// Represents a node in a graph
///
/// ```
/// use graph_theory::GraphEngine;
/// ```
pub trait Node {
    /// The type of the node's index
    type Name: AsRef<str>;
    /// The index of the node, all kinds of nodes must have an index
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::GraphEngine;
    /// ```
    fn index(&self) -> NodeID;
    /// The weight of the node, only weighted graph nodes has a weight
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::GraphEngine;
    /// ```
    fn name(&self) -> Self::Name;
}
