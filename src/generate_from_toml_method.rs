use quote::quote;
use syn::{Data, DeriveInput, Error};

pub fn generate_from_toml_method(input: &DeriveInput) -> proc_macro2::TokenStream {
    let struct_name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data_struct) => &data_struct.fields,
        _ => {
            return Error::new_spanned(&input, "from_toml can only be derived for structs")
                .to_compile_error()
                .into();
        }
    };

    let from_toml_code = {
        let field_deserialization = fields.iter().map(|field| {
            let field_name = &field.ident;
            quote! {
                #field_name: match toml_object.get(stringify!(#field_name)) {
                    Some(value) => toml::de::from_str(&value.to_string())
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
        /// Generates a `from_toml` method to deserialize the structure from TOML.
        ///
        /// This method deserializes a TOML table into the current structure based on field names.
        ///
        /// # Example
        ///
        /// ```rust
        /// use darth_rust::DarthRust;
        /// use toml::Value;
        /// #[derive(DarthRust, Debug, PartialEq)]
        /// struct MyStruct {
        ///     field1: String,
        ///     field2: i32,
        /// }
        ///
        /// let toml_str = r#"
        ///     field1 = "Hello"
        ///     field2 = 42
        /// "#;
        ///
        /// let toml_value = toml::de::from_str(toml_str).unwrap();
        /// let my_instance = MyStruct::from_toml(&toml_value);
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
        /// - `toml_table`: A `toml::Value` object representing the TOML table.
        ///
        /// # Return
        ///
        /// The method returns an instance of the structure deserialized from the TOML table.
        pub fn from_toml(toml_table: &toml::Value) -> Result<Self, String> {
            if let toml::Value::Table(toml_object) = toml_table {
                Ok(Self {
                    #from_toml_code
                })
            } else {
                Err(format!(
                    "Invalid TOML format for deserialization: TOML must match the structure '{}'.", 
                    stringify!(#struct_name)
                ))
            }
        }
    }
}
