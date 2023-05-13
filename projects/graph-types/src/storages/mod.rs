use crate::{errors::GraphError, EdgeQuery, EntryEngine, GraphEntry, NodeQuery, Query};
use std::ops::{Deref, DerefMut};

mod btree;
mod btree_default;
mod vector;

pub use self::{btree::DictStorage, btree_default::DictDefault, vector::ListStorage};
