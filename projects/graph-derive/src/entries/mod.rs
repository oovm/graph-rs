use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{spanned::Spanned, Attribute, Error, Ident, Meta};

pub enum ParseResult<T> {
    Ok(T),
    NotGood,
    Bad(Error),
}

pub struct GraphAttribute {
    pub head: &'static str,
    pub body: TokenStream,
}

impl GraphAttribute {
    pub fn new(attr: &Attribute) -> Option<GraphAttribute> {
        let (head, rest) = match &attr.meta {
            Meta::Path(p) => (p.to_token_stream().to_string(), TokenStream::new()),
            Meta::List(l) => (l.path.to_token_stream().to_string(), l.tokens.clone()),
            _ => {
                return None;
            }
        };
        let name = match head.as_str() {
            "graph" => "graph",
            "easy_graph" => "easy_graph",
            _ => return None,
        };
        Some(GraphAttribute { head: name, body: rest })
    }
    pub fn is_easy_graph(&self) -> bool {
        self.head == "easy_graph"
    }
}
