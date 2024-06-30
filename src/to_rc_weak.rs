use proc_macro2::TokenStream;
use quote::quote;

pub fn generate_to_rc_weak() -> TokenStream {
    quote! {
        pub fn to_rc_weak(self) -> std::rc::Weak<Self> {
            std::rc::Rc::downgrade(&self.to_rc())
        }
    }
}
