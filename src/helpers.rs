use proc_macro2::Ident;
use quote::quote;
use syn::{
    parse::Parse, parse_str, Attribute, Data, DataStruct,
    DeriveInput, Fields, FieldsNamed, GenericParam,
    ImplGenerics, ItemStruct, Path, Type, TypeGenerics,
    TypePath, WhereClause,
};

#[derive(Clone)]
pub struct Helpers {
    pub input: Option<DeriveInput>,
}

pub trait HelpersTrait {
    fn get_function_signature(
        ty: &syn::Type,
    ) -> Option<(Vec<&syn::Type>, &syn::Type)>;
    fn get_function_args(
        syn_path: &Path,
    ) -> Option<proc_macro2::TokenStream>;
    fn is_function(syn_type: &Type) -> bool;
    fn get_named_fields(
        &self,
    ) -> Result<&FieldsNamed, &str>;
    fn get_item_struct(
        attributes: Vec<Attribute>,
        name: &str,
    ) -> Result<ItemStruct, String>;
    fn get_struct<T: Parse>(
        attributes: Vec<Attribute>,
        name: &str,
    ) -> Result<T, String>;
    fn get_data_struct(&self) -> Result<&DataStruct, &str>;
    fn get_fields(&self) -> Result<&Fields, &str>;
    fn new() -> Self;
    fn input(self, new: DeriveInput) -> Self;
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
    fn get_attr_list<T: Parse>(
        attributes: &mut Vec<Attribute>,
        name: &str,
    ) -> Result<T, syn::Error>;
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
        Self { input: None }
    }
    fn input(mut self, new: DeriveInput) -> Self {
        self.input = new.into();
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
    fn is_function(ty: &Type) -> bool {
        matches!(ty, &Type::BareFn(_))
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
            },
            Type::BareFn(bare_fn) => {
                let return_type = &bare_fn.output;
                match return_type {
                    syn::ReturnType::Type(_, ty) => {
                        if let Type::Path(type_path) = ty.as_ref() {
                            Ok(type_path)
                        } else {
                            Err("Expected a Type::Path within the return type of the BareFn")
                        }
                    },
                    syn::ReturnType::Default => Err("Expected a return type for the BareFn"),
                }
            },
            Type::ImplTrait(_) => Err("The provided type is an impl trait type"),
            Type::Verbatim(_) | Type::Macro(_) => Err("The provided type is a closure or macro type"),
            Type::Array(_) => Err("The provided type is a array"),
            Type::Group(_) => Err("The provided type is a group"),
            Type::Infer(_) => Err("The provided type is a infer"),
            Type::Never(_) => Err("The provided type is a never"),
            Type::Ptr(_) => Err("The provided type is a ptr"),
            Type::Slice(_) => Err("The provided type is a slice"),
            Type::TraitObject(_) => Err("The provided type is a trait object"),
            _ => todo!("empty type")
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
    fn get_attr_list<T: Parse>(
        attributes: &mut Vec<Attribute>,
        name: &str,
    ) -> Result<T, syn::Error> {
        let pos = attributes
            .iter()
            .position(|attr| attr.path().is_ident(name));
        match pos {
            Some(pos) => {
                let attr = attributes.remove(pos);
                attr.parse_args()
            }
            None => Err(syn::Error::new_spanned(
                name,
                format!("Attribute {} not found", name),
            )),
        }
    }
    fn get_item_struct(
        attributes: Vec<Attribute>,
        name: &str,
    ) -> Result<ItemStruct, String> {
        match attributes
            .iter()
            .find(|attr| attr.path().is_ident(name))
        {
            Some(attr) => {
                let attr_tokens = quote! { #attr };
                let attr_string = attr_tokens.to_string();
                match parse_str::<ItemStruct>(&attr_string) {
                    Ok(item_struct) => Ok(item_struct),
                    Err(_) => Err(format!("Could not parse attribute {} as ItemStruct", name)),
                }
            }
            None => {
                Err(format!("Attribute {} not found", name))
            }
        }
    }
    fn get_struct<T: Parse>(
        attributes: Vec<Attribute>,
        name: &str,
    ) -> Result<T, String> {
        match attributes
            .iter()
            .find(|attr| attr.path().is_ident(name))
        {
            Some(attr) => {
                let attr_tokens = quote! { #attr };
                let attr_string = attr_tokens.to_string();
                match parse_str::<T>(&attr_string) {
                    Ok(item) => Ok(item),
                    Err(_) => Err(format!(
                        "Could not parse attribute {}",
                        name
                    )),
                }
            }
            None => {
                Err(format!("Attribute {} not found", name))
            }
        }
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
                    ref mut _type_param,
                ) = *param
                {
                    #[cfg(feature = "build")]
                    _type_param.bounds.push(
                        syn::parse_quote!(
                            ::std::default::Default
                        ),
                    );
                    #[cfg(any(
                        feature = "print",
                        feature = "print_by_field"
                    ))]
                    _type_param.bounds.push(
                        syn::parse_quote!(
                            ::std::fmt::Debug
                        ),
                    );
                    #[cfg(feature = "json")]
                    _type_param.bounds.push(
                        syn::parse_quote!(
                            ::serde::Serialize
                        ),
                    );
                    #[cfg(feature = "json")]
                    _type_param.bounds.push(
                        syn::parse_quote!(
                            for<'de> ::serde::Deserialize<
                                'de,
                            >
                        ),
                    );
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
    fn get_function_args(
        syn_path: &syn::Path,
    ) -> Option<proc_macro2::TokenStream> {
        if let Some(segment) = syn_path.segments.last() {
            if let syn::PathArguments::AngleBracketed(
                args,
            ) = &segment.arguments
            {
                let args_tokens = args
                    .args
                    .iter()
                    .map(|arg| match arg {
                        syn::GenericArgument::Type(ty) => {
                            quote! { #ty }
                        }
                        _ => quote! {},
                    })
                    .collect::<Vec<_>>();
                return Some(quote! { #(#args_tokens),* });
            }
        }
        None
    }
    fn get_function_signature(
        ty: &syn::Type,
    ) -> Option<(Vec<&syn::Type>, &syn::Type)> {
        if let syn::Type::Path(type_path) = ty {
            if let Some(segment) =
                type_path.path.segments.last()
            {
                if segment.ident == "fn" {
                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        let mut args_types = Vec::new();
                        for arg in args.args.iter() {
                            if let syn::GenericArgument::Type(ty) = arg {
                                args_types.push(ty);
                            }
                        }
                        if args_types.is_empty() {
                            return None;
                        }
                        let ret_type = args_types.pop().unwrap();
                        return Some((args_types, ret_type));
                    }
                }
            }
        }
        None
    }
}
