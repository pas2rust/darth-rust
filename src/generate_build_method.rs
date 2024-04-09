use crate::helpers::{Helpers, HelpersTrait};
use quote::quote;
use syn::{LitInt, LitStr};

#[cfg(feature = "build")]
pub fn generate_build_method(
    helpers: Helpers,
) -> proc_macro2::TokenStream {
    let iter =
        helpers.get_named_fields().unwrap().named.iter();
    let methods = iter.clone().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        quote! {
            pub fn #field_name<Darth: Into<#field_type>>(mut self, new: Darth) -> Self {
                self.#field_name = new.into();
                self
            }
        }
    });

    let check_pattern = iter.clone().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let attributes = &field.attrs;
        let pattern =
            Helpers::get_attr::<LitStr>(attributes.clone(), "pattern");
        match pattern {
            Ok(attr) => quote! {
                let reg = regex::Regex::new(#attr).unwrap();
                if !reg.is_match(&self.#field_name.to_string()) {
                    return Err(
                        format!("Field {}: {} does not match #[pattern({})]",
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

    let check_min = iter.clone().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let attributes = &field.attrs;
        let min = Helpers::get_attr::<LitInt>(attributes.clone(), "min");
        match min {
            Ok(min) => quote! {
                if self.#field_name < #min.into() {
                    return Err(
                        format!("Field {}: {} does not match #[min({})]",
                            stringify!(#field_name),
                            &self.#field_name,
                            #min
                        )
                    );
                }
            },
            Err(_) => quote! {},
        }
    });

    let check_max = iter.clone().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let attributes = &field.attrs;
        let max = Helpers::get_attr::<LitInt>(attributes.clone(), "max");
        match max {
            Ok(max) => quote! {
                if self.#field_name > #max.into() {
                    return Err(
                        format!("Field {}: {} does not match #[max({})]",
                            stringify!(#field_name),
                            &self.#field_name,
                            #max
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
            #(#check_pattern)*
            #(#check_max)*
            #(#check_min)*
            Ok(self)
        }
    };

    quote! {
        #static_methods
        #(#methods)*
    }
}
