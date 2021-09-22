use crate::Result;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{GenericParam, Generics, Ident, Index, Lifetime, LifetimeDef};

pub struct DeriveStruct {
    name: Ident,
    generics: Generics,
    fields: Vec<TokenStream2>,
}

impl DeriveStruct {
    pub fn parse(name: Ident, generics: Generics, str: syn::DataStruct) -> Result<Self> {
        let fields = match str.fields {
            syn::Fields::Named(fields) => fields
                .named
                .iter()
                .map(|f| f.ident.clone().unwrap().to_token_stream())
                .collect(),
            syn::Fields::Unnamed(fields) => fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, _)| Index::from(i).to_token_stream())
                .collect(),
            syn::Fields::Unit => Vec::new(),
        };
        Ok(Self {
            name,
            generics,
            fields,
        })
    }

    pub fn generate_encodable(self) -> Result<TokenStream> {
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

    pub fn generate_decodable(self) -> Result<TokenStream> {
        let DeriveStruct {
            name,
            generics,
            fields,
        } = self;

        let (mut impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        // check if we don't already have a '__de lifetime
        let mut should_insert_lifetime = true;

        for param in &generics.params {
            if let GenericParam::Lifetime(lt) = param {
                if lt.lifetime.ident == "__de" {
                    should_insert_lifetime = false;
                    break;
                }
            }
        }

        // if we don't have a '__de lifetime, insert it
        let mut generics_with_decode_lifetime;
        if should_insert_lifetime {
            generics_with_decode_lifetime = generics.clone();
            generics_with_decode_lifetime
                .params
                .push(GenericParam::Lifetime(LifetimeDef::new(Lifetime::new(
                    "'__de",
                    Span::call_site(),
                ))));

            impl_generics = generics_with_decode_lifetime.split_for_impl().0;
        }

        let fields = fields
            .into_iter()
            .map(|field| {
                quote! {
                    #field: bincode::de::Decodable::decode(&mut decoder)?,
                }
            })
            .collect::<Vec<_>>();

        let result = quote! {
            impl #impl_generics bincode::de::Decodable< '__de > for #name #ty_generics #where_clause {
                fn decode<D: bincode::de::Decode< '__de >>(mut decoder: D) -> Result<Self, bincode::error::DecodeError> {
                    Ok(#name {
                        #(#fields)*
                    })
                }

            }
        };

        Ok(result.into())
    }
}
