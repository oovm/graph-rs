use super::*;

impl ToTokens for EasyTuple {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = &self.graph_name;
        let ty = &self.field_type;
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
            impl std::hash::Hash for #name {
                fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                    self.0.hash(state);
                }
            }
        };
        let ctor = quote! {
            impl #name {
                /// Create a one-way edge.
                #[inline]
                pub const fn one_way(rank: usize) -> Self {
                    Self(rank as #ty)
                }
                /// Create a two-way edge.
                #[inline]
                pub const fn two_way(rank: usize) -> Self {
                    Self(-(rank as #ty))
                }
            }
        };
        let methods = quote! {
            impl #name {
                /// Get the rank of the edge.
                #[inline]
                pub const fn rank(&self) -> usize {
                    self.0.abs() as usize
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
                    serializer.serialize_i128(self.0 as i128)
                }
            }

            #[cfg(feature = "serde")]
            impl<'de> serde::de::Deserialize<'de> for #name {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::de::Deserializer<'de>,
                {
                    i128::deserialize(deserializer).map(|mask| Self(mask as #ty))
                }
            }
        };
        tokens.extend(copy);
        tokens.extend(easy_display(&self.graph_name));
        if self.config.has_wolfram() {
            tokens.extend(easy_wolfram(name, &name.to_string()));
        }
        tokens.extend(eq);
        tokens.extend(serde);
        if self.config.has_constructor() {
            tokens.extend(ctor);
        };
        tokens.extend(methods);
    }
}
