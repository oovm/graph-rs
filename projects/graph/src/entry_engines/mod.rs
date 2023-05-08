#![doc = include_str!("readme.md")]

mod storage;

pub use self::storage::{btree::DictStorage, vector::ListStorage};
pub use crate::entries::{EntryName, EntryWeight};
