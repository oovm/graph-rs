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
    pub fn has_wolfram(&self) -> bool {
        let str = self.body.to_string();
        if str.contains("wolfram") {
            return true;
        }
        false
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

pub fn easy_wolfram(rust: &Ident, wolfram_str: &str) -> TokenStream2 {
    let name_str = LitStr::new(&rust.to_string(), rust.span());
    let doc = format!(
        "Convert rust type [{}] to wolfram type [{wolfram_str}](https://reference.wolfram.com/language/ref/{wolfram_str}.html)",
        name_str.value()
    );
    quote! {
        #[cfg(feature = "wolfram")]
        impl graph_types::ToWolfram for #rust {
            #[doc = #doc]
            fn to_wolfram(&self) -> graph_types::WolframValue {
                let n = graph_types::WolframValue::Integer64(self.rank() as i64);
                let args = match self.graph_kind() {
                    GraphKind::Directed => {
                        let arg1 = graph_types::WolframValue::pair("DirectedEdges", true, false);
                        vec![n, arg1]
                    }
                    GraphKind::Undirected => vec![n],
                };
                graph_types::WolframValue::function(#wolfram_str, args)
            }
        }
    }
}
