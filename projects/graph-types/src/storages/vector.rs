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
/// use graph_theory::GraphEngine;
/// ```
#[derive(Clone, Debug)]
pub struct ListStorage<T> {
    nodes: Vec<T>,
    edges: Vec<T>,
}

impl<T> Default for ListStorage<T>
where
    T: Default,
{
    fn default() -> Self {
        Self { nodes: Vec::new(), edges: Vec::new() }
    }
}

impl<'i, V> EntryEngine<'i, V> for ListStorage<V> {
    type EntryRef = ();
    type EntryMut = ();

    fn try_entry<Q: Into<Query>>(&'i self, query: Q) -> Result<Self::EntryRef, GraphError> {
        match query.into() {
            Query::NodeID(v) => match self.nodes.get(v) {
                None => {}
                Some(_) => {}
            },
            Query::EdgeID(v) => match self.nodes.get(v) {
                None => {}
                Some(_) => {}
            },
            Query::Directed(_) => Err(GraphError::custom("")),
            Query::Undirected(_) => Err(GraphError::custom("")),
        }
    }

    fn mut_entry<Q: Into<Query>>(&'i mut self, query: Q) -> Result<Self::EntryMut, GraphError> {
        todo!()
    }
}
