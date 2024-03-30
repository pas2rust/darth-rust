use proc_macro2::Ident;
use syn::{
    Attribute, Data, DataStruct, DeriveInput, Fields, FieldsNamed, Type,
    TypePath,
};

pub struct Helpers {
    pub input: Option<DeriveInput>,
    pub cache_name: Option<Ident>,
}

pub trait HelpersTrait {
    fn get_named_fields(&self) -> Result<&FieldsNamed, &str>;
    fn get_data_struct(&self) -> Result<&DataStruct, &str>;
    fn get_fields(&self) -> Result<&Fields, &str>;
    fn new() -> Self;
    fn input(self, new: DeriveInput) -> Self;
    fn cache_name(self, new: Ident) -> Self;
    fn new_ident(prefix: &str, field_name: Ident) -> Ident;
    fn new_ident_camel_case(prefix: &str, field_name: Ident) -> Ident;
    fn get_type_path(ty: &Type) -> Result<&TypePath, &str>;
    fn get_attr(self, name: &str) -> Option<Attribute>;
}

impl HelpersTrait for Helpers {
    fn new() -> Self {
        Self { input: None, cache_name: None }
    }
    fn input(mut self, new: DeriveInput) -> Self {
        self.input = new.into();
        self
    }
    fn cache_name(mut self, new: Ident) -> Self {
        self.cache_name = new.into();
        self
    }
    fn get_named_fields(&self) -> Result<&FieldsNamed, &str> {
        match &self.input.as_ref().unwrap().data {
            Data::Struct(data_struct) => match &data_struct.fields {
                Fields::Named(named_fields) => Ok(named_fields),
                _ => Err("Fields are not named"),
            },
            _ => Err("Data is not a struct"),
        }
    }
    fn get_fields(&self) -> Result<&Fields, &str> {
        match &self.input.as_ref().unwrap().data {
            Data::Struct(data_struct) => Ok(&data_struct.fields),
            _ => Err("from_json can only be derived for structs"),
        }
    }
    fn get_data_struct(&self) -> Result<&DataStruct, &str> {
        match &self.input.as_ref().unwrap().data {
            Data::Struct(data_struct) => Ok(data_struct),
            _ => Err("Data is not a struct"),
        }
    }
    fn get_type_path(ty: &Type) -> Result<&TypePath, &str> {
        match ty {
            Type::Path(type_path) => Ok(type_path),
            _ => Err("Type is not a path"),
        }
    }
    fn new_ident(prefix: &str, field_name: Ident) -> Ident {
        Ident::new(
            format!("{}_{}", prefix, field_name).as_str(),
            field_name.span(),
        )
    }
    fn new_ident_camel_case(prefix: &str, field_name: Ident) -> Ident {
        Ident::new(
            format!("{}{}", prefix, field_name).as_str(),
            field_name.span(),
        )
    }
    fn get_attr(self, name: &str) -> Option<Attribute> {
        self.input
            .as_ref()
            .unwrap()
            .attrs
            .iter()
            .find(|a| match &a.meta {
                syn::Meta::List(list) => {
                    list.tokens.clone().into_iter().any(|t| match t {
                        proc_macro2::TokenTree::Ident(i) => i == name,
                        _ => false,
                    })
                }
                _ => false,
            })
            .cloned()
    }
}
