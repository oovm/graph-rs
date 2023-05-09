use crate::entries::ParseResult;
use quote::{quote, ToTokens};
use syn::{
    __private::TokenStream2,
    parse::{Parse, ParseStream},
    spanned::Spanned,
    AttrStyle, Error, ExprStruct, Fields, Ident, ItemStruct,
};

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
            Err(_) => Err(Error::new(input.span(), "`Graph` only work on struct"))?,
        };
        match EasyGraph::new(&item) {
            ParseResult::Ok(o) => return Ok(GraphDerive::Easy(o)),
            ParseResult::NotGood => {}
            ParseResult::Bad(e) => Err(e)?,
        }
        todo!()
    }
}

impl EasyGraph {
    fn new(item: &ItemStruct) -> ParseResult<Self> {
        let name = item.ident.clone();
        let mut is_tuple = false;
        let mut fields = item.fields.iter();
        match fields.next() {
            Some(field) => {
                if field.ident.is_none() {
                    is_tuple = true;
                }
            }
            None => Err(Error::new(item.span(), "Graph must have at least one field"))?,
        }

        for field in &item.fields {
            let is_tuple = field.ident.is_none();
            for i in &field.attrs {
                panic!("{:?}", i.to_token_stream().to_string());
            }
        }
        ParseResult::Ok(Self { is_tuple: false, graph_name: name.clone(), field_name: name.clone(), field_type: name.clone() })
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

        let copy = quote! {
            impl Copy for #name {}

            impl Eq for #name {}
        };
        tokens.extend(copy);
    }
}

impl GraphDerive {
    pub fn build(&self) -> TokenStream2 {
        quote! {
            self
        }
    }
}
