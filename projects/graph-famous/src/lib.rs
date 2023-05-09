// #![deny(missing_debug_implementations, missing_copy_implementations)]
// #![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod complete_graph;
mod cycle_graph;
mod star_graph;
pub(crate) mod utils;
mod wheel_graph;

pub use crate::{complete_graph::CompleteGraph, cycle_graph::EasyGraphTable, star_graph::StarGraph, wheel_graph::WheelGraph};
