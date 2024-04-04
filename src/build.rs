use proc_macro2::TokenStream;
use syn::DeriveInput;

use crate::{
    generate_build_method, generate_default_method, generate_from_json_method,
    generate_getters, generate_math_methods,
    generate_mut_getters, generate_printers, generate_printers_by_field,
    generate_printers_err_by_field, generate_printers_info_by_field,
    generate_printers_rust_by_field, generate_printers_success_by_field,
    generate_printers_warning_by_field, generate_setters,
    generate_to_box_method, generate_to_json_method, generate_to_rc_method,
    generate_to_rc_weak_method, generate_to_ref_cell_method,
    helpers::{Helpers, HelpersTrait},
};

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
            generate_math_methods(helpers.clone()),
            generate_default_method(helpers.clone()),
            generate_from_json_method(helpers.clone()),
            generate_getters(helpers.clone()),
            generate_mut_getters(helpers.clone()),
            generate_printers_by_field(helpers.clone()),
            generate_printers_err_by_field(helpers.clone()),
            generate_printers_info_by_field(helpers.clone()),
            generate_printers_rust_by_field(helpers.clone()),
            generate_printers_success_by_field(helpers.clone()),
            generate_printers_warning_by_field(helpers.clone()),
            generate_printers(),
            generate_setters(helpers.clone()),
            generate_to_json_method(helpers.clone()),
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
