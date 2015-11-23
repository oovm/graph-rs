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
/// use graph_theory::Graph;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectedEdge {
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

impl Edge for DirectedEdge {
    type Weight = ();

    fn from(&self) -> usize {
        self.from
    }

    fn goto(&self) -> usize {
        self.goto
    }
}

impl From<(usize, usize)> for DirectedEdge {
    fn from(ordinal: (usize, usize)) -> Self {
        Self { from: ordinal.0 - 1, goto: ordinal.1 - 1 }
    }
}

impl DirectedEdge {
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
