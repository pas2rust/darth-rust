#![cfg(feature = "print_by_field")]
use crate::helpers::{Helpers, HelpersTrait};
use quote::quote;
pub fn generate_printers_by_field(
    helpers: Helpers,
) -> proc_macro2::TokenStream {
    let struct_name =
        &helpers.input.as_ref().unwrap().ident;
    let print_field_methods = helpers
        .get_named_fields()
        .unwrap()
        .named
        .iter()
        .map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let method_name = Helpers::new_ident(
                "print",
                field_name.clone(),
            );
            quote! {
                /// Print the value of the `{field_name}` field with a default label and light yellow color-coded output.
                pub fn #method_name(&self, custom: &str) {
                    let message = format!(
                        "({}) @PRINT '{}' {}.{} = {:#?}",
                        chrono::Local::now(),
                        custom,
                        stringify!(#struct_name),
                        stringify!(#field_name),
                        &self.#field_name,
                    );
                    println!("{}", message);
                }
            }
        });

    quote! {
        #(#print_field_methods)*
    }
}
