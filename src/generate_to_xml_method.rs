use quote::quote;
use syn::{Data, DeriveInput};

pub fn generate_to_xml_method(input: &DeriveInput) -> proc_macro2::TokenStream {
    let struct_name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data_struct) => &data_struct.fields,
        _ => {
            return quote! {
                compile_error!("to_xml can only be derived for structs");
            };
        }
    };

    let field_mappings = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        quote! {
            let field_xml = serde_xml_rs::to_string(&self.#field_name).expect("Failed to serialize field to XML");
            xml.push_str(&format!("<{}>{}</{}>", stringify!(#field_name), field_xml, stringify!(#field_name)));
        }
    });

    quote! {
        pub fn to_xml(&self) -> String {
            let mut xml = String::new();
            xml.push_str(&format!("<{}>", stringify!(#struct_name)));
            #(#field_mappings)*
            xml.push_str(&format!("</{}>", stringify!(#struct_name)));
            xml
        }
    }
}
