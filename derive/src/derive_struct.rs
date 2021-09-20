use crate::Result;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Generics, Ident};

pub struct DeriveStruct {
    name: Ident,
    generics: Generics,
    fields: Vec<Ident>,
}

impl DeriveStruct {
    pub fn parse(name: Ident, generics: Generics, str: syn::DataStruct) -> Result<Self> {
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
        Ok(Self {
            name,
            generics,
            fields,
        })
    }

    pub fn to_encodable(self) -> Result<TokenStream> {
        let DeriveStruct {
            name,
            generics,
            fields,
        } = self;

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let fields = fields
            .into_iter()
            .map(|field| {
                quote! {
                    bincode::enc::Encodeable::encode(&self. #field, &mut encoder)?;
                }
            })
            .collect::<Vec<_>>();

        let result = quote! {
            impl #impl_generics bincode::enc::Encodeable for #name #ty_generics #where_clause {
                fn encode<E: bincode::enc::Encode>(&self, mut encoder: E) -> Result<(), bincode::error::EncodeError> {
                    #(#fields)*
                    Ok(())
                }

            }
        };
        Ok(result.into())
    }

    pub fn to_decodable(self) -> Result<TokenStream> {
        let DeriveStruct {
            name,
            generics,
            fields,
        } = self;

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let fields = fields
            .into_iter()
            .map(|field| {
                quote! {
                    #field: bincode::de::Decodable::decode(&mut decoder)?,
                }
            })
            .collect::<Vec<_>>();

        let result = quote! {
            impl #impl_generics bincode::de::Decodable for #name #ty_generics #where_clause {
                fn decode<D: bincode::de::Decode>(mut decoder: D) -> Result<#name #ty_generics, bincode::error::DecodeError> {
                    Ok(#name {
                        #(#fields)*
                    })
                }

            }
        };
        Ok(result.into())
    }
}
