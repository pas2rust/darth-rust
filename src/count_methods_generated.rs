pub fn count_methods_generated(token_stream: &proc_macro2::TokenStream) -> usize {
    let mut method_count = 0;
    let tokens = token_stream.clone().into_iter();

    for token in tokens {
        if let proc_macro2::TokenTree::Group(group) = token {
            let content = group.stream();
            let content_tokens = content.clone().into_iter();

            for content_token in content_tokens {
                if let proc_macro2::TokenTree::Ident(ident) = content_token {
                    let ident_str = ident.to_string();

                    // Verifique se o identificador começa com "fn" para identificar métodos
                    if ident_str == "fn" {
                        method_count += 1;
                    }
                }
            }
        }
    }

    method_count
}
