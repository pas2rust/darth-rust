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
mod generate_to_ref_cell_method;
mod generate_printers_success_by_field;
mod generate_printers_warning_by_field;
mod generate_setters;
mod generate_to_rc_weak_method;
mod generate_to_box_method;
mod generate_to_json_method;
mod generate_vec_cache_sync;
mod generate_to_rc_method;
mod helpers;
mod structs;
use build::{Build, BuildTrait};
use crates::*;
use structs::{Structs, StructsTrait};

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
#[proc_macro_derive(DarthRust, attributes(pattern))]
pub fn darth_rust(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let cache_struct = Structs::gen_cache_struct(input.clone().ident);
    let build = Build::new(input.clone());
    let struct_name = &input.ident;
    let methods = build.gen();
    let expanded = quote! {
        #cache_struct
        impl #struct_name {
            #methods
        }
    };
    expanded.into()
}
