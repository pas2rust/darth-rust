mod crates;
mod generate_from_json_method;
mod generate_getters;
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
// mod generate_validator_method;
use crates::*;

/// # Usage
/// ### run `cargo add colorful` in your project
/// ### run `cargo add chrono` in your project
/// ```rust
/// use darth_rust::DarthRust;
/// use serde::{Deserialize, Serialize};
/// #[derive(DarthRust, Debug, Serialize, Deserialize)]
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
    let mut_getters = generate_mut_getters(&input.data);
    let getters = generate_getters(&input.data);
    let setters = generate_setters(&input.data);
    let new = generate_new_method(&input.data, generics);
    let to_json = generate_to_json_method(&input);
    let from_json = generate_from_json_method(&input);
    let printers = generate_printers(&input);
    let printers_success_by_field = generate_printers_success_by_field(&input);
    let printers_by_field = generate_printers_by_field(&input);
    let printters_rust_by_field = generate_printers_rust_by_field(&input);
    let printers_info_by_field = generate_printers_info_by_field(&input);
    let printers_err_by_field = generate_printers_err_by_field(&input);
    let printers_warning_by_field = generate_printers_warning_by_field(&input);
    let expanded = quote! {
        impl #struct_name {
            #mut_getters
            #from_json
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
        }
    };
    expanded.into()
}
