#![cfg(feature = "build")]
use crate::helpers::{Helpers, HelpersTrait};
use quote::quote;
use syn::{LitInt, LitStr};
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

    let check_pattern = iter.clone().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let attributes = &field.attrs;
        let pattern =
            Helpers::get_attr::<LitStr>(attributes.clone(), "pattern");
        match pattern {
            Ok(attr) => quote! {
                let reg = regex::Regex::new(#attr).unwrap();
                if !reg.is_match(&self.#field_name.to_string()) {
                    return Err(
                        format!("Field {}: {} does not match #[pattern({})]",
                            stringify!(#field_name),
                            &self.#field_name,
                            #attr
                        )
                    );
                }
            },
            Err(_) => quote! {},
        }
    });

    let check_min = iter.clone().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let attributes = &field.attrs;
        let min = Helpers::get_attr::<LitInt>(attributes.clone(), "min");
        match min {
            Ok(min) => quote! {
                if self.#field_name < #min.into() {
                    return Err(
                        format!("Field {}: {} does not match #[min({})]",
                            stringify!(#field_name),
                            &self.#field_name,
                            #min
                        )
                    );
                }
            },
            Err(_) => quote! {},
        }
    });

    let check_max = iter.clone().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let attributes = &field.attrs;
        let max = Helpers::get_attr::<LitInt>(attributes.clone(), "max");
        match max {
            Ok(max) => quote! {
                if self.#field_name > #max.into() {
                    return Err(
                        format!("Field {}: {} does not match #[max({})]",
                            stringify!(#field_name),
                            &self.#field_name,
                            #max
                        )
                    );
                }
            },
            Err(_) => quote! {},
        }
    });

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
