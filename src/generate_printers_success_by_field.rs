use quote::quote;
use syn::Ident;

use crate::helpers::{Helpers, HelpersTrait};

pub fn generate_printers_success_by_field(
    helpers: Helpers,
    struct_name: Ident,
) -> proc_macro2::TokenStream {
    let named_fields = helpers.get_named_fields().unwrap();
    let print_field_methods = named_fields.named.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let method_name =
            Helpers::new_ident("print_success", field_name.clone());
        quote! {
            /// Print the value of the `{field_name}` field with a 'SUCCESS' label and light yellow color-coded output.
            pub fn #method_name(&self, custom: &str) {
                use colorful::{Color, Colorful};
                let message = format!(
                    "({}) @SUCCESS '{}' {}.{} = {:#?}",
                    chrono::Utc::now(),
                    custom,
                    stringify!(#struct_name),
                    stringify!(#field_name),
                    &self.#field_name
                );
                println!("{}", message.gradient(Color::LightGreen));
            }
        }
    });

    quote! {
        #(#print_field_methods)*
    }
}
