use crate::{Entry, GraphError, Query, ValueProvider};

#[derive(Clone, Debug)]
pub struct ListStorage<T> {
    nodes: Vec<T>,
    edges: Vec<T>,
}

// noinspection DuplicatedCode
impl<'i, T: 'i> ValueProvider<'i, T> for ListStorage<T> {
    type ValueRef = &'i T;
    type ValueMut = &'i mut T;

    fn get_value(&'i self, query: Query) -> Result<Self::ValueRef, GraphError> {
        match self.get_data(query) {
            Some(item) => Ok(item),
            None => Err(GraphError::not_found(query)),
        }
    }

    fn mut_value(&'i mut self, query: Query) -> Result<Self::ValueMut, GraphError> {
        match self.mut_data(query) {
            Some(item) => Ok(item),
            None => Err(GraphError::not_found(query)),
        }
    }
}

impl<T> ListStorage<T> {
    pub fn get_data(&self, query: Query) -> Option<&T> {
        let item = match query.entry {
            Entry::Node => self.nodes.get(query.index)?,
            Entry::Edge => self.edges.get(query.index)?,
        };
        Some(item)
    }
    pub fn mut_data(&mut self, query: Query) -> Option<&mut T> {
        let item = match query.entry {
            Entry::Node => self.nodes.get_mut(query.index)?,
            Entry::Edge => self.edges.get_mut(query.index)?,
        };
        Some(item)
    }
    pub fn set_data(&mut self, query: Query, data: T) {
        let index = match query.entry {
            Entry::Node => {
                self.nodes.push(data);
                self.nodes.len()
            }
            Entry::Edge => {
                self.edges.push(data);
                self.edges.len()
            }
        };
    }
}
