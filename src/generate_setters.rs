use quote::quote;

use crate::helpers::{Helpers, HelpersTrait};

#[cfg(feature = "set")]
pub fn generate_setters(
    helpers: Helpers,
) -> proc_macro2::TokenStream {
    let methods =
        helpers.get_named_fields().unwrap().named.iter().map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let field_type = &field.ty;
            let method_name = Helpers::new_ident("set", field_name.clone());
            quote! {
                /// Set the value of the `{field_name}` field of the struct.
                ///
                /// This method allows you to update the value of the `{field_name}` field of the struct.
                ///
                /// # Arguments
                ///
                /// - `new`: A value of type that can be converted into `{field_name}`'s type.
                pub fn #method_name<Darth: Into<#field_type>>(&mut self, new: Darth) {
                    self.#field_name = new.into();
                }
            }
        });

    quote! { #(#methods)* }
}
