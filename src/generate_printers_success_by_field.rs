use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident};

pub fn generate_printers_success_by_field(input: &DeriveInput) -> proc_macro2::TokenStream {
    let struct_name = &input.ident;
    match &input.data {
        Data::Struct(data_struct) => {
            if let Fields::Named(named_fields) = &data_struct.fields {
                let print_field_methods = named_fields.named.iter().map(|field| {
                    let field_name = &field.ident.as_ref().unwrap();
                    let method_name =
                        Ident::new(&format!("print_success_{}", field_name), field_name.span());
                    quote! {
                        /// Print the value of the `{field_name}` field with a 'SUCCESS' label and light yellow color-coded output.
                        fn #method_name(&self) {
                            use colorful::{Color, Colorful};
                            let message = format!(
                                "({}) @SUCCESS ➝ {}.{} = {:#?}",
                                chrono::Utc::now(),
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
            } else {
                // Handle tuple structs or other cases.
                // You can add custom logic here if needed.
                quote! {}
            }
        }
        _ => {
            return syn::Error::new_spanned(&input, "print can only be derived for structs")
                .to_compile_error()
                .into();
        }
    }
}
