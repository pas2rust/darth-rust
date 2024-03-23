mod build;
mod crates;
mod generate_cache;
mod generate_calc_methods;
mod generate_default_method;
mod generate_from_json_method;
mod generate_getters;
mod generate_is_range_method;
mod generate_is_regex_method;
mod generate_mut_getters;
mod generate_new_method;
mod generate_printers;
mod generate_printers_by_field;
mod generate_printers_err_by_field;
mod generate_printers_info_by_field;
mod generate_printers_rust_by_field;
mod generate_printers_success_by_field;
mod generate_printers_warning_by_field;
mod generate_setters;
mod generate_to_json_method;
mod helpers;
use build::{Build, BuildTrait};
use crates::*;

/// # Usage
/// ### run `cargo add regex`
/// ### run `cargo add colorful`
/// ### run `cargo add chrono`
/// ### run `cargo add serde`
/// ### run `cargo add serde_json`
/// ```rust
/// use darth_rust::DarthRust;
/// use serde::{Deserialize, Serialize};
/// #[derive(DarthRust, Debug, Serialize, Deserialize, Clone, Default)]
/// struct MyStruct {
///     field1: String,
///     field2: i32,
/// }
/// ```
#[proc_macro_derive(DarthRust)]
pub fn darth_rust(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let build = Build::new(input.clone());
    let struct_name = &input.ident;
    let cache_struct_name = syn::Ident::new(
        &format!("Cache{}", struct_name),
        proc_macro2::Span::call_site(),
    );
    let mut_getters = build.gen_mut_getters();
    let getters = build.gen_getters();
    let setters = build.gen_setters();
    let calc_methods = build.gen_calc();
    let new = build.gen_new();
    let to_json = build.gen_to_json();
    let from_json = build.gen_from_json();
    let printers = build.gen_printers();
    let printers_success_by_field = build.gen_printers_success_by_field();
    let printers_by_field = build.gen_printers_by_field();
    let printters_rust_by_field = build.gen_printers_rust_by_field();
    let printers_info_by_field = build.gen_printers_info_by_field();
    let printers_err_by_field = build.gen_printers_err_by_field();
    let printers_warning_by_field = build.gen_printers_warning_by_field();
    let default = build.gen_default();
    let cache = build.gen_cache();
    let is_regex = build.gen_is_regex();
    let is_range = build.gen_is_range();
    let cache_struct = quote! {
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
    };
    let expanded = quote! {
        #cache_struct
        impl #struct_name {
            #cache
            #mut_getters
            #from_json
            #default
            #to_json
            #new
            #getters
            #setters
            #printers
            #printers_success_by_field
            #printers_by_field
            #printters_rust_by_field
            #printers_info_by_field
            #printers_warning_by_field
            #printers_err_by_field
            #calc_methods
            #is_regex
            #is_range
        }
    };
    expanded.into()
}
