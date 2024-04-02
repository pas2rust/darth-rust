use quote::quote;
use proc_macro2::TokenStream;

pub fn generate_to_rc_weak_method() -> TokenStream {
    quote! {
        pub fn to_rc_weak(self) -> std::rc::Weak<Self> {
            std::rc::Rc::downgrade(&self.to_rc())
        }
    }
}