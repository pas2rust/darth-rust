mod build;
mod crates;
mod generate_build_method;
mod generate_default_method;
mod generate_from_json_method;
mod generate_getters;
mod generate_hash_cache_sync;
mod generate_math_methods;
mod generate_mut_getters;
mod generate_printers;
mod generate_printers_by_field;
mod generate_printers_err_by_field;
mod generate_printers_info_by_field;
mod generate_printers_rust_by_field;
mod generate_printers_success_by_field;
mod generate_printers_warning_by_field;
mod generate_setters;
mod generate_to_json_method;
mod generate_vec_cache_sync;
mod helpers;
mod structs;
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
    let mut_getters = build.gen_mut_getters();
    let getters = build.gen_getters();
    let setters = build.gen_setters();
    let math = build.gen_math();
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
    let vec_cache_sync = build.gen_vec_cache_sync();
    let hash_cache_sync = build.gen_hash_cache_sync();
    let cache_struct = build.gen_cache_struct();
    let pattern_build = build.gen_pattern_build();
    let expanded = quote! {
        #cache_struct
        impl #struct_name {
            #vec_cache_sync
            #mut_getters
            #from_json
            #default
            #to_json
            #getters
            #setters
            #printers
            #printers_success_by_field
            #printers_by_field
            #printters_rust_by_field
            #printers_info_by_field
            #printers_warning_by_field
            #printers_err_by_field
            #math
            #hash_cache_sync
            #pattern_build
        }
    };
    expanded.into()
}
