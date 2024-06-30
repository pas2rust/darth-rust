use proc_macro2::TokenStream;
use quote::quote;

pub fn generate_to_rc() -> TokenStream {
    quote! {
        pub fn to_rc(self) -> std::rc::Rc<Self> {
            std::rc::Rc::new(self)
        }
    }
}
