use quote::quote;
use syn::{Data, Fields, Ident};

#[derive(Debug, FromMeta)]
struct Range {
    start: Lit,
    end: Lit,
}

#[derive(Debug, FromMeta)]
struct Regex {
    #[darling(default)]
    pattern: Option<Lit>,
}

pub fn generate_validator_method(input: &DeriveInput) -> proc_macro2::TokenStream {
    let mut validations = Vec::new();
    if let Fields::Named(fields) = &input.data.fields {
        for field in &fields.named {
            let field_name = field.ident.as_ref().unwrap();
            let mut has_range = false;
            let mut has_regex = false;
            &field.attrs.into_iter().for_each(|attr| {
                if let Ok(meta) = attr.parse_meta() {
                    if let Some(range) = Range::from_meta(&meta) {
                        has_range = true;
                        let start = &range.start;
                        let end = &range.end;
                        validations.push(quote! {
                            if self.#field_name < #start || self.#field_name > #end {
                                // Adicione lógica de erro para o atributo range aqui
                            }
                        });
                    } else if let Some(regex) = Regex::from_meta(&meta) {
                        has_regex = true;
                        if let Some(pattern) = regex.pattern {
                            validations.push(quote! {
                                let regex = regex::Regex::new(#pattern).expect("Failed to create regex");
                                if !regex.is_match(&self.#field_name) {
                                    // Adicione lógica de erro para o atributo regex aqui
                                }
                            });
                        }
                    }
                }
            });

            if !has_range {
                // Adicione lógica para o caso em que nenhum atributo `range` foi encontrado
            }

            if !has_regex {
                // Adicione lógica para o caso em que nenhum atributo `regex` foi encontrado
            }
        }
    }
}
