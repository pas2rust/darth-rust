use quote::quote;
use syn::{Data, Fields, Ident};

pub fn generate_setters(fields: &Data) -> proc_macro2::TokenStream {
    match fields {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(named_fields) => {
                let methods = named_fields.named.iter().map(|field| {
                    let field_name = &field.ident.as_ref().unwrap();
                    let field_type = &field.ty;
                    let method_name = Ident::new(&format!("set_{}", field_name), field_name.span());
                    quote! {
                        /// Set the value of the `{field_name}` field of the struct.
                        ///
                        /// This method allows you to update the value of the `{field_name}` field of the struct.
                        ///
                        /// # Arguments
                        ///
                        /// - `new`: A value of type that can be converted into `{field_name}`'s type.
                        pub fn #method_name<T: Into<#field_type>>(&mut self, new: T) {
                            self.#field_name = new.into();
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
