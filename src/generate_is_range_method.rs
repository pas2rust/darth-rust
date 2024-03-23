use proc_macro2::TokenStream;
use quote::quote;
use syn::Type;

use crate::helpers::{Helpers, HelpersTrait};

pub fn generate_is_range_method(helpers: Helpers) -> TokenStream {
    let named_fields = helpers.get_named_fields().unwrap();
    let mut methods = TokenStream::new();
    for field in named_fields.named.iter() {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        if let Type::Path(type_path) = field_type {
            let type_ident =
                type_path.path.segments.last().unwrap().ident.to_string();
            let basic_numeric_types = [
                "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32",
                "i64", "i128", "isize", "f64", "f32",
            ];
            if basic_numeric_types.contains(&type_ident.as_str()) {
                let method_name_is_range =
                    Helpers::new_ident("is_range", field_name.clone());
                methods.extend(quote! {
                    pub fn #method_name_is_range(&self, min: #field_type, max: #field_type) -> Result<(), String> {
                        let value = self.#field_name;
                        if value >= min && value <= max {
                            Ok(())
                        } else {
                            Err(format!("The field '{}: {}' is not within the range {} to {}", stringify!(#field_name), value, min, max))
                        }
                    }
                });
            }
        }
    }
    methods
}
