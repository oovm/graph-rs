pub mod get_iter;
pub mod indexed;
pub mod mut_iter;

use crate::GraphEngine;
use std::{
    any::type_name,
    fmt::{Debug, Formatter},
    ops::{Bound, Range, RangeBounds},
};

/// used to determine the direction of an edge
pub type NodeID = usize;
/// used to determine the direction of an edge

#[derive(Copy, Clone, Debug)]
pub struct NodeDegree {
    /// used to determine the direction of an edge
    pub in_coming: usize,
    /// used to determine the direction of an edge
    pub out_going: usize,
}

impl NodeDegree {
    /// used to determine the direction of an edge
    pub fn total(&self) -> usize {
        self.in_coming + self.out_going
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
