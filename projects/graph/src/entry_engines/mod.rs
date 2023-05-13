#![doc = include_str!("readme.md")]

mod storage;

pub use crate::entries::{EntryName, EntryWeight};
pub use graph_types::{errors::*, DictDefault, DictStorage, ListStorage};
