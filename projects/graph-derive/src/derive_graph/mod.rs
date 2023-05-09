use crate::entries::{GraphAttribute, ParseResult};
use quote::{
    quote, ToTokens,
    __private::{ext::RepToTokensExt, TokenStream},
};
use syn::{
    __private::TokenStream2,
    parse::{Parse, ParseStream},
    spanned::Spanned,
    AttrStyle, Attribute, Error, ExprStruct, Fields, Ident, ItemStruct, Meta,
};

mod simple;

pub enum GraphDerive {
    Undefined,
    Easy(EasyGraph),
}

pub struct EasyGraph {
    graph_name: Ident,
    field_name: Option<Ident>,
    field_type: Ident,
}

impl Parse for GraphDerive {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let item = match input.parse::<ItemStruct>() {
            Ok(o) => o,
            Err(_) => Err(Error::new(input.span(), "#[derive(Graph)] only work on struct"))?,
        };
        match EasyGraph::new(&item) {
            ParseResult::Ok(o) => return Ok(GraphDerive::Easy(o)),
            ParseResult::NotGood => {}
            ParseResult::Bad(e) => Err(e)?,
        }
        todo!()
    }
}

// pub fn extra_graph_attribute(expr: &ExprStruct) -> Option<AttrStyle> {
//     for i in &expr.fields {
//         for j in &i.attrs {
//             panic!("{:?}", j.to_token_stream().to_string());
//         }
//     }
//     ParseResult::Ok(())
// }

impl ToTokens for GraphDerive {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            GraphDerive::Undefined => {
                todo!()
            }
            GraphDerive::Easy(e) => e.to_tokens(tokens),
        }
    }
}

impl ToTokens for EasyGraph {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = &self.graph_name;

        let copy = match self.field_name {
            None => {
                quote! {
                impl Copy for #name {}
                impl Clone for TupleGraph {
                #[inline]
                fn clone(&self) -> Self {
                Self(self.0)
                }
                }
                impl Eq for #name {}
                }
            }
            Some(_) => {
                quote! {
                impl Copy for #name {}
                impl Clone for TupleGraph {
                #[inline]
                fn clone(&self) -> Self {
                Self(self.0)
                }
                }

                impl Eq for #name {}
                }
            }
        };
        tokens.extend(copy);
    }
}
