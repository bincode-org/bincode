extern crate proc_macro;

mod derive_enum;
mod derive_struct;
mod error;
mod generate;
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

#[proc_macro_derive(Encode)]
pub fn derive_encode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    #[allow(clippy::useless_conversion)]
    derive_encode_inner(input.into())
        .unwrap_or_else(|e| e.into_token_stream())
        .into()
}

fn derive_encode_inner(input: TokenStream) -> Result<TokenStream> {
    let source = &mut input.into_iter().peekable();

    let _attributes = parse::Attribute::try_take(source)?;
    let _visibility = parse::Visibility::try_take(source)?;
    let (datatype, name) = parse::DataType::take(source)?;
    let generics = parse::Generics::try_take(source)?;
    let generic_constraints = parse::GenericConstraints::try_take(source)?;

    let mut generator = generate::Generator::new(name.clone(), generics, generic_constraints);

    match datatype {
        parse::DataType::Struct => {
            let body = parse::StructBody::take(source)?;
            derive_struct::DeriveStruct {
                fields: body.fields,
            }
            .generate_encode(&mut generator)?;
        }
        parse::DataType::Enum => {
            let body = parse::EnumBody::take(source)?;
            derive_enum::DeriveEnum {
                variants: body.variants,
            }
            .generate_encode(&mut generator)?;
        }
    }

    let stream = generator.take_stream();
    dump_output(name, "Encode", &stream);
    Ok(stream)
}

#[proc_macro_derive(Decode)]
pub fn derive_decode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    #[allow(clippy::useless_conversion)]
    derive_decode_inner(input.into())
        .unwrap_or_else(|e| e.into_token_stream())
        .into()
}

fn derive_decode_inner(input: TokenStream) -> Result<TokenStream> {
    let source = &mut input.into_iter().peekable();

    let _attributes = parse::Attribute::try_take(source)?;
    let _visibility = parse::Visibility::try_take(source)?;
    let (datatype, name) = parse::DataType::take(source)?;
    let generics = parse::Generics::try_take(source)?;
    let generic_constraints = parse::GenericConstraints::try_take(source)?;

    let mut generator = generate::Generator::new(name.clone(), generics, generic_constraints);

    match datatype {
        parse::DataType::Struct => {
            let body = parse::StructBody::take(source)?;
            derive_struct::DeriveStruct {
                fields: body.fields,
            }
            .generate_decode(&mut generator)?;
        }
        parse::DataType::Enum => {
            let body = parse::EnumBody::take(source)?;
            derive_enum::DeriveEnum {
                variants: body.variants,
            }
            .generate_decode(&mut generator)?;
        }
    }

    let stream = generator.take_stream();
    dump_output(name, "Decode", &stream);
    Ok(stream)
}

fn dump_output(name: crate::prelude::Ident, derive: &str, stream: &crate::prelude::TokenStream) {
    use std::io::Write;

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

#[cfg(test)]
pub(crate) fn token_stream(
    s: &str,
) -> std::iter::Peekable<impl Iterator<Item = proc_macro2::TokenTree>> {
    use std::str::FromStr;

    let stream = proc_macro2::TokenStream::from_str(s)
        .unwrap_or_else(|e| panic!("Could not parse code: {:?}\n{:?}", s, e));
    stream.into_iter().peekable()
}
