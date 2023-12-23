use quote::quote;
use syn::{Data, Fields, Ident};

pub fn generate_getters(fields: &Data) -> proc_macro2::TokenStream {
    match fields {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(named_fields) => {
                let methods = named_fields.named.iter().map(|field| {
                    let field_name = &field.ident.as_ref().unwrap();
                    let field_type = &field.ty;
                    let method_name = Ident::new(
                        &format!("get_{}", field_name),
                        field_name.span(),
                    );
                    quote! {
                        /// Get an immutable reference to the `{field_name}` field of the struct.
                        ///
                        /// This method allows you to obtain an immutable reference to the `{field_name}`
                        /// field of the struct, which you can use to access its value.
                        ///
                        /// # Returns
                        ///
                        /// An immutable reference to the `{field_name}` field.
                        pub fn #method_name(&self) -> &#field_type {
                            &self.#field_name
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
