use quote::quote;
use syn::{Data, DeriveInput, Error};

pub fn generate_from_json_method(input: &DeriveInput) -> proc_macro2::TokenStream {
    let struct_name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data_struct) => &data_struct.fields,
        _ => {
            return Error::new_spanned(&input, "from_json can only be derived for structs")
                .to_compile_error()
                .into();
        }
    };

    let from_json_code = {
        let field_deserialization = fields.iter().map(|field| {
            let field_name = &field.ident;
            quote! {
                #field_name: match json_object.get(stringify!(#field_name)) {
                    Some(value) => serde_json::from_value(value.clone())
                        .unwrap_or_default(),
                    None => Default::default(),
                },
            }
        });

        quote! {
            #(#field_deserialization)*
        }
    };

    quote! {
        /// Generates a `from_json` method to deserialize the structure from JSON.
        ///
        /// This method deserializes a JSON object into the current structure based on field names.
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
        ///
        /// let json_value = json!({
        ///     "field1": "Hello",
        ///     "field2": 42
        /// });
        ///
        /// let my_instance = MyStruct::from_json(json_value);
        ///
        /// let expected_instance = MyStruct {
        ///     field1: "Hello".to_string(),
        ///     field2: 42,
        /// };
        ///
        /// assert_eq!(my_instance, expected_instance);
        /// ```
        ///
        /// # Arguments
        ///
        /// - `json_value`: A `serde_json::Value` object representing the JSON data.
        ///
        /// # Return
        ///
        /// The method returns an instance of the structure deserialized from the JSON data.
        fn from_json(json_value: serde_json::Value) -> Result<Self, String> {
            if let serde_json::Value::Object(json_object) = json_value {
                Ok(Self {
                    #from_json_code
                })
            } else {
                Err(format!(
                    "Invalid JSON format for deserialization: JSON must match the structure '{}'.", 
                    stringify!(#struct_name)
                ))
            }
        }
    }
}
