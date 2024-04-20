use crate::helpers::{Helpers, HelpersTrait};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, LitInt, LitStr};

pub fn max_validator(field: &Field) -> TokenStream {
    let field_name = field.ident.as_ref().unwrap();
    let attributes = &field.attrs;
    let notify = Helpers::get_attr::<LitStr>(
        attributes.clone(),
        "max_notify",
    );
    let max = Helpers::get_attr::<LitInt>(
        attributes.clone(),
        "max",
    );
    if let Ok(attr_max) = max {
        if let Ok(attr_notify) = notify {
            quote! {
                if self.#field_name > #attr_max.into() {
                    return Err(#attr_notify.to_string());
                }
            }
        } else {
            quote! {
                if self.#field_name > #attr_max.into() {
                    return Err(format!("Field {}: {} does not match #[max({})]",
                        stringify!(#field_name),
                        &self.#field_name,
                        #attr_max
                    ));
                }
            }
        }
    } else {
        quote! {}
    }
}
