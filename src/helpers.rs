use proc_macro2::Ident;
use syn::{
    parse::Parse, parse_quote, Attribute, Data, DataStruct,
    DeriveInput, Fields, FieldsNamed, GenericParam,
    ImplGenerics, Type, TypeGenerics, TypePath,
    WhereClause,
};

#[derive(Clone)]
pub struct Helpers {
    pub input: Option<DeriveInput>,
    pub cache_name: Option<Ident>,
}

pub trait HelpersTrait {
    fn get_named_fields(
        &self,
    ) -> Result<&FieldsNamed, &str>;
    fn get_data_struct(&self) -> Result<&DataStruct, &str>;
    fn get_fields(&self) -> Result<&Fields, &str>;
    fn new() -> Self;
    fn input(self, new: DeriveInput) -> Self;
    fn cache_name(self, new: Ident) -> Self;
    fn new_ident(prefix: &str, field_name: Ident) -> Ident;
    fn new_ident_camel_case(
        prefix: &str,
        field_name: Ident,
    ) -> Ident;
    fn get_type_path(ty: &Type) -> Result<&TypePath, &str>;
    fn get_attr<T: Parse>(
        attributes: Vec<Attribute>,
        name: &str,
    ) -> Result<T, String>;
    fn add_traits_to_generics(&mut self);
    fn generics_split_for_impl(
        &self,
    ) -> (
        ImplGenerics<'_>,
        TypeGenerics<'_>,
        Option<&WhereClause>,
    );
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
    fn get_named_fields(
        &self,
    ) -> Result<&FieldsNamed, &str> {
        match &self.input.as_ref().unwrap().data {
            Data::Struct(data_struct) => {
                match &data_struct.fields {
                    Fields::Named(named_fields) => {
                        Ok(named_fields)
                    }
                    _ => Err("Fields are not named"),
                }
            }
            _ => Err("Data is not a struct"),
        }
    }
    fn get_fields(&self) -> Result<&Fields, &str> {
        match &self.input.as_ref().unwrap().data {
            Data::Struct(data_struct) => {
                Ok(&data_struct.fields)
            }
            _ => Err(
                "from_json can only be derived for structs",
            ),
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
            Type::Reference(type_reference) => {
                if let Type::Path(type_path) = &*type_reference.elem {
                    Ok(type_path)
                } else {
                    Err("Expected a Type::Path within the Type::Reference")
                }
            }
            Type::Paren(type_paren) => {
                if let Type::Path(type_path) = &*type_paren.elem {
                    Ok(type_path)
                } else {
                    Err("Expected a Type::Path within the Type::Paren")
                }
            }
            Type::Tuple(type_tuple) => {
                for elem in &type_tuple.elems {
                    if let Type::Path(type_path) = elem {
                        return Ok(type_path);
                    }
                }
                Err("No Type::Path found within the Type::Tuple")
            }
            _ => Err("The provided type is not a Type::Path, Type::Reference \
                      containing a Type::Path, or Type::Paren containing a \
                      Type::Path, or Type::Tuple"),
        }
    }
    fn new_ident(prefix: &str, field_name: Ident) -> Ident {
        Ident::new(
            format!("{}_{}", prefix, field_name).as_str(),
            field_name.span(),
        )
    }
    fn new_ident_camel_case(
        prefix: &str,
        field_name: Ident,
    ) -> Ident {
        Ident::new(
            format!("{}{}", prefix, field_name).as_str(),
            field_name.span(),
        )
    }
    fn get_attr<T: Parse>(
        attributes: Vec<Attribute>,
        name: &str,
    ) -> Result<T, String> {
        match attributes
            .iter()
            .find(|attr| attr.path().is_ident(name))
        {
            Some(attr) => Ok(attr.parse_args().unwrap()),
            None => {
                Err(format!("Attribute {} not found", name))
            }
        }
    }
    fn add_traits_to_generics(&mut self) {
        if let Some(ref mut input) = self.input {
            for param in input.generics.params.iter_mut() {
                if let GenericParam::Type(
                    ref mut type_param,
                ) = *param
                {
                    type_param.bounds.push(parse_quote!(
                        ::std::default::Default
                    ));
                    type_param.bounds.push(parse_quote!(
                        ::std::fmt::Debug
                    ));
                    #[cfg(feature = "json")]
                    type_param.bounds.push(parse_quote!(
                        ::serde::Serialize
                    ));
                    #[cfg(feature = "json")]
                    type_param.bounds.push(parse_quote!(
                        for<'de> ::serde::Deserialize<'de>
                    ));
                }
            }
        }
    }
    fn generics_split_for_impl(
        &self,
    ) -> (
        ImplGenerics<'_>,
        TypeGenerics<'_>,
        Option<&WhereClause>,
    ) {
        self.input
            .as_ref()
            .unwrap()
            .generics
            .split_for_impl()
    }
}
