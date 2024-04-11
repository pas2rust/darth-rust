use crate::crates::*;
use crate::helpers::{Helpers, HelpersTrait};
use proc_macro2::TokenStream;
use syn::DeriveInput;

pub struct Build {
    pub derive_input: DeriveInput,
}

pub trait BuildTrait {
    fn gen(&self) -> TokenStream;
    fn new(derive_input: DeriveInput) -> Self;
}

impl BuildTrait for Build {
    fn gen(&self) -> TokenStream {
        let mut tokens = TokenStream::new();
        let helpers = Helpers::new()
            .input(self.derive_input.clone())
            .cache_name(Helpers::new_ident_camel_case(
                "Cache",
                self.derive_input.ident.clone(),
            ));
        let streams = vec![
            #[cfg(feature = "json")]
            generate_to_json(helpers.clone()),
            #[cfg(feature = "json")]
            generate_from_json(helpers.clone()),
            #[cfg(feature = "math")]
            generate_math(helpers.clone()),
            #[cfg(feature = "build")]
            generate_default(helpers.clone()),
            #[cfg(feature = "get")]
            generate_getters(helpers.clone()),
            #[cfg(feature = "get_mut")]
            generate_mut_getters(helpers.clone()),
            #[cfg(feature = "print_by_field")]
            generate_printers_by_field(helpers.clone()),
            #[cfg(feature = "print_by_field")]
            generate_printers_err_by_field(helpers.clone()),
            #[cfg(feature = "print_by_field")]
            generate_printers_info_by_field(
                helpers.clone(),
            ),
            #[cfg(feature = "print_by_field")]
            generate_printers_rust_by_field(
                helpers.clone(),
            ),
            #[cfg(feature = "print_by_field")]
            generate_printers_success_by_field(
                helpers.clone(),
            ),
            #[cfg(feature = "print_by_field")]
            generate_printers_warning_by_field(
                helpers.clone(),
            ),
            #[cfg(feature = "print")]
            generate_printers(),
            #[cfg(feature = "set")]
            generate_setters(helpers.clone()),
            #[cfg(feature = "build")]
            generate_build(helpers.clone()),
            generate_to_box(),
            generate_to_rc(),
            generate_to_ref_cell(),
            generate_to_rc_weak(),
        ];
        for token in streams {
            tokens.extend(token);
        }
        tokens
    }
    fn new(derive_input: DeriveInput) -> Self {
        Self { derive_input }
    }
}
