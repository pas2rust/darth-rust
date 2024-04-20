use crate::helpers::{Helpers, HelpersTrait};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, LitInt, LitStr};

pub fn min_validator(field: &Field) -> TokenStream {
    let field_name = field.ident.as_ref().unwrap();
    let attributes = &field.attrs;
    let notify = Helpers::get_attr::<LitStr>(
        attributes.clone(),
        "min_notify",
    );
    let min = Helpers::get_attr::<LitInt>(
        attributes.clone(),
        "min",
    );
    if let Ok(attr_min) = min {
        if let Ok(attr_notify) = notify {
            quote! {
                if self.#field_name < #attr_min.into() {
                    return Err(#attr_notify.to_string());
                }
            }
        } else {
            quote! {
                if self.#field_name < #attr_min.into() {
                    return Err(format!("Field {}: {} does not match #[min({})]",
                        stringify!(#field_name),
                        &self.#field_name,
                        #attr_min
                    ));
                }
            }
        }
    } else {
        quote! {}
    }
}
