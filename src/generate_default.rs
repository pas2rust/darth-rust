#![cfg(feature = "build")]
use crate::helpers::{Helpers, HelpersTrait};
use quote::quote;

pub fn generate_default(
    helpers: Helpers,
) -> proc_macro2::TokenStream {
    let data_struct = helpers.get_data_struct().unwrap();
    let field_names =
        data_struct.fields.iter().map(|field| {
            let field_name = &field.ident;
            quote! {
                #field_name: Default::default(),
            }
        });

    quote! {
        pub fn default() -> Self {
            Self {
                #(#field_names)*
            }
        }
    }
}
