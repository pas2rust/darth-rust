use proc_macro2::TokenStream;
use quote::quote;

pub fn generate_to_mutex() -> TokenStream {
    quote! {
        pub fn to_mutex(self) -> std::sync::Mutex<Self> {
            std::sync::Mutex::new(self)
        }
    }
}
