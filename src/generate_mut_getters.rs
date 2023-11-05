use quote::quote;
use syn::{Data, Fields, Ident};

pub fn generate_mut_getters(fields: &Data) -> proc_macro2::TokenStream {
    match fields {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(named_fields) => {
                let methods = named_fields.named.iter().map(|field| {
                    let field_name = &field.ident.as_ref().unwrap();
                    let field_type = &field.ty;
                    let method_name =
                        Ident::new(&format!("get_mut_{}", field_name), field_name.span());

                    quote! {
                        /// Get a mutable reference to the `{field_name}` field of the struct.
                        ///
                        /// This method allows you to obtain a mutable reference to the `{field_name}`
                        /// field of the struct, which you can use to modify its value.
                        ///
                        /// # Example
                        ///
                        ///```rust
                        /// use darth_rust::DarthRust;
                        ///
                        /// #[derive(DarthRust, Debug, PartialEq)]
                        /// struct MyStruct {
                        ///     field1: String,
                        ///     field2: i32,
                        /// }
                        ///
                        /// let new_instance = MyStruct::new("Hello", 18);
                        /// *new_instance.get_mut_field1() = "mut hello".to_string()
                        ///
                        /// assert_eq!(new_instance.field1, "mut hello");
                        /// ```
                        /// # Returns
                        ///
                        /// A mutable reference to the `{field_name}` field.
                        pub fn #method_name(&mut self) -> &mut #field_type {
                            &mut self.#field_name
                        }
                    }
                });

                quote! { #(#methods)* }
            }
            _ => {
                quote! {}
            }
        },
        _ => {
            quote! {}
        }
    }
}
