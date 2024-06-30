mod build;
mod builder;
mod crates;
mod default;
mod from_aes;
mod from_bytes;
mod from_camellia;
mod from_chacha20;
mod from_hex;
mod from_jacc;
mod from_json;
mod from_tacc;
mod getters;
mod helpers;
mod math;
mod mut_getters;
mod printers;
mod printers_by_field;
mod printers_err_by_field;
mod printers_info_by_field;
mod printers_rust_by_field;
mod printers_success_by_field;
mod printers_warning_by_field;
mod setters;
mod to_aes;
mod to_arc;
mod to_arc_mutex;
mod to_box;
mod to_bytes;
mod to_camellia;
mod to_chacha20;
mod to_hex;
mod to_jacc;
mod to_json;
mod to_jwt;
mod to_mutex;
mod to_rc;
mod to_rc_weak;
mod to_ref_cell;
mod to_tacc;
mod validators;

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
