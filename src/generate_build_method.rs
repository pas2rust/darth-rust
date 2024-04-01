use quote::quote;

use crate::helpers::{Helpers, HelpersTrait};

pub fn generate_build_method(helpers: Helpers) -> proc_macro2::TokenStream {
    let iter = helpers.get_named_fields().unwrap().named.iter();
    let methods = iter.clone().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        quote! {
            pub fn #field_name<T: Into<#field_type>>(mut self, new: T) -> Self {
                self.#field_name = new.into();
                self
            }
        }
    });

    let checks = iter.map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let attributes = &field.attrs;
        let attr = Helpers::get_attr(attributes.clone(), "pattern");
        match attr {
            Ok(attr) => quote! {
                let reg = regex::Regex::new(#attr).unwrap();
                if !reg.is_match(&self.#field_name.to_string()) {
                    return Err(
                        format!("Field {}: {} does not match pattern {}",
                            stringify!(#field_name),
                            &self.#field_name,
                            #attr
                        )
                    );
                }
            },
            Err(_) => quote! {},
        }
    });

    let static_methods = quote! {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn build(self) -> Result<Self, String> {
            #(#checks)*
            Ok(self)
        }
    };

    quote! {
        #static_methods
        #(#methods)*
    }
}
