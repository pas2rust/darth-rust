#![cfg(feature = "get_mut")]
use crate::helpers::{Helpers, HelpersTrait};
use quote::quote;
pub fn generate_mut_getters(
    helpers: Helpers,
) -> proc_macro2::TokenStream {
    let methods =
        helpers.get_named_fields().unwrap().named.iter().map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let field_type = &field.ty;
            let method_name = Helpers::new_ident("get_mut", field_name.clone());

            quote! {
                pub fn #method_name(&mut self) -> &mut #field_type {
                    &mut self.#field_name
                }
            }
        });
    quote! { #(#methods)* }
}
