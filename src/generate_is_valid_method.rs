use quote::quote;
use syn::{Lit, DeriveInput, Data};
use darling::FromMeta;
use darling::ast::NestedMeta;

#[derive(Debug, FromMeta)]
struct Range {
    start: Lit,
    end: Lit,
}

#[derive(Debug, FromMeta, Default)]
struct Is {
    #[darling(default)]
    range: Option<Range>,
    #[darling(default)]
    pattern: Option<Lit>,
}

pub fn generate_is_valid_method(input: &DeriveInput) -> proc_macro2::TokenStream {
    let nested_metas: Vec<NestedMeta> = input.attrs
        .iter()
        .flat_map(|attr| attr.parse_args::<NestedMeta>().ok())
        .collect();
    // use unwrap_or_default instead of unwrap
    let is = nested_metas.iter().find_map(|meta: &NestedMeta| Is::from_nested_meta(meta).ok()).unwrap_or_default();        
    let struct_name = &input.ident;
    let fields = &input.data;
    let mut range_checks = Vec::new();
    let mut regex_checks = Vec::new();
    match &fields {
        Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields_named) => {
                for field in &fields_named.named {
                    let field_name = &field.ident;
                    if let Some(range) = &is.range {
                        let start = &range.start;
                        let end = &range.end;
                        range_checks.push(quote! {
                            if self.#field_name < #start || self.#field_name > #end {
                                Err(format!("Invalid range for field {}.{}", stringify!(#struct_name), stringify!(#field_name)))
                            }
                        });
                    }
                    if let Some(regex) = &is.pattern {
                        let pattern = &regex;
                        regex_checks.push(quote! {
                            let re = regex::Regex::new(#pattern).unwrap();
                            if !re.is_match(&self.#field_name) {
                                Err(format!("Invalid regex for field {}.{}" ,stringify!(#struct_name), stringify!(#field_name)))
                            }
                        });
                    }
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
    quote! {
        pub fn is_valid(&self) -> Result<bool, String> {
            #(#range_checks)*
            #(#regex_checks)*
            Ok(true)
        }
    }
}
