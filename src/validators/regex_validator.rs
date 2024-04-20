use crate::helpers::{Helpers, HelpersTrait};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, LitStr};

pub fn regex_validator(field: &Field) -> TokenStream {
    let field_name = field.ident.as_ref().unwrap();
    let attributes = &field.attrs;
    let notify = Helpers::get_attr::<LitStr>(
        attributes.clone(),
        "pattern_notify",
    );
    let pattern = Helpers::get_attr::<LitStr>(
        attributes.clone(),
        "pattern",
    );
    if let Ok(attr_pattern) = pattern {
        if let Ok(attr_notify) = notify {
            quote! {
                let reg = regex::Regex::new(#attr_pattern).unwrap();
                if !reg.is_match(&self.#field_name.to_string()) {
                    return Err(#attr_notify.to_string());
                }
            }
        } else {
            quote! {
                let reg = regex::Regex::new(#attr_pattern).unwrap();
                if !reg.is_match(&self.#field_name.to_string()) {
                    return Err(format!("Field {}: {} does not match #[pattern({})]",
                        stringify!(#field_name),
                        &self.#field_name,
                        #attr_pattern
                    ));
                }
            }
        }
    } else {
        quote! {}
    }
}
