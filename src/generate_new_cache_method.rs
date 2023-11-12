use quote::quote;
use syn::{Data, Fields, Ident};
use chrono::DateTime;

pub struct Cache<T, D> {
    pub items: Vec<T>,
    pub threads: u64,
    pub max_ram: f64,
    pub expiration: DateTime<D>,
    pub last_access: DateTime<D>
}

pub fn generate_new_cache_method(fields: &Data) -> proc_macro2::TokenStream {
    
}
