#![cfg(feature = "json")]
use crate::helpers::{Helpers, HelpersTrait};
use quote::quote;

pub fn generate_from_json(
    helpers: Helpers,
) -> proc_macro2::TokenStream {
    let struct_name =
        &helpers.input.as_ref().unwrap().ident;
    let from_json_code = {
        let field_deserialization = helpers.get_fields().unwrap()
        .iter().map(|field| {
            let field_name = &field.ident;
            let _field_type = &field.ty;
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
        /// Creates a new instance of the struct from a JSON value.
        ///
        /// This method uses the `serde_json` crate to deserialize the JSON value into the struct. Each field in the struct is matched with the corresponding key in the JSON object. If the key is found, the value is deserialized and assigned to the field. If the key is not found or the value cannot be deserialized, the field is assigned its default value.
        ///
        /// # Arguments
        ///
        /// * `json_value` - A JSON value to deserialize into the struct.
        ///
        /// # Returns
        ///
        /// * `Result<Self, String>` - A result that is `Ok` if the JSON value could be deserialized into the struct, or `Err` if the JSON value is not an object or does not match the structure of the struct.
        ///
        /// # Examples
        ///
        /// ```rust
        /// let json_value = serde_json::json!({
        ///     "field1": "value1",
        ///     "field2": "value2"
        /// });
        /// let instance = StructName::from_json(json_value).unwrap();
        /// ```
        pub fn from_json(json_value: serde_json::Value) -> Result<Self, String> {
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
