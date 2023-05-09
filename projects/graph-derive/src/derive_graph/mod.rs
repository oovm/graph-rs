use quote::{quote, ToTokens};
use syn::{
    __private::TokenStream2,
    parse::{Parse, ParseStream},
    Error, ExprStruct, ItemStruct,
};

pub struct GraphDerive {
    Undefined,
    Easy,
    Normal,
}

pub struct Name {

}

impl Parse for GraphDerive {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let item = match input.parse::<ItemStruct>() {
            Ok(o) => o,
            Err(_) => Err(Error::new(input.span(), "`Graph` only work on struct"))?,
        };
        for i in item.fields {
            for i in i.attrs {
                for i in i.parse_args::<ExprStruct>() {
                    panic!("{:#?}", i.into_token_stream().to_string());
                }
            }
        }
        Ok(GraphDerive::Easy)
    }
}

fn maybe_simple() {}

impl GraphDerive {
    pub fn build(&self) -> TokenStream2 {
        quote! {

        }
    }
}
