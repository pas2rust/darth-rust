use quote::quote;
use syn::{Data, Fields, Ident, Type};

pub fn generate_calc_methods(fields: &Data) -> proc_macro2::TokenStream {
    match fields {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(named_fields) => {
                let methods = named_fields.named.iter().filter_map(|field| {
                    let field_name = &field.ident.as_ref().unwrap();
                    let field_type = &field.ty;
                    match field_type {
                        Type::Path(type_path) => {
                            let type_ident = &type_path.path.segments.last().unwrap().ident;
                            let ident = |prefix: &str| Ident::new(format!("{}_{}", prefix, field_name).as_str(), field_name.span());
                            let method_name_sum = ident("sum");
                            let method_name_subtract = ident("subtract");
                            let method_name_multiply = ident("multiply");
                            let method_name_divide = ident("divide");
                            let method_name_inflate = ident("inflate");
                            let method_name_discount = ident("discount");
                            let basic_numeric_types = ["u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128", "isize"];
                            let basic_methods = quote! {
                                pub fn #method_name_inflate(&mut self, percentage: f64) {
                                    self.#field_name = (self.#field_name as f64 * (1.0 + percentage / 100.0)) as _;
                                }
                                pub fn #method_name_discount(&mut self, percentage: f64) {
                                    self.#field_name = (self.#field_name as f64 * (1.0 - percentage / 100.0)) as _;
                                }
                                pub fn #method_name_sum(&mut self, other: #field_type) {
                                    self.#field_name += other;
                                }

                                pub fn #method_name_subtract(&mut self, other: #field_type) {
                                    self.#field_name -= other;
                                }

                                pub fn #method_name_multiply(&mut self, other: #field_type) {
                                    self.#field_name *= other;
                                }

                                pub fn #method_name_divide(&mut self, other: #field_type) {
                                    self.#field_name /= other;
                                }
                            };

                            if type_ident == "f32" || type_ident == "f64" {
                                let method_name_sqrt = ident("sqrt");
                                let method_name_log = ident("log");
                                let method_name_round = ident("round");
                                let method_name_abs = ident("abs");
                                let advanced_methods = quote! {
                                    pub fn #method_name_sqrt(&mut self) {
                                        self.#field_name = self.#field_name.sqrt();
                                    }

                                    pub fn #method_name_log(&mut self) {
                                        self.#field_name = self.#field_name.ln();
                                    }

                                    pub fn #method_name_round(&mut self) {
                                        self.#field_name = self.#field_name.round();
                                    }

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
                        }
                        _ => None,
                    }
                });

                quote! { #(#methods)* }
            }
            _ => {
                quote! {}
            }
        },
        _ => {
            quote! {}
        }
    }
}
