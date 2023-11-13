use quote::quote;
use syn::{Data, DeriveInput, Error};

pub fn generate_from_xml_method(input: &DeriveInput) -> proc_macro2::TokenStream {
    let fields = match &input.data {
        Data::Struct(data_struct) => &data_struct.fields,
        _ => {
            return Error::new_spanned(&input, "from_xml can only be derived for structs")
                .to_compile_error()
                .into();
        }
    };

    let from_xml_code = {
        let field_deserialization = fields.iter().map(|field| {
            let field_name = &field.ident;
            let field_type = &field.ty;

            quote! {
                #field_name: {
                    let field_start_tag = format!("<{}>", stringify!(#field_name));
                    let field_end_tag = format!("</{}>", stringify!(#field_name));
                    let start_pos = xml_string.find(&field_start_tag);
                    if let Some(start_pos) = start_pos {
                        let start_pos = start_pos + field_start_tag.len();
                        if let Some(end_pos) = xml_string[start_pos..].find(&field_end_tag) {
                            let field_value = &xml_string[start_pos..start_pos + end_pos];
                            if let Ok(parsed_value) = field_value.parse::<#field_type>() {
                                parsed_value
                            } else {
                                Default::default()
                            }
                        } else {
                            Default::default()
                        }
                    } else {
                        Default::default()
                    }
                }
            }
        });

        quote! {
            #(#field_deserialization,)*
        }
    };

    quote! {
        /// Generates a `from_xml` method to deserialize the structure from XML.
        ///
        /// This method deserializes an XML string into the current structure based on field names.
        ///
        /// # Example
        ///
        /// ```rust
        /// use darth_rust::DarthRust;
        ///
        /// #[derive(DarthRust, Debug, PartialEq)]
        /// struct User {
        ///     id: String,
        ///     name: String,
        ///     password: String,
        ///     email: String,
        ///     age: u8,
        /// }
        ///
        /// let xml_str = r#"
        ///     <User>
        ///         <id>123</id>
        ///         <name>Alice</name>
        ///         <password>my_password</password>
        ///         <email>alice@example.com</email>
        ///         <age>30</age>
        ///     </User>
        /// "#;
        ///
        /// let user_instance = User::from_xml(xml_str);
        ///
        /// let expected_user = User {
        ///     id: "123".to_string(),
        ///     name: "Alice".to_string(),
        ///     password: "my_password".to_string(),
        ///     email: "alice@example.com".to_string(),
        ///     age: 30,
        /// };
        ///
        /// assert_eq!(user_instance, expected_user);
        /// ```
        ///
        /// # Arguments
        ///
        /// - `xml_string`: A string containing the XML data to be deserialized.
        ///
        /// # Return
        ///
        /// The method returns an instance of the structure deserialized from the provided XML string.
        pub fn from_xml(xml_string: &str) -> Self {
            Self {
                #from_xml_code
            }
        }
    }
}