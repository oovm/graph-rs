#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str ! ("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

extern crate proc_macro;

mod derive_graph;

use proc_macro::TokenStream;

#[proc_macro_derive(Graph, attributes(graph))]
pub fn derive_graph(input: TokenStream) -> TokenStream {
    todo!()
}
