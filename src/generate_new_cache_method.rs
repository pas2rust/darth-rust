use quote::quote;
use syn::{Data, Fields, Ident};
use chrono::{DateTime, Utc};

pub struct Cache<T> {
    pub items: Vec<T>,
    pub threads: u64,
    pub max_ram: f64,
    pub expiration: DateTime<Utc>,
}

pub fn generate_new_cache_method(fields: &Data) -> proc_macro2::TokenStream {
    
}
