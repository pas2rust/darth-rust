use quote::quote;
use syn::{Data, Generics};

pub fn generate_new_method(
    data: &Data,
    generics: &Generics,
) -> proc_macro2::TokenStream {
    match data {
        Data::Struct(data_struct) => {
            let field_param_initializers =
                data_struct.fields.iter().map(|field| {
                    let field_name = &field.ident;
                    let field_type = &field.ty;
                    quote! {
                        #field_name: #field_type
                    }
                });

            let field_names = data_struct.fields.iter().map(|field| {
                let field_name = &field.ident;
                quote! {
                    #field_name,
                }
            });

            let (_, ty_generics, where_clause) = generics.split_for_impl();

            quote! {
                pub fn new(#(#field_param_initializers),*) -> Self #ty_generics #where_clause {
                    Self {
                        #(#field_names)*
                    }
                }
            }
        }
        _ => quote! {},
    }
}
