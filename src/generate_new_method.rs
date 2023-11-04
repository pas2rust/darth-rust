use quote::quote;
use syn::Data;

pub fn generate_new_method(data: &Data) -> proc_macro2::TokenStream {
    match data {
        Data::Struct(data_struct) => {
            let field_param_initializers = data_struct.fields.iter().map(|field| {
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
                /// # Example
                ///
                /// ```rust
                /// use darth_rust::DarthRust;
                ///
                /// // Define a struct named MyStruct with two fields: field1 (a String) and field2 (an i32).
                /// #[derive(DarthRust, Debug, PartialEq)]
                /// struct MyStruct {
                ///     field1: String,
                ///     field2: i32,
                /// }
                ///
                /// // Create a new instance of MyStruct with field1 set to "Hello" and field2 set to 42 using the generated `new` method.
                /// let new_instance = MyStruct::new("Hello", 42);
                ///
                /// // Create another instance of MyStruct with field1 and field2 set to specific values.
                /// let instance = MyStruct {
                ///     field1: "Hello".to_string(),
                ///     field2: 42,
                /// };
                ///
                /// // Assert that the two instances are equal, demonstrating the use of the PartialEq trait.
                /// assert_eq!(instance, new_instance);
                /// ```
                ///
                /// # Returns
                ///
                /// A new instance of the `MyStruct` struct with `field1` set to the provided String and `field2` set to the provided i32.
                pub fn new(#(#field_param_initializers),*) -> Self {
                    Self {
                        #(#field_names)*
                    }
                }
            }
        }
        _ => quote! {},
    }
}