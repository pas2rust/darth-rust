use quote::quote;
use syn::{Data, DeriveInput, Error};

pub fn generate_from_xml_method(input: &DeriveInput) -> proc_macro2::TokenStream {
    let struct_name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data_struct) => &data_struct.fields,
        _ => {
            return Error::new_spanned(&input, "from_xml can only be derived for structs")
                .to_compile_error()
                .into();
        }
    };

    let from_xml_code = {
        let field_deserialization = fields.iter().map(|field| {
            let field_name = &field.ident;
            quote! {
                #field_name: {
                    let field_start_tag = format!("<{}>", stringify!(#field_name));
                    let field_end_tag = format!("</{}>", stringify!(#field_name));
                    let start_pos = xml_string.find(&field_start_tag);
                    if let Some(start_pos) = start_pos {
                        let start_pos = start_pos + field_start_tag.len();
                        if let Some(end_pos) = xml_string[start_pos..].find(&field_end_tag) {
                            let field_value = &xml_string[start_pos..start_pos + end_pos];
                            field_value.to_string() // Corrected: Convert &str to String
                        } else {
                            Default::default()
                        }
                    } else {
                        Default::default()
                    }
                }
            }
        });

        quote! {
            #(#field_deserialization,)*
        }
    };

    quote! {
        // ... (your doc comments)

        pub fn from_xml(xml_string: &str) -> Self {
            Self {
                #from_xml_code
            }
        }
    }
}
