use colorful::{Color, Colorful};
use syn::Ident;

use crate::count_methods_generated::count_methods_generated;

pub fn generate_count_message(
    struct_name: &Ident,
    token_stream: &proc_macro2::TokenStream,
) {
    let count = count_methods_generated(token_stream);
    let welcome = "Welcome to the dark side!".to_uppercase();
    let message = format!(
        "You've eliminated writing {} methods with struct `{}` and saved {} \
         minutes of your life!",
        count,
        struct_name,
        count * 30 / 60
    )
    .to_uppercase();

    println!(
        "🦀{}\n🍷{}",
        welcome.gradient(Color::LightRed),
        message.gradient(Color::DarkMagenta2)
    );
}
