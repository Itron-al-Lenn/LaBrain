use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ID)]
pub fn derive_from_u64(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    quote! {
        impl ID for #name {
            fn id(&self) -> i64 {
                self.0
            }
        }

        impl From<i64> for #name {
            fn from(value: i64) -> Self {
                Self(value)
            }
        }

        impl Clone for #name {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl Copy for #name {}

        impl PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }
        }

        impl Eq for #name {}

        impl PartialOrd for #name {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for #name {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.0.cmp(&other.0)
            }
        }

        impl core::fmt::Debug for #name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_tuple(stringify!(#name))
                    .field(&self.0)
                    .finish()
            }
        }
    }
    .into()
}
