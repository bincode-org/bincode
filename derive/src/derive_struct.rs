use crate::Result;
use proc_macro::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, Ident};

pub struct DeriveStruct {
    name: Ident,
    fields: Vec<Ident>,
}

impl DeriveStruct {
    pub fn parse(name: Ident, str: syn::DataStruct) -> Result<Self> {
        let fields = match str.fields {
            syn::Fields::Named(fields) => fields
                .named
                .iter()
                .map(|f| f.ident.clone().unwrap())
                .collect(),
            syn::Fields::Unnamed(fields) => fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, field)| Ident::new(&i.to_string(), field.ty.span()))
                .collect(),
            syn::Fields::Unit => Vec::new(),
        };
        Ok(Self { name, fields })
    }

    pub fn to_encodable(self) -> Result<TokenStream> {
        let DeriveStruct { name, fields } = self;

        let fields = fields
            .into_iter()
            .map(|field| {
                quote! {
                    bincode::enc::Encodeable::encode(&self. #field, &mut encoder)?;
                }
            })
            .collect::<Vec<_>>();

        let result = quote! {
            impl bincode::enc::Encodeable for #name {
                fn encode<E: bincode::enc::Encode>(&self, mut encoder: E) -> Result<(), bincode::error::EncodeError> {
                    #(#fields)*
                    Ok(())
                }

            }
        };
        Ok(result.into())
    }

    pub fn to_decodable(self) -> Result<TokenStream> {
        unimplemented!()
    }
}
