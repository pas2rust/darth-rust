use quote::quote;
use syn::Data;

pub fn generate_default_method(data: &Data) -> proc_macro2::TokenStream {
    match data {
        Data::Struct(data_struct) => {
            let field_initializers = data_struct.fields.iter().map(|field| {
                let field_name = &field.ident;
                quote! {
                    #field_name: Default::default()
                }
            });

            quote! {
                /// Generate a `default` method for the struct.
                ///
                /// This method initializes all of the struct's fields with their default values
                /// and returns a new instance of the struct with those initial values.
                ///
                ///
                /// # Example
                ///
                /// ```rust
                /// #[derive(Default)]
                /// struct MyStruct {
                ///     field1: String,
                ///     field2: i32,
                /// }
                ///
                /// let default_instance = MyStruct::default();
                ///
                /// // Verify that field1 and field2 have their default values.
                /// assert_eq!(default_instance.field1, String::default());
                /// assert_eq!(default_instance.field2, i32::default());
                /// ```
                ///
                /// # Returns
                /// A new instance of the struct with all fields set to their default values.
                pub fn default() -> Self {
                    Self {
                        #(#field_initializers),*
                    }
                }
            }
        }
        _ => quote! {},
    }
}
