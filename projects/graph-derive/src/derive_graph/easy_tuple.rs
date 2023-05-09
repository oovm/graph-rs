use super::*;

impl ToTokens for EasyTuple {
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
