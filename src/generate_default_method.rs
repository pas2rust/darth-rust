use crate::helpers::{Helpers, HelpersTrait};
use quote::quote;
use syn::Generics;
pub fn generate_default_method(
    helpers: Helpers,
    generics: &Generics,
) -> proc_macro2::TokenStream {
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
