use quote::quote;
use syn::{Data, DeriveInput, Error};

pub fn generate_to_csv_method(input: &DeriveInput) -> proc_macro2::TokenStream {
    let fields = match &input.data {
        Data::Struct(data_struct) => &data_struct.fields,
        _ => {
            return Error::new_spanned(&input, "to_csv can only be derived for structs")
                .to_compile_error()
                .into();
        }
    };

    let field_names = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        quote! {
            stringify!(#field_name)
        }
    });

    let field_values = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        quote! {
            self.#field_name
        }
    });

    quote! {
        /// Generates a `to_csv` method to serialize the structure into CSV.
        ///
        /// This method generates a CSV string that represents the current structure and its fields.
        ///
        /// # Example
        ///
        /// ```rust
        /// use darth_rust::DarthRust;
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
        /// let csv_string = my_instance.to_csv();
        /// let expected_csv = "field1,field2\nHello,42";
        ///
        /// assert_eq!(csv_string, expected_csv);
        /// ```
        ///
        /// # Return
        ///
        /// The method returns a `String` containing the CSV representation of the structure.
        ///
        /// This method serializes the structure and its fields into a CSV string.
        fn to_csv(&self) -> String {
            let field_names = vec![#(#field_names), *].join(",");
            let field_values = vec![#(#field_values.to_string()), *].join(",");
            format!("{},{}", field_names, field_values)
        }
    }
}
