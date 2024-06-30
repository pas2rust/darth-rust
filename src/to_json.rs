#![cfg(feature = "json")]
use crate::helpers::{Helpers, HelpersTrait};
use quote::quote;

pub fn generate_to_json(
    helpers: Helpers,
) -> proc_macro2::TokenStream {
    let json_object =
        helpers.get_fields().unwrap().iter().map(|field| {
            let field_name = &field.ident;
            let field_type = &field.ty;
            if Helpers::is_function(field_type) {
                quote! {
                    stringify!(#field_name): "function",
                }
            } else {
                quote! {
                    stringify!(#field_name): self.#field_name,
                }
            }
        });

    quote! {
        pub fn to_json(&self) -> serde_json::Value {
            serde_json::json!({
                #(#json_object)*
            })
        }
    }
}
