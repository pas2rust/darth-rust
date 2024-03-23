use crate::helpers::{Helpers, HelpersTrait};
use quote::quote;
use syn::Ident;

pub fn generate_cache(
    helpers: Helpers,
    _cache_struct_name: &Ident,
) -> proc_macro2::TokenStream {
    let named_fields = helpers.get_named_fields().unwrap();
    let methods = named_fields.named.iter().map(|field| {
        let field_name = &field.ident.as_ref().unwrap();
        let ident = |prefix: &str| Ident::new(format!("{}_{}_in_cache", prefix, field_name).as_str(), field_name.span());
        let vec_find_by = ident("vec_find_by");
        let vec_find_many_by = ident("vec_find_many_by");
        let vec_delete_by = ident("vec_delete_by");
        let vec_update_by = ident("vec_update_by");
        quote! {
            pub fn #vec_delete_by<'a>(&'a self, cache: &'a mut Vec<&'a Self>) {
                cache.retain(|item| item.#field_name != self.#field_name);
            }
            pub fn #vec_find_by<'a>(&'a self, cache: &'a Vec<Self>) -> Option<&'a Self> {
                cache.iter().find(|item| item.#field_name == self.#field_name)
            }
            pub fn #vec_find_many_by<'a>(&'a self, cache: &'a Vec<Self>) -> Vec<&'a Self> {
                cache.iter().filter(|item| item.#field_name == self.#field_name).collect()
            }
            pub fn #vec_update_by<'a>(&'a mut self, new_value: Self, cache: &'a mut Vec<Self>) {
                if let Some(item) = cache.iter_mut().find(|item| item.#field_name == self.#field_name) {
                    *item = new_value;
                }
            }
        }
    });

    let cache_methods = quote! {
        pub fn vec_new_cache_with_limit(limit: usize) -> Vec<Self> {
            Vec::with_capacity(limit)
        }
        pub fn vec_new_cache() -> Vec<Self> {
            Vec::new()
        }
        pub fn vec_clear_cache(cache: &mut Vec<Self>) {
            cache.clear();
        }
        pub fn vec_len_cache(cache: &Vec<Self>) -> usize {
            cache.len()
        }
        pub fn vec_insert_cache(&self, cache: &mut Vec<Self>) where Self: Clone {
            cache.push(self.clone());
        }
        pub fn hash_clear_cache(cache: &mut ::std::collections::HashMap<String, Self>) {
            cache.clear();
        }
        pub fn hash_len_cache(cache: &::std::collections::HashMap<String, Self>) -> usize {
            cache.len()
        }
        pub fn hash_new_cache() -> ::std::collections::HashMap<String, Self> {
            ::std::collections::HashMap::new()
        }
        pub fn hash_set_cache(&self, key: &str, cache: &mut ::std::collections::HashMap<String, Self>) {
            cache.insert(key.to_string(), self.clone());
        }
        pub fn hash_find_by_key_in_cache<'a>(key: &str, cache: &'a ::std::collections::HashMap<String, Self>) -> Option<&'a Self> {
            cache.get(key)
        }
        pub fn hash_find_by_keys_in_cache<'a>(keys: Vec<&str>, cache: &'a ::std::collections::HashMap<String, Self>) -> Vec<Option<&'a Self>> {
            keys.iter().map(|key| cache.get(&key.to_string())).collect()
        }
        pub fn hash_delete_by_key_in_cache(key: &str, cache: &mut ::std::collections::HashMap<String, Self>) {
            cache.remove(key);
        }
        pub fn hash_delete_by_keys_in_cache(keys: Vec<&str>, cache: &mut ::std::collections::HashMap<String, Self>) {
            for key in keys {
                cache.remove(key);
            }
        }
    };

    quote! {
        #cache_methods
        #(#methods)*
    }
}
