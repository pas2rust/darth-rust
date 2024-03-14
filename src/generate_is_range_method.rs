use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, Fields, Type};

pub fn generate_is_range_method(data: &Data) -> TokenStream {
    let mut methods = TokenStream::new();
    if let Data::Struct(data_struct) = data {
        if let Fields::Named(named_fields) = &data_struct.fields {
            for field in named_fields.named.iter() {
                let field_name = &field.ident;
                let field_type = &field.ty;
                if let Type::Path(type_path) = field_type {
                    let type_ident = type_path
                        .path
                        .segments
                        .last()
                        .unwrap()
                        .ident
                        .to_string();
                    let basic_numeric_types = [
                        "u8", "u16", "u32", "u64", "u128", "usize", "i8",
                        "i16", "i32", "i64", "i128", "isize", "f64", "f32",
                    ];
                    if basic_numeric_types.contains(&type_ident.as_str()) {
                        let method_name_is_range = format_ident!(
                            "is_{}_range",
                            field_name.as_ref().unwrap()
                        );
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
        }
    }
    methods
}
