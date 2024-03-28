use crate::helpers::{Helpers, HelpersTrait};
pub use quote::quote;
use syn::Ident;
pub struct Structs;

pub trait StructsTrait {
    fn gen_cache_struct(struct_name: Ident) -> proc_macro2::TokenStream;
}

impl StructsTrait for Structs {
    fn gen_cache_struct(struct_name: Ident) -> proc_macro2::TokenStream {
        let cache_struct_name = Helpers::new_ident_camel_case("Cache", struct_name);
        quote! {
            pub struct #cache_struct_name<'a, T> {
                pub items: T,
                pub expiration_by_item: Vec<(&'a str, Option<usize>)>,
                pub last_access_by_utc: chrono::DateTime<chrono::Utc>,
                pub last_access_by_local: chrono::DateTime<chrono::Local>,
                pub created_at_local: chrono::DateTime<chrono::Local>,
                pub created_at_utc: chrono::DateTime<chrono::Utc>,
                pub updated_at_local: chrono::DateTime<chrono::Local>,
                pub updated_at_utc: chrono::DateTime<chrono::Utc>,
                pub access_count: usize,
            }
        }
    }
}
