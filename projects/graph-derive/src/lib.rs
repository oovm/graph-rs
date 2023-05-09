// #![deny(missing_debug_implementations, missing_copy_implementations)]
// #![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str ! ("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

extern crate proc_macro;

mod derive_graph;

use crate::derive_graph::GraphDerive;

use proc_macro::TokenStream;
use syn::{parse::Parse, parse_macro_input, Error};

pub(crate) mod entries;

/// Derive the `Graph` trait for a struct.
#[proc_macro_derive(Graph, attributes(graph))]
pub fn derive_graph(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as GraphDerive);
    TokenStream::from(input.build())
}
