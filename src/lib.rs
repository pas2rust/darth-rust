mod count_methods_generated;
mod crates;
mod generate_count_message;
mod generate_default_method;
mod generate_from_json_method;
mod generate_from_toml_method;
mod generate_from_xml_method;
mod generate_from_yaml_method;
mod generate_getters;
mod generate_mut_getters;
mod generate_new_method;
mod generate_printers;
mod generate_printers_by_field;
mod generate_printers_err_by_field;
mod generate_printers_info_by_field;
mod generate_printers_rust_by_field;
mod generate_printers_success_by_field;
mod generate_printers_warning_by_field;
mod generate_setters;
mod tools;
mod generate_to_csv_method;
mod generate_to_json_method;
mod generate_to_toml_method;
mod generate_to_xml_method;
mod generate_to_yaml_method;
//mod generate_is_by_field;
use crates::*;

///
/// # Usage
///
/// ```rust
/// use darth_rust::DarthRust;
/// #[derive(DarthRust, Debug)]
/// struct MyStruct {
///     field1: String,
///     field2: i32,
/// }
/// ```
#[proc_macro_derive(DarthRust)]
pub fn darth_rust(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let mut_getters = generate_mut_getters(&input.data);
    let getters = generate_getters(&input.data);
    let setters = generate_setters(&input.data);
    let new = generate_new_method(&input.data);
    let default = generate_default_method(&input.data);
    let to_json = generate_to_json_method(&input);
    let from_json = generate_from_json_method(&input);
    let from_yaml = generate_from_yaml_method(&input);
    let from_xml = generate_from_xml_method(&input);
    let from_toml = generate_from_toml_method(&input);
    let to_yaml = generate_to_yaml_method(&input);
    let to_xml = generate_to_xml_method(&input);
    let to_csv = generate_to_csv_method(&input);
    let to_toml = generate_to_toml_method(&input);
    let printers = generate_printers(&input);
    //let is = generate_is_by_field(&input.data);
    let printers_success_by_field = generate_printers_success_by_field(&input);
    let printers_by_field = generate_printers_by_field(&input);
    let printters_rust_by_field = generate_printers_rust_by_field(&input);
    let printers_info_by_field = generate_printers_info_by_field(&input);
    let printers_err_by_field = generate_printers_err_by_field(&input);
    let printers_warning_by_field = generate_printers_warning_by_field(&input);
    let expanded = quote! {
        impl #struct_name {
            #mut_getters
            #from_toml
            //#is
            #from_json
            #from_yaml
            #from_xml
            #to_json
            #to_csv
            #to_toml
            #to_yaml
            #to_xml
            #new
            #default
            #getters
            #setters
            #printers
            #printers_success_by_field
            #printers_by_field
            #printters_rust_by_field
            #printers_info_by_field
            #printers_warning_by_field
            #printers_err_by_field
        }
    };

    generate_count_message(struct_name, &expanded);
    expanded.into()
}
