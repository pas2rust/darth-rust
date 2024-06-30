use proc_macro2::TokenStream;
use quote::quote;

pub fn generate_to_arc() -> TokenStream {
    quote! {
        pub fn to_arc(self) -> std::sync::Arc<Self> {
            std::sync::Arc::new(self)
        }
    }
}
