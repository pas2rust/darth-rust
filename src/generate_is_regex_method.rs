use proc_macro2::TokenStream;
use quote::quote;
use syn::Type;

use crate::helpers::{Helpers, HelpersTrait};

pub fn generate_is_regex_method(helpers: Helpers) -> TokenStream {
    let named_fields = helpers.get_named_fields().unwrap();
    let mut methods = TokenStream::new();
    for field in named_fields.named.iter() {
        let field_name = &field.ident;
        let field_type = &field.ty;
        if let Type::Path(type_path) = field_type {
            let type_ident = &type_path.path.segments.last().unwrap().ident;
            if type_ident == "String" || type_ident == "str" {
                let method_name_is_regex = Helpers::new_ident(
                    "is_match_regex",
                    field_name.as_ref().unwrap().clone(),
                );
                methods.extend(quote! {
                    pub fn #method_name_is_regex(&self, regex: &str) -> Result<(), String> {
                        let value = &self.#field_name;
                        let re = regex::Regex::new(regex).unwrap();
                        if re.is_match(value) {
                            Ok(())
                        } else {
                            Err(format!("The field '{}: {}' does not match the regex '{}'", stringify!(#field_name), value, regex))
                        }
                    }
                });
            }
        }
    }
    methods
}
