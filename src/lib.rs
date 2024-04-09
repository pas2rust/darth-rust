mod build;
mod crates;
#[cfg(feature = "build")]
mod generate_build_method;
#[cfg(feature = "build")]
mod generate_default_method;
#[cfg(feature = "json")]
mod generate_from_json_method;
#[cfg(feature = "get")]
mod generate_getters;
#[cfg(feature = "math")]
mod generate_math_methods;
#[cfg(feature = "get_mut")]
mod generate_mut_getters;
#[cfg(feature = "print")]
mod generate_printers;
#[cfg(feature = "print_by_field")]
mod generate_printers_by_field;
#[cfg(feature = "print_by_field")]
mod generate_printers_err_by_field;
#[cfg(feature = "print_by_field")]
mod generate_printers_info_by_field;
#[cfg(feature = "print_by_field")]
mod generate_printers_rust_by_field;
#[cfg(feature = "print_by_field")]
mod generate_printers_success_by_field;
#[cfg(feature = "print_by_field")]
mod generate_printers_warning_by_field;
#[cfg(feature = "set")]
mod generate_setters;
#[cfg(feature = "json")]
mod generate_to_json_method;

mod generate_to_box_method;
mod generate_to_rc_method;
mod generate_to_rc_weak_method;
mod generate_to_ref_cell_method;
mod helpers;

use crates::*;
use helpers::{Helpers, HelpersTrait};

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
#[proc_macro_derive(
    DarthRust,
    attributes(pattern, min, max)
)]
pub fn darth_rust(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let build = Build::new(input.clone());
    let mut helpers = Helpers::new().input(input.clone());
    helpers.add_traits_to_generics();
    let (impl_generics, ty_generics, where_clause) =
        helpers.generics_split_for_impl();
    let struct_name = &input.ident;
    let methods = build.gen();
    let expanded = quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            #methods
        }
    };
    expanded.into()
}
