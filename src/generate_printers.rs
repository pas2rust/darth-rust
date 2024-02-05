use quote::quote;
use syn::{Data, DeriveInput};

/// printter
pub fn generate_printers(input: &DeriveInput) -> proc_macro2::TokenStream {
    match &input.data {
        Data::Struct(data_struct) => &data_struct.fields,
        _ => {
            return syn::Error::new_spanned(
                input,
                "print can only be derived for structs",
            )
            .to_compile_error()
        }
    };
    quote! {
        /// Print the struct with various labels and color-coded output.
        pub fn print_all_levels(&self, custom: &str) {
            self.print(custom);
            self.print_rust(custom);
            self.print_info(custom);
            self.print_success(custom);
            self.print_warning(custom);
            self.print_err(custom);
        }

        /// Print the struct with a default label and color-coded output.
        pub fn print(&self, custom: &str) {
            let message = format!("({}) @PRINT '{}' = {:#?}", chrono::Utc::now(), custom, &self);
            println!("{}", message);
        }

        /// Print the struct with a 'RUST' label and dark red color-coded output.
        pub fn print_rust(&self, custom: &str) {
            use colorful::{Color, Colorful};
            let message = format!("({}) @RUST '{}' = {:#?}", chrono::Utc::now(), custom, &self);
            println!("{}", message.gradient(Color::DarkRed2));
        }

        /// Print the struct with an 'INFO' label and purple color-coded output.
        pub fn print_info(&self, custom: &str) {
            use colorful::{Color, Colorful};
            let message = format!("({}) @INFO '{}' = {:#?}", chrono::Utc::now(), custom, &self);
            println!("{}", message.gradient(Color::Purple3));
        }

        /// Print the struct with a 'SUCCESS' label and light green color-coded output.
        pub fn print_success(&self, custom: &str) {
            use colorful::{Color, Colorful};
            let message = format!("({}) @SUCCESS '{}' = {:#?}", chrono::Utc::now(), custom, &self);
            println!("{}", message.gradient(Color::LightGreen));
        }

        /// Print the struct with a 'WARNING' label and light yellow color-coded output.
        pub fn print_warning(&self, custom: &str){
            use colorful::{Color, Colorful};
            let message = format!("({}) @WARNING '{}' = {:#?}", chrono::Utc::now(), custom, &self);
            println!("{}", message.gradient(Color::LightYellow));
        }

        /// Print the struct with an 'ERR' label and magenta color-coded output.
        pub fn print_err(&self, custom: &str){
            use colorful::{Color, Colorful};
            let message = format!("({}) @ERR '{}' = {:#?}", chrono::Utc::now(), custom, &self);
            println!("{}", message.gradient(Color::Magenta));
        }
    }
}
