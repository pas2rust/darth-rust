use proc_macro2::Ident;
use syn::{Data, DataStruct, Fields, FieldsNamed};

pub struct Helpers {
    pub data: Data,
}

pub trait HelpersTrait {
    fn get_named_fields(&self) -> Result<&FieldsNamed, &str>;
    fn get_data_struct(&self) -> Result<&DataStruct, &str>;
    fn get_fields(&self) -> Result<&Fields, &str>;
    fn new(data: Data) -> Self;
    fn new_ident(prefix: &str, field_name: Ident) -> Ident;
}

impl HelpersTrait for Helpers {
    fn new(data: Data) -> Self {
        Self { data }
    }
    fn get_named_fields(&self) -> Result<&FieldsNamed, &str> {
        match &self.data {
            Data::Struct(data_struct) => match &data_struct.fields {
                Fields::Named(named_fields) => Ok(named_fields),
                _ => Err("Fields are not named"),
            },
            _ => Err("Data is not a struct"),
        }
    }
    fn get_fields(&self) -> Result<&Fields, &str> {
        match &self.data {
            Data::Struct(data_struct) => Ok(&data_struct.fields),
            _ => Err("from_json can only be derived for structs"),
        }
    }
    fn get_data_struct(&self) -> Result<&DataStruct, &str> {
        match &self.data {
            Data::Struct(data_struct) => Ok(data_struct),
            _ => Err("Data is not a struct"),
        }
    }
    fn new_ident(prefix: &str, field_name: Ident) -> Ident {
        Ident::new(
            format!("{}_{}", prefix, field_name).as_str(),
            field_name.span(),
        )
    }
}
