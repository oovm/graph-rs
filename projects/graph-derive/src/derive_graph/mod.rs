use crate::utils::{easy_display, easy_wolfram, GraphAttribute, ParseResult};
use quote::{quote, ToTokens};
use syn::{
    __private::TokenStream2,
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Error, Ident, ItemStruct,
};
mod easy_graph;
mod easy_table;
mod easy_tuple;

pub enum GraphDerive {
    Undefined,
    EasyTuple(EasyTuple),
    EasyTable(EasyTable),
}

pub struct EasyTuple {
    graph_name: Ident,
    field_type: Ident,
    config: GraphAttribute,
}

pub struct EasyTable {
    graph_name: Ident,
    field_name: Ident,
    field_type: Ident,
    config: GraphAttribute,
}

impl Parse for GraphDerive {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let item = match input.parse::<ItemStruct>() {
            Ok(o) => o,
            Err(_) => Err(Error::new(input.span(), "#[derive(Graph)] only work on struct"))?,
        };
        match GraphDerive::easy_graph(&item) {
            ParseResult::Ok(o) => return Ok(o),
            ParseResult::NotGood => {}
            ParseResult::Bad(e) => Err(e)?,
        }
        todo!()
    }
}

impl ToTokens for GraphDerive {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            GraphDerive::Undefined => {
                todo!()
            }
            GraphDerive::EasyTable(e) => e.to_tokens(tokens),
            GraphDerive::EasyTuple(e) => e.to_tokens(tokens),
        }
    }
}
