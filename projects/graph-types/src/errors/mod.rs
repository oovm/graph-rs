use crate::{Entry, Query};
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
    ops::DerefMut,
    path::Path,
};

/// The result type alias of a graph operation, see [`GraphError`] & [`GraphErrorKind`] for more.
///
/// # Examples
///
/// ```
/// use graph_theory::GraphResult;
/// ```
pub type GraphResult<T = ()> = Result<T, GraphError>;

/// The error type of a graph operation, see [`GraphErrorKind`] for more.
///
/// # Size
///
/// The size of this struct is always 8 bytes.
///
/// ```
/// use graph_theory::GraphError;
/// assert_eq!(std::mem::size_of::<GraphError>(), 8);
/// ```
///
/// # Examples
///
/// ```
/// use graph_theory::GraphError;
/// GraphError::node_not_found(0); // node id 0 not found
/// GraphError::edge_out_of_range(0, 5); // edge id 0 out of range (max 5)
/// GraphError::custom("user angry"); // user is angry now
/// ```
#[derive(Debug)]
pub struct GraphError {
    kind: Box<GraphErrorKind>,
}

/// The real error type of a graph operation, it will not be exposed to the user, see [`GraphError`] for more.
///
/// # Examples
///
/// ```
/// use graph_theory::Graph;
/// ```
#[derive(Debug)]
pub enum GraphErrorKind {
    /// Some index is not found in storage.
    ///
    /// # Examples
    ///
    /// - edge id 0 out of range
    ///
    /// ```
    /// use graph_theory::{GraphError, Query};
    /// GraphError::not_found(Query::edge(0));
    /// GraphError::edge_not_found(0);
    /// ```
    NotFound {
        /// The entry type of the error.
        entry: Entry,
        /// The entry index you want to access.
        index: usize,
    },
    /// Some index is out of range of storage.
    ///
    /// Max index is 5, but you query index 6.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::{GraphError, Query};
    /// GraphError::not_found(Query::edge(0));
    /// GraphError::node_out_of_range(6, 5);
    /// ```
    OutOfRange {
        /// The entry type of the error.
        entry: Entry,
        /// The entry index you want to access.
        index: usize,
        /// All elements count of the storage.
        max: usize,
    },
    /// An IO error occurred.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_theory::{GraphError, Query};
    /// ```
    IO {
        /// The position of the error occurred.
        entry: String,
        /// The wrapped IO error.
        error: std::io::Error,
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
            GraphErrorKind::IO { entry, error } if entry.is_empty() => {
                write!(f, "IO error: {error}", error = error)
            }
            GraphErrorKind::IO { entry, error } => {
                write!(f, "IO error at {entry:?}: {error}", entry = entry, error = error)
            }
        }
    }
}

impl GraphError {
    /// Some index is not found in storage.
    ///
    /// # Examples
    ///
    /// - edge id 0 out of range
    ///
    /// ```
    /// use graph_theory::{GraphError, Query};
    /// GraphError::not_found(Query::edge(0));
    /// GraphError::edge_not_found(0);
    /// ```
    pub fn custom<S>(message: S) -> Self
    where
        S: ToString,
    {
        Self { kind: Box::new(GraphErrorKind::Custom { message: message.to_string() }) }
    }
    /// Some index is not found in storage.
    ///
    /// # Examples
    ///
    /// - edge id 0 out of range
    ///
    /// ```
    /// use graph_theory::{GraphError, Query};
    /// GraphError::not_found(Query::edge(0));
    /// GraphError::edge_not_found(0);
    /// ```
    pub fn not_found<Q: Into<Query>>(query: Q) -> Self {
        let query = query.into();
        Self { kind: Box::new(GraphErrorKind::NotFound { entry: query.entry, index: query.index }) }
    }
    /// Some index is not found in storage.
    ///
    /// # Examples
    ///
    /// - edge id 0 out of range
    ///
    /// ```
    /// use graph_theory::{GraphError, Query};
    /// GraphError::not_found(Query::edge(0));
    /// GraphError::edge_not_found(0);
    /// ```
    pub fn node_not_found(id: usize) -> Self {
        Self { kind: Box::new(GraphErrorKind::NotFound { entry: Entry::Node, index: id }) }
    }
    /// Some index is not found in storage.
    ///
    /// # Examples
    ///
    /// - edge id 0 out of range
    ///
    /// ```
    /// use graph_theory::{GraphError, Query};
    /// GraphError::not_found(Query::edge(0));
    /// GraphError::edge_not_found(0);
    /// ```
    pub fn edge_not_found(id: usize) -> Self {
        Self { kind: Box::new(GraphErrorKind::NotFound { entry: Entry::Edge, index: id }) }
    }
    /// Fill where the io error occurred.
    ///
    /// # Examples
    ///
    /// - edge id 0 out of range
    ///
    /// ```
    /// use graph_theory::{GraphError, Query};
    /// GraphError::not_found(Query::edge(0));
    /// GraphError::edge_not_found(0);
    /// ```
    pub fn with_io_path<P>(mut self, path: P) -> Self
    where
        P: AsRef<Path>,
    {
        match self.kind.deref_mut() {
            GraphErrorKind::IO { entry, error: _ } => {
                *entry = path.as_ref().to_string_lossy().to_string();
                self
            }
            _ => self,
        }
    }
    /// Some index is not found in storage.
    ///
    /// # Examples
    ///
    /// - edge id 0 out of range
    ///
    /// ```
    /// use graph_theory::{GraphError, Query};
    /// GraphError::not_found(Query::edge(0));
    /// GraphError::edge_not_found(0);
    /// ```
    pub fn node_out_of_range(index: usize, max: usize) -> Self {
        Self { kind: Box::new(GraphErrorKind::OutOfRange { entry: Entry::Node, index, max }) }
    }
    /// Some index is not found in storage.
    ///
    /// # Examples
    ///
    /// - edge id 0 out of range
    ///
    /// ```
    /// use graph_theory::{GraphError, Query};
    /// GraphError::not_found(Query::edge(0));
    /// ```
    pub fn edge_out_of_range(index: usize, max: usize) -> Self {
        Self { kind: Box::new(GraphErrorKind::OutOfRange { entry: Entry::Edge, index, max }) }
    }
}
