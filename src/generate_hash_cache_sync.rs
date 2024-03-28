use crate::helpers::{Helpers, HelpersTrait};
use quote::quote;
use syn::Ident;

pub fn generate_hash_cache_sync(
    helpers: Helpers,
    cache_struct_name: &Ident,
) -> proc_macro2::TokenStream {
    helpers.get_named_fields().unwrap();
    quote! {
        pub fn hash_cache_delete_by_keys<'a>(keys: Vec<&'a str>, cache: &mut ::std::collections::HashMap<String, #cache_struct_name<Vec<&'a Self>>>) {
            for key in keys {
                cache.remove(key);
            }
        }
        pub fn hash_cache_delete_by<'a>(key: &'a str, cache: &'a mut std::collections::HashMap<&'a str, #cache_struct_name<Vec<&'a Self>>>) {
            cache.remove(key);
        }
        /*pub fn hash_cache_find_by<'a>(key: &'a str, cache: &'a std::collections::HashMap<&'a str, #cache_struct_name<Vec<&'a Self>>>) -> Option<#cache_struct_name<Vec<&'a Self>>> {
            cache.get(key)
        }*/
        /*pub fn hash_cache_set_by<'a>(key: &'a str, new_value: Self, cache:&'a mut std::collections::HashMap<&'a str, #cache_struct_name<Vec<&'a Self>>>) {
            cache.insert(key, new_value);
        }*/
    }
}
