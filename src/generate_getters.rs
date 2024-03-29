use quote::quote;

use crate::helpers::{Helpers, HelpersTrait};

pub fn generate_getters(helpers: Helpers) -> proc_macro2::TokenStream {
    let methods =
        helpers.get_named_fields().unwrap().named.iter().map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let field_type = &field.ty;
            let method_name = Helpers::new_ident("get", field_name.clone());
            quote! {
                /// Get an immutable reference to the `{field_name}` field of the struct.
                ///
                /// This method allows you to obtain an immutable reference to the `{field_name}`
                /// field of the struct, which you can use to access its value.
                ///
                /// # Returns
                ///
                /// An immutable reference to the `{field_name}` field.
                pub fn #method_name(&self) -> &#field_type {
                    &self.#field_name
                }
            }
        });

    quote! { #(#methods)* }
}
