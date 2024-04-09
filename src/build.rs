use proc_macro2::TokenStream;
use syn::DeriveInput;

#[cfg(feature = "build")]
use crate::generate_build_method;

#[cfg(feature = "build")]
use crate::generate_default_method;

#[cfg(feature = "json")]
use crate::generate_from_json_method;

#[cfg(feature = "get")]
use crate::generate_getters;

#[cfg(feature = "math")]
use crate::generate_math_methods;

#[cfg(feature = "get_mut")]
use crate::generate_mut_getters;

#[cfg(feature = "print")]
use crate::generate_printers;

#[cfg(feature = "print_by_field")]
use crate::generate_printers_by_field;

#[cfg(feature = "print_by_field")]
use crate::generate_printers_err_by_field;

#[cfg(feature = "print_by_field")]
use crate::generate_printers_info_by_field;

#[cfg(feature = "print_by_field")]
use crate::generate_printers_rust_by_field;

#[cfg(feature = "print_by_field")]
use crate::generate_printers_success_by_field;

#[cfg(feature = "print_by_field")]
use crate::generate_printers_warning_by_field;

#[cfg(feature = "set")]
use crate::generate_setters;

use crate::generate_to_box_method;

#[cfg(feature = "json")]
use crate::generate_to_json_method;

use crate::generate_to_rc_method;

use crate::generate_to_rc_weak_method;

use crate::generate_to_ref_cell_method;

use crate::helpers::{Helpers, HelpersTrait};

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
            #[cfg(feature = "math")]
            generate_math_methods(helpers.clone()),
            #[cfg(feature = "build")]
            generate_default_method(helpers.clone()),
            #[cfg(feature = "json")]
            generate_from_json_method(helpers.clone()),
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
            #[cfg(feature = "json")]
            generate_to_json_method(helpers.clone()),
            #[cfg(feature = "build")]
            generate_build_method(helpers.clone()),
            generate_to_box_method(),
            generate_to_rc_method(),
            generate_to_ref_cell_method(),
            generate_to_rc_weak_method(),
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
