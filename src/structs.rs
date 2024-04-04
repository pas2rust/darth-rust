use crate::helpers::{Helpers, HelpersTrait};
pub use quote::quote;
use syn::Ident;
pub struct Structs;

pub trait StructsTrait {
    fn gen_structs(struct_name: Ident) -> proc_macro2::TokenStream;
}

impl StructsTrait for Structs {
    fn gen_structs(struct_name: Ident) -> proc_macro2::TokenStream {
        let cache_struct_name =
            Helpers::new_ident_camel_case("Cache", struct_name.clone());
        let accessed =
            Helpers::new_ident_camel_case("Accessed", struct_name.clone());
        let metadata =
            Helpers::new_ident_camel_case("Metadata", struct_name.clone());
        // let observer = Helpers::new_ident_camel_case("Observer",
        // struct_name.clone());
        quote! {
            pub struct #metadata {
                pub utc: chrono::DateTime<chrono::Utc>,
                pub local: chrono::DateTime<chrono::Local>,
            }
            pub struct #accessed {
                pub last_accessed: #metadata,
                pub count: usize,
            }
            pub struct #cache_struct_name<'a, T> {
                pub items: T,
                pub expiration_by_item: Vec<(&'a str, Option<usize>)>,
                pub accessed: #accessed,
                pub created_at: #metadata,
                pub updated: #metadata
            }
        }
    }
}
