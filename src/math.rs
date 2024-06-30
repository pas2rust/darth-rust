#![cfg(feature = "math")]
use crate::helpers::{Helpers, HelpersTrait};
use quote::quote;
pub fn generate_math(
    helpers: Helpers,
) -> proc_macro2::TokenStream {
    let methods = helpers.get_named_fields().unwrap()
    .named.iter().filter_map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        let type_path = Helpers::get_type_path(field_type).unwrap();
        let type_ident = &type_path.path.segments.last().unwrap().ident;
        let method_name_sum = Helpers::new_ident("sum", field_name.clone());
        let method_name_subtract = Helpers::new_ident("subtract", field_name.clone());
        let method_name_multiply = Helpers::new_ident("multiply", field_name.clone());
        let method_name_divide = Helpers::new_ident("divide", field_name.clone());
        let method_name_inflate = Helpers::new_ident("inflate", field_name.clone());
        let method_name_discount = Helpers::new_ident("discount", field_name.clone());
        let basic_numeric_types = ["u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128", "isize"];
        let basic_methods = quote! {
            /// Adds a given value to the field.
            ///
            /// # Arguments
            ///
            /// * `other` - The value to add to the field.
            ///
            /// # Examples
            ///
            /// ```rust
            /// instance.#field_name_sum(10);
            /// ```
            pub fn #method_name_inflate(&mut self, percentage: f64) {
                self.#field_name = (self.#field_name as f64 * (1.0 + percentage / 100.0)) as _;
            }
            /// Discounts a given value from the field.
            ///
            /// # Arguments
            ///
            /// * `other` - The value to discount percentage from the field.
            ///
            /// # Examples
            ///
            /// ```rust
            /// instance.#field_name_discount(5);
            /// ```
            pub fn #method_name_discount(&mut self, percentage: f64) {
                self.#field_name = (self.#field_name as f64 * (1.0 - percentage / 100.0)) as _;
            }
            /// Sums a given value from the field.
            ///
            /// # Arguments
            ///
            /// * `other` - The value to sum from the field.
            ///
            /// # Examples
            ///
            /// ```rust
            /// instance.#field_name_sum(5);
            /// ```
            pub fn #method_name_sum(&mut self, other: #field_type) {
                self.#field_name += other;
            }
            /// Subtracts a given value from the field.
            ///
            /// # Arguments
            ///
            /// * `other` - The value to subtract from the field.
            ///
            /// # Examples
            ///
            /// ```rust
            /// instance.#field_name_subtract(5);
            /// ```
            pub fn #method_name_subtract(&mut self, other: #field_type) {
                self.#field_name -= other;
            }
            /// Multiplies the field by a given value.
            ///
            /// # Arguments
            ///
            /// * `other` - The value to multiply the field by.
            ///
            /// # Examples
            ///
            /// ```rust
            /// instance.#field_name_multiply(2);
            /// ```
            pub fn #method_name_multiply(&mut self, other: #field_type) {
                self.#field_name *= other;
            }
            /// Divides the field by a given value.
            ///
            /// # Arguments
            ///
            /// * `other` - The value to divide the field by.
            ///
            /// # Examples
            ///
            /// ```rust
            /// instance.#field_name_divide(2);
            /// ```
            pub fn #method_name_divide(&mut self, other: #field_type) {
                self.#field_name /= other;
            }
        };

        if type_ident == "f32" || type_ident == "f64" {
            let method_name_sqrt = Helpers::new_ident("sqrt", field_name.clone());
            let method_name_log = Helpers::new_ident("log", field_name.clone());
            let method_name_round = Helpers::new_ident("round", field_name.clone());
            let method_name_abs = Helpers::new_ident("abs", field_name.clone());
            let advanced_methods = quote! {
                /// Computes the square root of the field value.
                ///
                /// # Examples
                ///
                /// ```
                /// struct.#field_name_sqrt();
                /// ```
                pub fn #method_name_sqrt(&mut self) {
                    self.#field_name = self.#field_name.sqrt();
                }
                /// Computes the natural logarithm of the field value.
                ///
                /// # Examples
                ///
                /// ```
                /// struct.#field_name_log();
                /// ```
                pub fn #method_name_log(&mut self) {
                    self.#field_name = self.#field_name.ln();
                }
                /// Rounds the field value to the nearest whole number.
                ///
                /// # Examples
                ///
                /// ```
                /// struct.#field_name_round();
                /// ```
                pub fn #method_name_round(&mut self) {
                    self.#field_name = self.#field_name.round();
                }
                /// Computes the absolute value of the field value.
                ///
                /// # Examples
                ///
                /// ```
                /// struct.#field_name_abs();
                /// ```
                pub fn #method_name_abs(&mut self) {
                    self.#field_name = self.#field_name.abs();
                }
            };

            Some(quote! {
                #basic_methods
                #advanced_methods
            })
        } else if basic_numeric_types.iter().any(|&n| *type_ident == n) {
            Some(basic_methods)
        } else {
            None
        }
    });
    quote! { #(#methods)* }
}
