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
use crates::*;

/// # Usage
/// ### run `cargo add colorful` in your project
/// ### run `cargo add chrono` in your project
/// ### run `cargo add serde` in your project
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
    let struct_name = &input.ident;
    let generics = &input.generics;
    let data = &input.data;
    let mut_getters = generate_mut_getters(data);
    let getters = generate_getters(data);
    let setters = generate_setters(data);
    let calc_methods = generate_calc_methods(data);
    let new = generate_new_method(data, generics);
    let to_json = generate_to_json_method(&input);
    let from_json = generate_from_json_method(&input);
    let printers = generate_printers(&input);
    let printers_success_by_field = generate_printers_success_by_field(&input);
    let printers_by_field = generate_printers_by_field(&input);
    let printters_rust_by_field = generate_printers_rust_by_field(&input);
    let printers_info_by_field = generate_printers_info_by_field(&input);
    let printers_err_by_field = generate_printers_err_by_field(&input);
    let printers_warning_by_field = generate_printers_warning_by_field(&input);
    let default = generate_default_method(data, generics);
    let cache = generate_cache(data);
    let is_regex = generate_is_regex_method(data);
    let is_range = generate_is_range_method(data);
    let expanded = quote! {
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
