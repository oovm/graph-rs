use quote::{quote, ToTokens};
use syn::{spanned::Spanned, Attribute, Error, Ident, LitStr, Meta, __private::TokenStream2};

pub enum ParseResult<T> {
    Ok(T),
    NotGood,
    Bad(Error),
}

pub const SUPPORTED_TAGS: [&str; 2] = ["graph", "easy_graph"];

#[derive(Clone)]
pub struct GraphAttribute {
    pub head: Ident,
    pub body: TokenStream2,
}

impl GraphAttribute {
    pub fn new(attr: &Attribute) -> Option<GraphAttribute> {
        let (head, rest) = match &attr.meta {
            Meta::Path(p) => (p.clone(), TokenStream2::new()),
            Meta::List(l) => (l.path.clone(), l.tokens.clone()),
            _ => return None,
        };
        let head_str = head.to_token_stream().to_string();
        for i in &SUPPORTED_TAGS {
            if i.eq(&head_str) {
                let head = Ident::new(i, attr.span());
                return Some(GraphAttribute { head, body: rest });
            }
        }
        None
    }
    pub fn is_easy_graph(&self) -> bool {
        self.head.eq("easy_graph")
    }
    pub fn has_constructor(&self) -> bool {
        // HACK: parse as map
        let str = self.body.to_string();
        if str.contains("constructor = false") {
            return false;
        }
        true
    }
}

pub fn easy_display(id: &Ident) -> TokenStream2 {
    let name_str = LitStr::new(&id.to_string(), id.span());
    quote! {
        impl core::fmt::Debug for #id {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct(#name_str)
                    .field("kind", &self.graph_kind())
                    .field("rank", &self.rank())
                    .field("nodes", &self.count_nodes())
                    .field("edges", &self.count_edges())
                    .finish()
            }
        }

        impl core::fmt::Display for #id {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_tuple(#name_str)
                    .field(&self.rank())
                    .field(&self.graph_kind())
                    .finish()
            }
        }
    }
}
