use quote::quote;

pub fn generate_to_box_method() -> proc_macro2::TokenStream {
    quote! {
        pub fn to_box(self) -> Box<Self> {
            Box::new(self)
        }
    }
}
