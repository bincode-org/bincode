use crate::Result;
use proc_macro::TokenStream;
use syn::Ident;

pub struct DeriveEnum {}

impl DeriveEnum {
    pub fn parse(_name: Ident, _en: syn::DataEnum) -> Result<Self> {
        unimplemented!()
    }

    pub fn to_encodable(self) -> Result<TokenStream> {
        unimplemented!()
    }

    pub fn to_decodable(self) -> Result<TokenStream> {
        unimplemented!()
    }
}
