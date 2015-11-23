use super::*;
use std::fmt::{Display, Formatter};

/// [UndirectedEdge](https://reference.wolfram.com/language/ref/UndirectedEdge.html)
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UndirectedEdge {
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
    pub from: usize,
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
    pub goto: usize,
}

impl Display for UndirectedEdge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ↔ {}", self.from + 1, self.goto + 1)
    }
}

impl Edge for UndirectedEdge {
    type Weight = ();

    fn from(&self) -> usize {
        self.min_index()
    }

    fn goto(&self) -> usize {
        self.max_index()
    }
}

impl From<(usize, usize)> for UndirectedEdge {
    fn from(ordinal: (usize, usize)) -> Self {
        Self { from: ordinal.0 - 1, goto: ordinal.1 - 1 }
    }
}

impl UndirectedEdge {
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
    pub fn max_index(&self) -> usize {
        max(self.from, self.goto)
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
    pub fn min_index(&self) -> usize {
        min(self.from, self.goto)
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
    pub fn as_range(&self) -> Range<usize> {
        self.min_index()..self.max_index()
    }
}
