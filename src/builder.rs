#![cfg(feature = "build")]
use crate::{
    helpers::{Helpers, HelpersTrait},
    validators::{
        max_validator::max_validator,
        min_validator::min_validator,
        regex_validator::regex_validator,
    },
};
use quote::quote;

pub fn generate_build(
    helpers: Helpers,
) -> proc_macro2::TokenStream {
    let iter =
        helpers.get_named_fields().unwrap().named.iter();
    let methods = iter.clone().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        quote! {
            /// Sets the value of `#field_name` for the struct and returns the modified struct.
            ///
            /// This method consumes the struct, modifies it, and then returns it for further chaining of method calls.
            ///
            /// The `new` parameter is the new value for `#field_name`. It is generic and can be converted into the type of `#field_name`.
            ///
            /// # Parameters
            ///
            /// * `new: Darth` - The new value for `#field_name`. `Darth` is any type that can be converted into the type of `#field_name`.
            ///
            /// # Returns
            ///
            /// * `Self` - The modified struct.
            ///
            /// # Examples
            ///
            /// ```rust
            /// let instance = StructName::new()
            ///     .#field_name(value)
            ///     .build();
            /// ```
            pub fn #field_name<Darth: Into<#field_type>>(mut self, new: Darth) -> Self {
                self.#field_name = new.into();
                self
            }
        }
    });
    let check_pattern = iter.clone().map(regex_validator);
    let check_min = iter.clone().map(min_validator);
    let check_max = iter.clone().map(max_validator);
    let static_methods = quote! {
        /// Creates a new instance of the struct with default values.
        ///
        /// # Examples
        ///
        /// ```rust
        /// let instance = StructName::new();
        /// ```
        pub fn new() -> Self {
            Self::default()
        }
        /// Validates the struct fields based on the specified attributes and returns the struct instance if all validations pass.
        /// If any validation fails, it returns an error message.
        ///
        /// The validation checks include:
        /// - Checking if the field value matches the specified pattern (if any).
        /// - Checking if the field value is greater than or equal to the specified minimum value (if any).
        /// - Checking if the field value is less than or equal to the specified maximum value (if any).
        ///
        /// # Returns
        ///
        /// * `Result<Self, String>` - A result that is `Ok` if the JSON value could be deserialized into the struct, or `Err` if the JSON value is not an object or does not match the structure of the struct.
        ///
        /// # Examples
        ///
        /// ```rust
        ///  let obj = StructName::new().id(1).build();
        ///  match obj {
        ///      OK(ok) => println!("{:#?}", ok),
        ///      Err(err) => println!("obj err: {}", err)
        ///  };
        /// ```
        pub fn build(self) -> Result<Self, String> {
            #(#check_pattern)*
            #(#check_max)*
            #(#check_min)*
            Ok(self)
        }
    };

    quote! {
        #static_methods
        #(#methods)*
    }
}
