use super::*;

impl ToTokens for EasyTable {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = &self.graph_name;
        let field = &self.field_name;
        let ty = &self.field_type;
        let copy = quote! {
            impl Copy for #name {}
            impl Clone for #name {
                #[inline]
                fn clone(&self) -> Self {
                    Self { #field: self.#field }
                }
            }
        };
        let display = easy_display(&self.graph_name);
        let eq = quote! {
            impl Eq for #name {}
            impl PartialEq for #name {
                fn eq(&self, other: &Self) -> bool {
                    self.#field == other.#field
                }
            }
            impl std::hash::Hash for #name {
                fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                    self.#field.hash(state);
                }
            }
        };
        let ctor = quote! {
            impl #name {
                pub const fn one_way(rank: usize) -> Self {
                    Self { mask: rank as #ty }
                }
                pub const fn two_way(rank: usize) -> Self {
                    Self { mask: -(rank as #ty) }
                }
                pub const fn rank(&self) -> usize {
                    self.mask.abs() as usize
                }
            }
        };
        let serde = quote! {
            #[cfg(feature = "serde")]
            impl serde::ser::Serialize for #name {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::ser::Serializer,
                {
                    serializer.serialize_i128(self.#field as i128)
                }
            }

            #[cfg(feature = "serde")]
            impl<'de> serde::de::Deserialize<'de> for #name {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::de::Deserializer<'de>,
                {
                    i128::deserialize(deserializer).map(|mask| Self { #field: mask as #ty })
                }
            }
        };
        tokens.extend(copy);
        tokens.extend(display);
        tokens.extend(eq);
        tokens.extend(serde);
        tokens.extend(ctor);
    }
}
