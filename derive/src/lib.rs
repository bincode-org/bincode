extern crate proc_macro;

mod derive_enum;
mod derive_struct;
mod error;

use derive_enum::DeriveEnum;
use derive_struct::DeriveStruct;
use error::Error;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

type Result<T = ()> = std::result::Result<T, Error>;

#[proc_macro_derive(Encodable)]
pub fn derive_encodable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_encodable_inner(input).unwrap_or_else(|e| e.into_token_stream())
}

fn derive_encodable_inner(input: DeriveInput) -> Result<TokenStream> {
    match input.data {
        syn::Data::Struct(struct_definition) => {
            DeriveStruct::parse(input.ident, input.generics, struct_definition)
                .and_then(|str| str.generate_encodable())
        }
        syn::Data::Enum(enum_definition) => {
            DeriveEnum::parse(input.ident, input.generics, enum_definition)
                .and_then(|str| str.generate_encodable())
        }
        syn::Data::Union(_) => Err(Error::UnionNotSupported),
    }
}

#[proc_macro_derive(Decodable)]
pub fn derive_decodable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_decodable_inner(input).unwrap_or_else(|e| e.into_token_stream())
}

fn derive_decodable_inner(input: DeriveInput) -> Result<TokenStream> {
    match input.data {
        syn::Data::Struct(struct_definition) => {
            DeriveStruct::parse(input.ident, input.generics, struct_definition)
                .and_then(|str| str.generate_decodable())
        }
        syn::Data::Enum(enum_definition) => {
            DeriveEnum::parse(input.ident, input.generics, enum_definition)
                .and_then(|str| str.generate_decodable())
        }
        syn::Data::Union(_) => Err(Error::UnionNotSupported),
    }
}
