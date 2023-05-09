use super::*;
use syn::LitStr;

impl ToTokens for EasyTable {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = &self.graph_name;
        let name_str = LitStr::new(&name.to_string(), name.span());
        let field = &self.field_name;
        let copy = quote! {
            impl Copy for #name {}
            impl Clone for #name {
                #[inline]
                fn clone(&self) -> Self {
                    Self { #field: self.#field }
                }
            }
        };
        let eq = quote! {
            impl Eq for #name {}
            impl PartialEq for #name {
                fn eq(&self, other: &Self) -> bool {
                    self.#field == other.#field
                }
            }
        };

        tokens.extend(copy);
        tokens.extend(display);
        tokens.extend(eq);
    }
}

pub fn easy_display(id: &Ident) -> TokenStream2 {
    let display = quote! {
        impl Debug for CycleGraph {
            #[inline]
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(#name_str)
                    .field("kind", &self.graph_kind())
                    .field("rank", &self.rank())
                    .field("nodes", &self.count_nodes())
                    .field("edges", &self.count_edges())
                    .finish()
            }
        }

        impl Display for CycleGraph {
            #[inline]
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple(#name_str)
                    .field(&self.rank())
                    .field(&self.graph_kind())
                    .finish()
            }
        }
    };
}
