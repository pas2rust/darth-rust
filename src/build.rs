use proc_macro2::TokenStream;
use syn::DeriveInput;

use crate::{
    generate_build_method, generate_default_method, generate_from_json_method,
    generate_getters, generate_hash_cache_sync, generate_is_range_method,
    generate_is_regex_method, generate_math_methods, generate_mut_getters,
    generate_new_method, generate_printers, generate_printers_by_field,
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
    fn gen_is_range(&self) -> TokenStream;
    fn gen_is_regex(&self) -> TokenStream;
    fn gen_mut_getters(&self) -> TokenStream;
    fn gen_new(&self) -> TokenStream;
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
    fn gen_vec_cache_sync(&self) -> TokenStream {
        let input = &self.derive_input;
        let helpers = Helpers::new(input.data.clone());
        let cache_name = syn::Ident::new(
            &format!("Cache{}", input.ident.clone()),
            proc_macro2::Span::call_site(),
        );
        generate_vec_cache_sync(helpers, &cache_name)
    }
    fn gen_math(&self) -> TokenStream {
        let input = &self.derive_input;
        let helpers = Helpers::new(input.data.clone());
        generate_math_methods(helpers)
    }
    fn gen_default(&self) -> TokenStream {
        let input = &self.derive_input;
        let helpers = Helpers::new(input.data.clone());
        generate_default_method(helpers, &input.generics)
    }
    fn gen_from_json(&self) -> TokenStream {
        let input = &self.derive_input;
        let struct_name = input.ident.clone();
        let helpers = Helpers::new(input.data.clone());
        generate_from_json_method(helpers, &struct_name)
    }
    fn gen_getters(&self) -> TokenStream {
        let input = &self.derive_input;
        let helpers = Helpers::new(input.data.clone());
        generate_getters(helpers)
    }
    fn gen_is_range(&self) -> TokenStream {
        let input = &self.derive_input;
        let helpers = Helpers::new(input.data.clone());
        generate_is_range_method(helpers)
    }
    fn gen_is_regex(&self) -> TokenStream {
        let input = &self.derive_input;
        let helpers = Helpers::new(input.data.clone());
        generate_is_regex_method(helpers)
    }
    fn gen_mut_getters(&self) -> TokenStream {
        let input = &self.derive_input;
        let helpers = Helpers::new(input.data.clone());
        generate_mut_getters(helpers)
    }
    fn gen_new(&self) -> TokenStream {
        let input = &self.derive_input;
        let helpers = Helpers::new(input.data.clone());
        generate_new_method(helpers, &input.generics)
    }
    fn gen_printers_by_field(&self) -> TokenStream {
        let input = &self.derive_input;
        let helpers = Helpers::new(input.data.clone());
        generate_printers_by_field(helpers, input.ident.clone())
    }
    fn gen_printers_err_by_field(&self) -> TokenStream {
        let input = &self.derive_input;
        let helpers = Helpers::new(input.data.clone());
        generate_printers_err_by_field(helpers, input.ident.clone())
    }
    fn gen_printers_info_by_field(&self) -> TokenStream {
        let input = &self.derive_input;
        let helpers = Helpers::new(input.data.clone());
        generate_printers_info_by_field(helpers, input.ident.clone())
    }
    fn gen_printers_rust_by_field(&self) -> TokenStream {
        let input = &self.derive_input;
        let helpers = Helpers::new(input.data.clone());
        generate_printers_rust_by_field(helpers, input.ident.clone())
    }
    fn gen_printers_success_by_field(&self) -> TokenStream {
        let input = &self.derive_input;
        let helpers = Helpers::new(input.data.clone());
        generate_printers_success_by_field(helpers, input.ident.clone())
    }
    fn gen_printers_warning_by_field(&self) -> TokenStream {
        let input = &self.derive_input;
        let helpers = Helpers::new(input.data.clone());
        generate_printers_warning_by_field(helpers, input.ident.clone())
    }
    fn gen_printers(&self) -> TokenStream {
        generate_printers()
    }
    fn gen_setters(&self) -> TokenStream {
        let input = &self.derive_input;
        let helpers = Helpers::new(input.data.clone());
        generate_setters(helpers)
    }
    fn gen_to_json(&self) -> TokenStream {
        let input = &self.derive_input;
        let helpers = Helpers::new(input.data.clone());
        generate_to_json_method(helpers)
    }
    fn gen_cache_struct(&self) -> TokenStream {
        let input = &self.derive_input;
        Structs::gen_cache_struct(input.ident.clone())
    }
    fn gen_hash_cache_sync(&self) -> TokenStream {
        let input = &self.derive_input;
        let helpers = Helpers::new(input.data.clone());
        let cache_name = syn::Ident::new(
            &format!("Cache{}", input.ident.clone()),
            proc_macro2::Span::call_site(),
        );
        generate_hash_cache_sync(helpers, &cache_name)
    }
    fn gen_pattern_build(&self) -> TokenStream {
        let input = &self.derive_input;
        let helpers = Helpers::new(input.data.clone());
        generate_build_method(helpers)
    }
}
