use quote::quote;
use proc_macro2::TokenStream;

pub fn generate_to_rc_method() -> TokenStream {
    quote! {
        pub fn to_rc(self) -> std::rc::Rc<Self> {
            std::rc::Rc::new(self)
        }
    }
}
