use super::*;

/// A sparse entry engine that uses a [BTreeMap] to store data.
///
/// # Examples
///
/// ```
/// use graph_theory::GraphEngine;
/// ```
#[derive(Clone, Debug)]
pub struct DictDefault<T> {
    dict: DictStorage<T>,
    node_default: T,
    edge_default: T,
}

impl<T> DictDefault<T> {
    pub fn new(node: T, edge: T) -> Self {
        Self { dict: DictStorage::default(), node_default: node, edge_default: edge }
    }
}
