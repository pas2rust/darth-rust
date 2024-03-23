use crate::helpers::{Helpers, HelpersTrait};
use quote::quote;

pub fn generate_to_json_method(helpers: Helpers) -> proc_macro2::TokenStream {
    let fields = helpers.get_fields().unwrap();
    let json_object = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! {
            stringify!(#field_name): self.#field_name,
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
