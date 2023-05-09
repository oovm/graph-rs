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

impl EasyGraph {
    fn new(item: &ItemStruct) -> ParseResult<Self> {
        let name = item.ident.clone();

        let mut fields = item.fields.iter();
        let field = match fields.next() {
            Some(field) => field,
            None => return ParseResult::Bad(Error::new(item.span(), "Graph must have at least one field")),
        };

        let mut attrs = vec![];
        for i in &field.attrs {
            match GraphAttribute::new(i) {
                Some(s) => attrs.push(s),
                None => {}
            }
        }

        let g = match attrs {
            [g] if g.is => g,
            _ => return ParseResult::NotGood,
        };

        let field_name = field.ident.clone();
        let str_type = field.ty.to_token_stream().to_string();
        let field_type = match str_type.trim() {
            "i8" => Ident::new("i8", field.ty.span()),
            "i16" => Ident::new("i16", field.ty.span()),
            "i32" => Ident::new("i32", field.ty.span()),
            "i64" => Ident::new("i64", field.ty.span()),
            "i128" => Ident::new("i128", field.ty.span()),
            "isize" => Ident::new("isize", field.ty.span()),
            _ => return ParseResult::Bad(Error::new(field.ty.span(), "Graph field must be a signed integer")),
        };

        ParseResult::Ok(Self { graph_name: name.clone(), field_name, field_type })
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
