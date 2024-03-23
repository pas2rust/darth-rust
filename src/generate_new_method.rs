use quote::quote;
use syn::Generics;

use crate::helpers::{Helpers, HelpersTrait};

pub fn generate_new_method(
    helpers: Helpers,
    generics: &Generics,
) -> proc_macro2::TokenStream {
    let fields = helpers.get_fields().unwrap();
    let field_param_initializers = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        quote! {
            #field_name: #field_type
        }
    });

    let field_names = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! {
            #field_name,
        }
    });

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
            pub fn new(#impl_generics #(#field_param_initializers),*) -> Self #ty_generics #where_clause {
                Self {
                    #(#field_names)*
                }
            }
    }
}
