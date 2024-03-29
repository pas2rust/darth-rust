use quote::quote;

use crate::helpers::{Helpers, HelpersTrait};

pub fn generate_build_method(helpers: Helpers) -> proc_macro2::TokenStream {
    let methods = helpers.get_named_fields().unwrap()
        .named.iter().map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let field_type = &field.ty;
            quote! {
                pub fn #field_name<T: Into<#field_type>>(mut self, new: T) -> Self {
                    self.#field_name = new.into();
                    self
                }
            }
    });

    let static_methods = quote! {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn build(self) -> Result<Self, String> {
            Ok(self)
        }
    };

    quote! {
        #static_methods
        #(#methods)*
    }
}
