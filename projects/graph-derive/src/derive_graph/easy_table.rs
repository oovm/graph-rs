use super::*;

impl ToTokens for EasyTable {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = &self.graph_name;
        let field = &self.field_name;
        let copy = quote! {
            impl Copy for #name {}
            impl Clone for #name {
                #[inline]
                fn clone(&self) -> Self {
                    Self(self.#field)
                }
            }
        };
        let eq = quote! {
            impl Eq for #name {}
            impl PartialEq for EasyGraphTable {
                fn eq(&self, other: &Self) -> bool {
                    self.#field == other.#field
                }
            }
        };
        tokens.extend(copy);
        tokens.extend(eq);
    }
}
