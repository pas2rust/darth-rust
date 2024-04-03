use proc_macro2::TokenStream;
use quote::quote;

pub fn generate_to_ref_cell_method() -> TokenStream {
    quote! {
        pub fn to_ref_cell(self) -> std::cell::RefCell<Self> {
            std::cell::RefCell::new(self)
        }
    }
}
