use quote::quote;
use syn::{Data, DeriveInput, Error};

pub fn generate_from_json_method(
    input: &DeriveInput,
) -> proc_macro2::TokenStream {
    let struct_name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data_struct) => &data_struct.fields,
        _ => {
            return Error::new_spanned(
                input,
                "from_json can only be derived for structs",
            )
            .to_compile_error()
        }
    };

    let from_json_code = {
        let field_deserialization = fields.iter().map(|field| {
            let field_name = &field.ident;
            quote! {
                #field_name: match json_object.get(stringify!(#field_name)) {
                    Some(value) => serde_json::from_value(value.clone())
                        .unwrap_or_default(),
                    None => Default::default(),
                },
            }
        });

        quote! {
            #(#field_deserialization)*
        }
    };

    quote! {
        pub fn from_json(json_value: serde_json::Value) -> Result<Self, String> {
            if let serde_json::Value::Object(json_object) = json_value {
                Ok(Self {
                    #from_json_code
                })
            } else {
                Err(format!(
                    "Invalid JSON format for deserialization: JSON must match the structure '{}'.",
                    stringify!(#struct_name)
                ))
            }
        }
    }
}
