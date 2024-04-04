use quote::quote;

use crate::helpers::{Helpers, HelpersTrait};

pub fn generate_printers_success_by_field(
    helpers: Helpers,
) -> proc_macro2::TokenStream {
    let struct_name = &helpers.input.as_ref().unwrap().ident;
    let print_field_methods =
        helpers.get_named_fields().unwrap().named.iter().map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let method_name =
                Helpers::new_ident("print_success", field_name.clone());
            quote! {
                /// Print the value of the `{field_name}` field with a 'SUCCESS' label and light yellow color-coded output.
                pub fn #method_name(&self, custom: &str) {
                    use colorful::{Color, Colorful};
                    let message = format!(
                        "({}) @SUCCESS '{}' {}.{} = {:#?}",
                        chrono::Local::now(),
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
