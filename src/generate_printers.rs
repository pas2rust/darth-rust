#![cfg(feature = "print")]
use quote::quote;

pub fn generate_printers() -> proc_macro2::TokenStream {
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
            let message = format!("({}) @PRINT '{}' = {:#?}", chrono::Local::now(), custom, &self);
            println!("{}", message);
        }

        /// Print the struct with a 'RUST' label and dark red color-coded output.
        pub fn print_rust(&self, custom: &str) {
            use colorful::{Color, Colorful};
            let message = format!("({}) @RUST '{}' = {:#?}", chrono::Local::now(), custom, &self);
            println!("{}", message.gradient(Color::DarkRed2));
        }

        /// Print the struct with an 'INFO' label and purple color-coded output.
        pub fn print_info(&self, custom: &str) {
            use colorful::{Color, Colorful};
            let message = format!("({}) @INFO '{}' = {:#?}", chrono::Local::now(), custom, &self);
            println!("{}", message.gradient(Color::Purple3));
        }

        /// Print the struct with a 'SUCCESS' label and light green color-coded output.
        pub fn print_success(&self, custom: &str) {
            use colorful::{Color, Colorful};
            let message = format!("({}) @SUCCESS '{}' = {:#?}", chrono::Local::now(), custom, &self);
            println!("{}", message.gradient(Color::LightGreen));
        }

        /// Print the struct with a 'WARNING' label and light yellow color-coded output.
        pub fn print_warning(&self, custom: &str){
            use colorful::{Color, Colorful};
            let message = format!("({}) @WARNING '{}' = {:#?}", chrono::Local::now(), custom, &self);
            println!("{}", message.gradient(Color::LightYellow));
        }

        /// Print the struct with an 'ERR' label and magenta color-coded output.
        pub fn print_err(&self, custom: &str){
            use colorful::{Color, Colorful};
            let message = format!("({}) @ERR '{}' = {:#?}", chrono::Local::now(), custom, &self);
            println!("{}", message.gradient(Color::Magenta));
        }
    }
}
