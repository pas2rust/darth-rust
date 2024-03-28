use crate::helpers::{Helpers, HelpersTrait};
use quote::quote;
use syn::Ident;

pub fn generate_vec_cache_sync(
    helpers: Helpers,
    cache_struct_name: &Ident,
) -> proc_macro2::TokenStream {
    let methods = helpers.get_named_fields().unwrap()
    .named.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let vec_find_by =  Helpers::new_ident("sync_vec_cache_find_by", field_name.clone());
        let vec_find_many_by =  Helpers::new_ident("sync_vec_cache_find_many_by", field_name.clone());
        let vec_delete_by =  Helpers::new_ident("sync_vec_cache_delete_by", field_name.clone());
        let vec_update_by =  Helpers::new_ident("sync_vec_cache_update_by", field_name.clone());
        quote! {
            pub fn #vec_delete_by<'a>(&'a self, cache: &'a mut #cache_struct_name<Vec<&'a Self>>) {
                cache.items.retain(|item| item.#field_name != self.#field_name);
            }
            pub fn #vec_find_by<'a>(&'a self, cache: &'a mut #cache_struct_name<Vec<&'a Self>>) -> Option<&'a &Self> {
                cache.items.iter().find(|item| item.#field_name == self.#field_name)
            }
            pub fn #vec_find_many_by<'a>(&'a self, cache: &'a mut #cache_struct_name<Vec<&'a Self>>) -> Vec<&'a &Self> {
                cache.items.iter().filter(|item| item.#field_name == self.#field_name).collect()
            }
            pub fn #vec_update_by<'a>(&'a self, new_value: &'a Self, cache: &'a mut #cache_struct_name<Vec<&'a Self>>) {
                if let Some(index) = cache.items.iter().position(|item| item.#field_name == self.#field_name) {
                    cache.items[index] = new_value;
                }
            }
        }
    });

    quote! {
        #(#methods)*
    }
}
