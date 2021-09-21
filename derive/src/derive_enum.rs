use crate::Result;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use quote::ToTokens;
use syn::{spanned::Spanned, Fields, Generics, Ident, Index, Variant};
pub struct DeriveEnum {
    name: Ident,
    generics: Generics,
    variants: Vec<Variant>,
}

impl DeriveEnum {
    pub fn parse(name: Ident, generics: Generics, en: syn::DataEnum) -> Result<Self> {
        let variants = en.variants.into_iter().collect();

        Ok(DeriveEnum {
            name,
            generics,
            variants,
        })
    }

    pub fn generate_encodable(self) -> Result<TokenStream> {
        let DeriveEnum {
            name,
            generics,
            variants,
        } = self;

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let match_arms = variants.iter().enumerate().map(|(index, variant)| {
            let fields_section = fields_to_match_arm(&variant.fields);
            let encode_statements = field_names_to_encodable(&fields_to_names(&variant.fields));
            let variant_name = variant.ident.clone();
            quote! {
                #name :: #variant_name #fields_section => {
                    encoder.encode_u32(#index as u32)?;
                    #(#encode_statements)*
                }
            }
        });
        let result = quote! {
            impl #impl_generics bincode::enc::Encodeable for #name #ty_generics #where_clause {
                fn encode<E: bincode::enc::Encode>(&self, mut encoder: E) -> Result<(), bincode::error::EncodeError> {
                    match self {
                        #(#match_arms)*
                    }
                    Ok(())
                }

            }
        };

        Ok(result.into())
    }

    pub fn generate_decodable(self) -> Result<TokenStream> {
        let DeriveEnum {
            name,
            generics,
            variants,
        } = self;

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        let max_variant = (variants.len() - 1) as u32;
        let match_arms = variants.iter().enumerate().map(|(index, variant)| {
            let index = index as u32;
            let decode_statements =
                field_names_to_decodable(&fields_to_constructable_names(&variant.fields));
            let variant_name = variant.ident.clone();
            quote! {
                #index => {
                    #name :: #variant_name {
                        #(#decode_statements)*
                    }
                }
            }
        });
        let result = quote! {
            impl #impl_generics bincode::de::Decodable for #name #ty_generics #where_clause {
                fn decode<D: bincode::de::Decode>(mut decoder: D) -> Result<#name #ty_generics, bincode::error::DecodeError> {
                    let i = decoder.decode_u32()?;
                    Ok(match i {
                        #(#match_arms)*
                        variant => return Err(bincode::error::DecodeError::UnexpectedVariant{
                            min: 0,
                            max: #max_variant,
                            found: variant,
                        })
                    })
                }

            }
        };

        Ok(result.into())
    }
}

fn fields_to_match_arm(fields: &Fields) -> TokenStream2 {
    match fields {
        syn::Fields::Named(fields) => {
            let fields: Vec<_> = fields
                .named
                .iter()
                .map(|f| f.ident.clone().unwrap().to_token_stream())
                .collect();
            quote! {
                {#(#fields),*}
            }
        }
        syn::Fields::Unnamed(fields) => {
            let fields: Vec<_> = fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, f)| Ident::new(&format!("_{}", i), f.span()))
                .collect();
            quote! {
                (#(#fields),*)
            }
        }
        syn::Fields::Unit => quote! {},
    }
}

fn fields_to_names(fields: &Fields) -> Vec<TokenStream2> {
    match fields {
        syn::Fields::Named(fields) => fields
            .named
            .iter()
            .map(|f| f.ident.clone().unwrap().to_token_stream())
            .collect(),
        syn::Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, f)| Ident::new(&format!("_{}", i), f.span()).to_token_stream())
            .collect(),
        syn::Fields::Unit => Vec::new(),
    }
}

fn field_names_to_encodable(names: &[TokenStream2]) -> Vec<TokenStream2> {
    names
        .iter()
        .map(|field| {
            quote! {
                bincode::enc::Encodeable::encode(#field, &mut encoder)?;
            }
        })
        .collect::<Vec<_>>()
}

fn fields_to_constructable_names(fields: &Fields) -> Vec<TokenStream2> {
    match fields {
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
    }
}

fn field_names_to_decodable(names: &[TokenStream2]) -> Vec<TokenStream2> {
    names
        .iter()
        .map(|field| {
            quote! {
                #field: bincode::de::Decodable::decode(&mut decoder)?,
            }
        })
        .collect::<Vec<_>>()
}
