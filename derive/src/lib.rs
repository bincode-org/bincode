extern crate proc_macro;

// mod derive_enum;
mod derive_struct;
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
    let source = &mut input.into_iter().peekable();

    let _visibility = parse::Visibility::try_take(source)?;
    let datatype = parse::DataType::take(source)?;
    let generics = parse::Generics::try_take(source)?;
    let generic_constraints = parse::GenericConstraints::try_take(source)?;

    match datatype {
        parse::DataType::Struct(name) => {
            let body = parse::StructBody::take(source)?;
            let stream = derive_struct::DeriveStruct {
                name: name.clone(),
                generics,
                generic_constraints,
                fields: body.fields,
            }
            .generate_encodable();

            dump_output(name, "Encodeable", &stream);

            stream
        }
        parse::DataType::Enum(_name) => {
            let body = parse::EnumBody::take(source)?;
            dbg!(&body);

            unimplemented!();
        }
    }
}

fn dump_output(
    name: crate::prelude::Ident,
    derive: &str,
    stream: &Result<crate::prelude::TokenStream>,
) {
    use std::io::Write;
    if let Ok(stream) = stream {
        if let Ok(var) = std::env::var("CARGO_MANIFEST_DIR") {
            let mut path = std::path::PathBuf::from(var);
            path.push("target");
            if path.exists() {
                path.push(format!("{}_{}.rs", name, derive));
                if let Ok(mut file) = std::fs::File::create(path) {
                    let _ = file.write_all(stream.to_string().as_bytes());
                }
            }
        }
    }
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
