use quote::quote;
use syn::{Data, Fields, Ident};

pub fn generate_mut_getters(fields: &Data) -> proc_macro2::TokenStream {
    match fields {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(named_fields) => {
                let methods = named_fields.named.iter().map(|field| {
                    let field_name = &field.ident.as_ref().unwrap();
                    let field_type = &field.ty;
                    let method_name = Ident::new(
                        &format!("get_mut_{}", field_name),
                        field_name.span(),
                    );

                    quote! {
                        pub fn #method_name(&mut self) -> &mut #field_type {
                            &mut self.#field_name
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
