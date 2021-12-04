mod derive_enum;
mod derive_struct;

use virtue::prelude::*;

#[proc_macro_derive(Encode, attributes(bincode))]
pub fn derive_encode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_encode_inner(input).unwrap_or_else(|e| e.into_token_stream())
}

fn derive_encode_inner(input: TokenStream) -> Result<TokenStream> {
    let parse = Parse::new(input)?;
    let (mut generator, body) = parse.into_generator();

    match body {
        Body::Struct(body) => {
            derive_struct::DeriveStruct {
                fields: body.fields,
            }
            .generate_encode(&mut generator)?;
        }
        Body::Enum(body) => {
            derive_enum::DeriveEnum {
                variants: body.variants,
            }
            .generate_encode(&mut generator)?;
        }
    }

    let name = generator.target_name().clone();
    let stream = generator.finish()?;
    dump_output(name, "Encode", &stream);
    Ok(stream)
}

#[proc_macro_derive(Decode, attributes(bincode))]
pub fn derive_decode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_decode_inner(input).unwrap_or_else(|e| e.into_token_stream())
}

fn derive_decode_inner(input: TokenStream) -> Result<TokenStream> {
    let parse = Parse::new(input)?;
    let (mut generator, body) = parse.into_generator();

    match body {
        Body::Struct(body) => {
            derive_struct::DeriveStruct {
                fields: body.fields,
            }
            .generate_decode(&mut generator)?;
        }
        Body::Enum(body) => {
            derive_enum::DeriveEnum {
                variants: body.variants,
            }
            .generate_decode(&mut generator)?;
        }
    }

    let name = generator.target_name().clone();
    let stream = generator.finish()?;
    dump_output(name, "Decode", &stream);
    Ok(stream)
}

#[proc_macro_derive(BorrowDecode, attributes(bincode))]
pub fn derive_brrow_decode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_borrow_decode_inner(input).unwrap_or_else(|e| e.into_token_stream())
}

fn derive_borrow_decode_inner(input: TokenStream) -> Result<TokenStream> {
    let parse = Parse::new(input)?;
    let (mut generator, body) = parse.into_generator();

    match body {
        Body::Struct(body) => {
            derive_struct::DeriveStruct {
                fields: body.fields,
            }
            .generate_borrow_decode(&mut generator)?;
        }
        Body::Enum(body) => {
            derive_enum::DeriveEnum {
                variants: body.variants,
            }
            .generate_borrow_decode(&mut generator)?;
        }
    }

    let name = generator.target_name().clone();
    let stream = generator.finish()?;
    dump_output(name, "BorrowDecode", &stream);
    Ok(stream)
}

fn dump_output(name: Ident, derive: &str, stream: &TokenStream) {
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

#[derive(Debug, Copy, Clone, PartialEq)]
enum FieldAttribute {
    WithSerde,
}

impl FromAttribute for FieldAttribute {
    fn parse(group: &Group) -> Option<Self> {
        let stream = &mut group.stream().into_iter();
        let mut tmp_stream;
        let stream = if let Some(TokenTree::Ident(attribute_ident)) = stream.next() {
            if attribute_ident.to_string() != "bincode" {
                return None;
            }
            if let Some(TokenTree::Group(group)) = stream.next() {
                tmp_stream = group.stream().into_iter().peekable();
                &mut tmp_stream
            } else {
                return None;
            }
        } else {
            return None;
        };

        if let Some(TokenTree::Ident(ident)) = stream.next() {
            if ident.to_string() == "with_serde" {
                Some(Self::WithSerde)
            } else {
                None
            }
        } else {
            None
        }
    }
}
