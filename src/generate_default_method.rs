use crate::helpers::{Helpers, HelpersTrait};
use quote::quote;

pub fn generate_default_method(helpers: Helpers) -> proc_macro2::TokenStream {
    let generics = &helpers.input.as_ref().unwrap().generics;
    let data_struct = helpers.get_data_struct().unwrap();
    let field_names = data_struct.fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! {
            #field_name: Default::default(),
        }
    });

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        pub fn default(#impl_generics) -> Self #ty_generics #where_clause {
            Self {
                #(#field_names)*
            }
        }
    }
}
