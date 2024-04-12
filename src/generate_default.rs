#![cfg(feature = "build")]
use crate::helpers::{Helpers, HelpersTrait};
use quote::quote;

pub fn generate_default(
    helpers: Helpers,
) -> proc_macro2::TokenStream {
    let data_struct = helpers.get_data_struct().unwrap();
    let field_names =
        data_struct.fields.iter().map(|field| {
            let field_name = &field.ident;
            quote! {
                #field_name: Default::default(),
            }
        });

    quote! {
        /// Creates a new instance of the struct with default values for each field.
        ///
        /// This method uses the `Default` trait to initialize each field of the struct. The `Default` trait provides a function `default` that returns the default value of the type.
        ///
        /// # Returns
        ///
        /// * `Self` - A new instance of the struct with default values for each field.
        ///
        /// # Examples
        ///
        /// ```rust
        /// let instance = StructName::default();
        /// ```
        pub fn default() -> Self {
            Self {
                #(#field_names)*
            }
        }
    }
}
