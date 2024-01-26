use quote::quote;
use syn::{Data, Generics, Type};

pub fn generate_new_method(
    data: &Data,
    generics: &Generics,
) -> proc_macro2::TokenStream {
    match data {
        Data::Struct(data_struct) => {
            let field_param_initializers =
                data_struct.fields.iter().map(|field| {
                    if let Some(field_name) = &field.ident {
                        let field_type = &field.ty;
                        quote! {
                            #field_name: #field_type
                        }
                    } else {
                        panic!("All fields in the struct must have names.");
                    }
                });

            let field_names = data_struct.fields.iter().map(|field| {
                if let Some(field_name) = &field.ident {
                    quote! {
                        #field_name,
                    }
                } else {
                    panic!("All fields in the struct must have names.");
                }
            });

            let generics = generics.clone();
            let (impl_generics, ty_generics, where_clause) =
                generics.split_for_impl();

            quote! {
                pub fn new #impl_generics (#(#field_param_initializers),*) -> Self #where_clause {
                    Self {
                        #(#field_names)*
                    }
                }
            }
        }
        _ => quote! {},
    }
}
