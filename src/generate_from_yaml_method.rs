use quote::quote;
use syn::{Data, DeriveInput, Error};

pub fn generate_from_yaml_method(input: &DeriveInput) -> proc_macro2::TokenStream {
    let struct_name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data_struct) => &data_struct.fields,
        _ => {
            return Error::new_spanned(&input, "from_yaml can only be derived for structs")
                .to_compile_error()
                .into();
        }
    };

    let from_yaml_code = {
        let field_deserialization = fields.iter().map(|field| {
            let field_name = &field.ident;
            quote! {
                #field_name: match yaml_object.get(stringify!(#field_name)) {
                    Some(value) => serde_yaml::from_value(value.clone())
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
        /// Generates a `from_yaml` method to deserialize the structure from YAML.
        ///
        /// This method deserializes a YAML object into the current structure based on field names.
        ///
        /// # Example
        ///
        /// ```rust
        /// use darth_rust::DarthRust;
        /// use serde_yaml::Value;
        /// #[derive(DarthRust, Debug, PartialEq)]
        /// struct MyStruct {
        ///     field1: String,
        ///     field2: i32,
        /// }
        ///
        /// let yaml_value: Value = serde_yaml::from_str("
        ///     field1: Hello
        ///     field2: 42
        /// ").unwrap();
        ///
        /// let my_instance = MyStruct::from_yaml(yaml_value);
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
        /// - `yaml_value`: A `serde_yaml::Value` object representing the YAML data.
        ///
        /// # Return
        /// The method returns an instance of the structure deserialized from the YAML data.
        fn from_yaml(yaml_value: serde_yaml::Value) -> Result<Self, String> {
            if let serde_yaml::Value::Mapping(yaml_object) = yaml_value {
                Ok(Self {
                    #from_yaml_code
                })
            } else {
                Err(format!(
                    "Invalid YAML format for deserialization: YAML must match the structure '{}'.",
                    stringify!(#struct_name)
                ))
            }
        }
    }
}
