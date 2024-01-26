use quote::quote;
use syn::Data;

pub fn generate_new_method(data: &Data) -> proc_macro2::TokenStream {
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

            quote! {
                pub fn new<'a>(#(#field_param_initializers),*) -> Self {
                    Self {
                        #(#field_names)*
                    }
                }
            }
        }
        _ => quote! {},
    }
}
