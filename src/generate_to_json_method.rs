use quote::quote;
use syn::{Data, DeriveInput, Error};

pub fn generate_to_json_method(
    input: &DeriveInput,
) -> proc_macro2::TokenStream {
    let fields = match &input.data {
        Data::Struct(data_struct) => &data_struct.fields,
        _ => {
            return Error::new_spanned(
                input,
                "to_json can only be derived for structs",
            )
            .to_compile_error()
        }
    };
    let json_object = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! {
            stringify!(#field_name): self.#field_name,
        }
    });

    quote! {
        pub fn to_json(&self) -> serde_json::Value {
            serde_json::json!({
                #(#json_object)*
            })
        }
    }
}
