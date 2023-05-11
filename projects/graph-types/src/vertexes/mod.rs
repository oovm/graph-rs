pub mod get_iter;
pub mod indexed;
pub mod mut_iter;

use crate::GraphEngine;
use std::{
    any::type_name,
    fmt::{Debug, Formatter},
    ops::{Bound, Range, RangeBounds},
};

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
    fn index(&self) -> usize;
    /// The weight of the node, only weighted graph nodes has a weight
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::GraphEngine;
    /// ```
    fn name(&self) -> Self::Name;
}
