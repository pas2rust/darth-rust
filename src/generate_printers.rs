use quote::quote;
use syn::{Data, DeriveInput};

/// printter
pub fn generate_printers(input: &DeriveInput) -> proc_macro2::TokenStream {
    match &input.data {
        Data::Struct(data_struct) => &data_struct.fields,
        _ => {
            return syn::Error::new_spanned(&input, "print can only be derived for structs")
                .to_compile_error()
                .into();
        }
    };
    quote! {
        /// Print the struct with various labels and color-coded output.
        fn print_all_levels(&self) {
            self.print();
            self.print_rust();
            self.print_info();
            self.print_success();
            self.print_warning();
            self.print_err();
        }

        /// Print the struct with a default label and color-coded output.
        fn print(&self) {
            let message = format!("{} @PRINT ➝ {:#?}", chrono::Utc::now(), &self);
            println!("{}", message);
        }

        /// Print the struct with a 'RUST' label and dark red color-coded output.
        fn print_rust(&self) {
            use colorful::{Color, Colorful};
            let message = format!("{} @RUST ➝ {:#?}", chrono::Utc::now(), &self);
            println!("{}", message.gradient(Color::DarkRed2));
        }

        /// Print the struct with an 'INFO' label and purple color-coded output.
        fn print_info(&self) {
            use colorful::{Color, Colorful};
            let message = format!("{} @INFO ➝ {:#?}", chrono::Utc::now(), &self);
            println!("{}", message.gradient(Color::Purple3));
        }

        /// Print the struct with a 'SUCCESS' label and light green color-coded output.
        fn print_success(&self) {
            use colorful::{Color, Colorful};
            let message = format!("{} @SUCCESS ➝ {:#?}", chrono::Utc::now(), &self);
            println!("{}", message.gradient(Color::LightGreen));
        }

        /// Print the struct with a 'WARNING' label and light yellow color-coded output.
        fn print_warning(&self) {
            use colorful::{Color, Colorful};
            let message = format!("{} @WARNING ➝ {:#?}", chrono::Utc::now(), &self);
            println!("{}", message.gradient(Color::LightYellow));
        }

        /// Print the struct with an 'ERR' label and magenta color-coded output.
        fn print_err(&self) {
            use colorful::{Color, Colorful};
            let message = format!("{} @ERR ➝ {:#?}", chrono::Utc::now(), &self);
            println!("{}", message.gradient(Color::Magenta));
        }
    }
}
