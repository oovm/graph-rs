#![doc = include_str!("readme.md")]

mod storage;

pub use self::storage::{btree::DictStorage, btree_default::DictDefault, vector::ListStorage};
pub use crate::entries::{EntryName, EntryWeight};
