use proc_macro2::TokenStream;
use quote::quote;

pub fn generate_to_arc_mutex() -> TokenStream {
    quote! {
        pub fn to_arc_mutex(self) -> std::sync::Arc<std::sync::Mutex<Self>> {
            std::sync::Arc::new(std::sync::Mutex::new(self))
        }
    }
}
