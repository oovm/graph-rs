use crate::{Entry, Query};
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

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
pub type GraphResult<T = ()> = Result<T, GraphError>;

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
pub struct GraphError {
    kind: Box<GraphErrorKind>,
}
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
#[derive(Clone, Debug)]
pub enum GraphErrorKind {
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
    NotFound {
        /// # Arguments
        entry: Entry,
        /// # Arguments
        index: usize,
    },
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
    OutOfRange {
        /// # Arguments
        entry: Entry,
        /// # Arguments
        index: usize,
        /// # Arguments
        max: usize,
    },
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
    Custom {
        /// # Arguments
        message: String,
    },
}

impl Error for GraphError {}

impl Display for GraphError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}

impl Display for GraphErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GraphErrorKind::NotFound { entry, index } => {
                write!(f, "{entry:?} index {index} not found")
            }
            GraphErrorKind::OutOfRange { entry, index, max } => {
                write!(f, "{entry:?} index {index} is out of range, max index is {max}")
            }
            GraphErrorKind::Custom { message } => f.write_str(message),
        }
    }
}

impl GraphError {
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
    pub fn custom<S>(message: S) -> Self
    where
        S: ToString,
    {
        Self { kind: Box::new(GraphErrorKind::Custom { message: message.to_string() }) }
    }
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
    pub fn not_found<Q: Into<Query>>(query: Q) -> Self {
        let query = query.into();
        Self { kind: Box::new(GraphErrorKind::NotFound { entry: query.entry, index: query.index }) }
    }

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
    pub fn node_not_found(id: usize) -> Self {
        Self { kind: Box::new(GraphErrorKind::NotFound { entry: Entry::Node, index: id }) }
    }
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
    pub fn edge_not_found(id: usize) -> Self {
        Self { kind: Box::new(GraphErrorKind::NotFound { entry: Entry::Edge, index: id }) }
    }
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
    pub fn node_out_of_range(index: usize, max: usize) -> Self {
        Self { kind: Box::new(GraphErrorKind::OutOfRange { entry: Entry::Node, index, max }) }
    }
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
    pub fn edge_out_of_range(index: usize, max: usize) -> Self {
        Self { kind: Box::new(GraphErrorKind::OutOfRange { entry: Entry::Edge, index, max }) }
    }
}
