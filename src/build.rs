use proc_macro2::TokenStream;
use syn::DeriveInput;

use crate::{
    generate_build_method, generate_default_method, generate_from_json_method,
    generate_getters, generate_hash_cache_sync, generate_math_methods,
    generate_mut_getters, generate_printers, generate_printers_by_field,
    generate_printers_err_by_field, generate_printers_info_by_field,
    generate_printers_rust_by_field, generate_printers_success_by_field,
    generate_printers_warning_by_field, generate_setters,
    generate_to_json_method, generate_vec_cache_sync,
    helpers::{Helpers, HelpersTrait},
    structs::{Structs, StructsTrait},
};

pub struct Build {
    pub derive_input: DeriveInput,
}

pub trait BuildTrait {
    fn gen_vec_cache_sync(&self) -> TokenStream;
    fn gen_math(&self) -> TokenStream;
    fn gen_default(&self) -> TokenStream;
    fn gen_from_json(&self) -> TokenStream;
    fn gen_getters(&self) -> TokenStream;
    fn gen_mut_getters(&self) -> TokenStream;
    fn gen_printers_by_field(&self) -> TokenStream;
    fn gen_printers_err_by_field(&self) -> TokenStream;
    fn gen_printers_info_by_field(&self) -> TokenStream;
    fn gen_printers_rust_by_field(&self) -> TokenStream;
    fn gen_printers_success_by_field(&self) -> TokenStream;
    fn gen_printers_warning_by_field(&self) -> TokenStream;
    fn gen_printers(&self) -> TokenStream;
    fn gen_setters(&self) -> TokenStream;
    fn gen_to_json(&self) -> TokenStream;
    fn gen_cache_struct(&self) -> TokenStream;
    fn gen_hash_cache_sync(&self) -> TokenStream;
    fn new(derive_input: DeriveInput) -> Self;
    fn gen_pattern_build(&self) -> TokenStream;
}

impl BuildTrait for Build {
    fn new(derive_input: DeriveInput) -> Self {
        Self { derive_input }
    }
    fn gen_math(&self) -> TokenStream {
        generate_math_methods(Helpers::new().input(self.derive_input.clone()))
    }
    fn gen_default(&self) -> TokenStream {
        generate_default_method(Helpers::new().input(self.derive_input.clone()))
    }
    fn gen_from_json(&self) -> TokenStream {
        generate_from_json_method(
            Helpers::new().input(self.derive_input.clone()),
        )
    }
    fn gen_getters(&self) -> TokenStream {
        generate_getters(Helpers::new().input(self.derive_input.clone()))
    }
    fn gen_mut_getters(&self) -> TokenStream {
        generate_mut_getters(Helpers::new().input(self.derive_input.clone()))
    }
    fn gen_printers_by_field(&self) -> TokenStream {
        generate_printers_by_field(
            Helpers::new().input(self.derive_input.clone()),
        )
    }
    fn gen_printers_err_by_field(&self) -> TokenStream {
        generate_printers_err_by_field(
            Helpers::new().input(self.derive_input.clone()),
        )
    }
    fn gen_printers_info_by_field(&self) -> TokenStream {
        generate_printers_info_by_field(
            Helpers::new().input(self.derive_input.clone()),
        )
    }
    fn gen_printers_rust_by_field(&self) -> TokenStream {
        generate_printers_rust_by_field(
            Helpers::new().input(self.derive_input.clone()),
        )
    }
    fn gen_printers_success_by_field(&self) -> TokenStream {
        generate_printers_success_by_field(
            Helpers::new().input(self.derive_input.clone()),
        )
    }
    fn gen_printers_warning_by_field(&self) -> TokenStream {
        generate_printers_warning_by_field(
            Helpers::new().input(self.derive_input.clone()),
        )
    }
    fn gen_printers(&self) -> TokenStream {
        generate_printers()
    }
    fn gen_setters(&self) -> TokenStream {
        generate_setters(Helpers::new().input(self.derive_input.clone()))
    }
    fn gen_to_json(&self) -> TokenStream {
        generate_to_json_method(Helpers::new().input(self.derive_input.clone()))
    }
    fn gen_cache_struct(&self) -> TokenStream {
        Structs::gen_cache_struct(self.derive_input.ident.clone())
    }
    fn gen_hash_cache_sync(&self) -> TokenStream {
        generate_hash_cache_sync(
            Helpers::new().input(self.derive_input.clone()).cache_name(
                Helpers::new_ident_camel_case(
                    "Cache",
                    self.derive_input.ident.clone(),
                ),
            ),
        )
    }
    fn gen_pattern_build(&self) -> TokenStream {
        generate_build_method(Helpers::new().input(self.derive_input.clone()))
    }
    fn gen_vec_cache_sync(&self) -> TokenStream {
        generate_vec_cache_sync(
            Helpers::new().input(self.derive_input.clone()).cache_name(
                Helpers::new_ident_camel_case(
                    "Cache",
                    self.derive_input.ident.clone(),
                ),
            ),
        )
    }
}
