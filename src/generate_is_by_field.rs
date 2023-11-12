use quote::quote;
use syn::{Data, Fields, Path, Ident};

fn get_type_path(ty: &syn::Type) -> Option<&Path> {
    match ty {
        syn::Type::Path(type_path) => Some(&type_path.path),
        _ => None,
    }
}

pub fn generate_is_by_field(fields: &Data) -> proc_macro2::TokenStream {
    match fields {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(named_fields) => {
                let methods = named_fields.named.iter().map(|field| {
                    let field_name = &field.ident.as_ref().unwrap();
                    let field_type = &field.ty;
                    let method_name = Ident::new(&format!("is_range_{}", field_name), field_name.span());
                    quote! {
                        pub fn #method_name<T>(&self, min: T, max: T) -> Result<#field_type, String>
                        where
                            T: Into<usize> + Clone,
                        {
                            let field_value = &self.#field_name;
                            if let Some(type_path) = get_type_path(field_type) {
                                if type_path.is_ident("String") {
                                    let chars = field_value.to_string().chars().count();
                                    if chars >= min.into() && chars <= max.into() {
                                        Ok(field_value.clone())
                                    } else {
                                        Err(format!("Value of {} is out of range ({}, {})", stringify!(#field_name), min.into(), max.into()))
                                    }
                                } else if type_path.is_ident("usize") {
                                    let min = min.into();
                                    let max = max.into();
                                    let field_value = *field_value as usize;
                                    if field_value >= min && field_value <= max {
                                        Ok(field_value)
                                    } else {
                                        Err(format!("Value of {} is out of range ({}, {})", stringify!(#field_name), min, max))
                                    }
                                } else {
                                    // Handle other types as needed
                                    Err(format!("Unsupported type for field {}", stringify!(#field_name)))
                                }
                            } else {
                                Err(format!("Type information not available for field {}", stringify!(#field_name)))
                            }
                        }
                    }
                });

                quote! { #(#methods)* }
            }
            _ => {
                quote! {}
            }
        },
        _ => {
            quote! {}
        }
    }
}
