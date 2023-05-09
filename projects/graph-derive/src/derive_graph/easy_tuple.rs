use super::*;

impl ToTokens for EasyTuple {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = &self.graph_name;
        let copy = quote! {
            impl Copy for #name {}
            impl Clone for #name {
                #[inline]
                fn clone(&self) -> Self {
                    Self(self.0)
                }
            }
        };
        let eq = quote! {
            impl Eq for #name {}
            impl PartialEq for #name {
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }
            }
        };
        tokens.extend(copy);
        tokens.extend(eq);
    }
}
