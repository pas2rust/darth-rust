use quote::quote;
use syn::{Data, DeriveInput, Error};

pub fn generate_to_toml_method(input: &DeriveInput) -> proc_macro2::TokenStream {
    let fields = match &input.data {
        Data::Struct(data_struct) => &data_struct.fields,
        _ => {
            return Error::new_spanned(&input, "to_toml can only be derived for structs")
                .to_compile_error()
                .into();
        }
    };

    let toml_object = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! {
            toml_table.insert(stringify!(#field_name).to_string(), toml::Value::try_from(&self.#field_name).unwrap());
        }
    });

    quote! {
        /// Generates a `to_toml` method to serialize the structure into TOML.
        ///
        /// This method generates a TOML table that represents the current structure and its fields.
        ///
        /// # Example
        ///
        /// ```rust
        /// use darth_rust::DarthRust;
        /// use toml::Value;
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
        /// let toml_value = my_instance.to_toml();
        ///
        /// let mut expected_toml = toml::value::Table::new();
        ///
        /// expected_toml.insert("field1".to_string(), toml::Value::try_from(&my_instance.field1).unwrap());
        /// expected_toml.insert("field2".to_string(), toml::Value::try_from(&my_instance.field2).unwrap());
        ///
        /// assert_eq!(toml_value, toml::Value::Table(expected_toml));
        /// ```
        ///
        /// # Return
        ///
        /// The method returns a `toml::Value` object that represents the structure serialized in TOML.
        fn to_toml(&self) -> toml::Value {
            let mut toml_table = toml::value::Table::new();
            #(#toml_object)*
            toml::Value::Table(toml_table)
        }
    }
}
