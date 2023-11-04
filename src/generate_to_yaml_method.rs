use quote::quote;
use syn::{Data, DeriveInput};

pub fn generate_to_yaml_method(input: &DeriveInput) -> proc_macro2::TokenStream {
    let fields = match &input.data {
        Data::Struct(data_struct) => &data_struct.fields,
        _ => {
            return quote! {
                compile_error!("to_yaml can only be derived for structs");
            };
        }
    };

    let field_mappings = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        quote! {
            mapping.insert(serde_yaml::Value::String(stringify!(#field_name).to_owned()), serde_yaml::to_value(&self.#field_name).unwrap_or(serde_yaml::Value::String("Error".to_string())));
        }
    });

    quote! {
        /// Generates a `to_yaml` method to serialize the structure into YAML.
        ///
        /// This method generates a YAML representation of the current structure and its fields.
        ///
        /// # Example
        ///
        /// ```rust
        /// use darth_rust::DarthRust;
        /// use serde_yaml::to_value;
        ///
        /// #[derive(DarthRust, Debug, PartialEq)]
        /// struct MyStruct {
        ///     field1: String,
        ///     field2: i32,
        /// }
        ///
        /// let my_instance = MyStruct {
        ///     field1: "Hello".to_string(),
        ///     field2: 42,
        /// };
        ///
        /// let yaml_value = my_instance.to_yaml();
        ///
        /// // Handle the `serde_yaml::Error` here if needed
        ///
        /// // Your code to handle the `serde_yaml::Value` here
        ///
        /// // assert_eq!(yaml_string, expected_yaml);
        /// ```
        ///
        /// # Return
        ///
        /// The method returns a `serde_yaml::Value` object that represents the structure serialized in YAML.
        fn to_yaml(&self) -> serde_yaml::Value {
            let mut mapping = serde_yaml::Mapping::new();
            #(#field_mappings)*
            serde_yaml::Value::Mapping(mapping)
        }
    }
}
