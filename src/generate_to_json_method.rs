use quote::quote;
use syn::{Data, DeriveInput, Error};

pub fn generate_to_json_method(input: &DeriveInput) -> proc_macro2::TokenStream {
    let fields = match &input.data {
        Data::Struct(data_struct) => &data_struct.fields,
        _ => {
            return Error::new_spanned(&input, "to_json can only be derived for structs")
                .to_compile_error()
                .into();
        }
    };
    let json_object = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! {
            stringify!(#field_name): self.#field_name,
        }
    });

    quote! {
        /// Generates a `to_json` method to serialize the structure into JSON.
        ///
        /// This method generates a JSON object that represents the current structure and its fields.
        ///
        /// # Example
        ///
        /// ```rust
        /// use darth_rust::DarthRust;
        /// use serde_json::json;
        /// #[derive(DarthRust, Debug, PartialEq)]
        /// struct MyStruct {
        ///     field1: String,
        ///     field2: i32,
        /// }
        /// let expected_json = json!({
        ///     "field1": "Hello",
        ///     "field2": 42
        /// });
        /// 
        /// let my_instance = MyStruct {
        ///     field1: "Hello".to_string(),
        ///     field2: 42,
        /// };
        ///
        /// let json_value = my_instance.to_json();
        /// 
        /// assert_eq!(json_value, expected_json);
        /// ```
        ///
        /// # Return
        ///
        /// The method returns a `serde_json::Value` object that represents the structure serialized in JSON.
        fn to_json(&self) -> serde_json::Value {
            serde_json::json!({
                #(#json_object)*
            })
        }
    }
}