mod build;
mod crates;
#[cfg(feature = "build")]
mod generate_build;
#[cfg(feature = "build")]
mod generate_default;
#[cfg(feature = "json")]
mod generate_from_json;
#[cfg(feature = "get")]
mod generate_getters;
#[cfg(feature = "math")]
mod generate_math;
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
mod generate_to_json;
mod validators;

mod generate_to_box;
mod generate_to_rc;
mod generate_to_rc_weak;
mod generate_to_ref_cell;
mod helpers;

use crates::*;
use helpers::{Helpers, HelpersTrait};

#[proc_macro_derive(
    DarthRust,
    attributes(
        pattern,
        pattern_notify,
        min_notify,
        max_notify,
        min,
        max,
        attr
    )
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
