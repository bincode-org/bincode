extern crate proc_macro;

// mod derive_enum;
// mod derive_struct;
mod error;
mod parse;

#[cfg(test)]
pub(crate) mod prelude {
    pub use proc_macro2::*;
}
#[cfg(not(test))]
pub(crate) mod prelude {
    pub use proc_macro::*;
}

use error::Error;
use prelude::TokenStream;

type Result<T = ()> = std::result::Result<T, Error>;

#[proc_macro_derive(Encodable)]
pub fn derive_encodable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    #[allow(clippy::useless_conversion)]
    derive_encodable_inner(input.into())
        .unwrap_or_else(|e| e.into_token_stream())
        .into()
}

fn derive_encodable_inner(input: TokenStream) -> Result<TokenStream> {
    let mut source = input.into_iter().peekable();
    let source = &mut source;
    let _visibility = parse::Visibility::try_take(source)?;
    let datatype = parse::DataType::take(source)?;
    let _generics = parse::Generics::try_take(source)?;
    let _where = parse::GenericConstraints::try_take(source)?;

    dbg!(&_visibility);
    dbg!(&datatype);
    dbg!(&_generics);
    dbg!(&_where);

    match datatype {
        parse::DataType::Struct(_name) => {
            let body = parse::StructBody::take(source)?;
            dbg!(&body);
        }
        parse::DataType::Enum(_name) => {
            let body = parse::EnumBody::take(source)?;
            dbg!(&body);
        }
    }

    unimplemented!();
}

/*
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
*/

#[cfg(test)]
pub(crate) fn token_stream(
    s: &str,
) -> std::iter::Peekable<impl Iterator<Item = proc_macro2::TokenTree>> {
    use std::str::FromStr;

    let stream = proc_macro2::TokenStream::from_str(s)
        .unwrap_or_else(|e| panic!("Could not parse code: {:?}\n{:?}", s, e));
    stream.into_iter().peekable()
}
