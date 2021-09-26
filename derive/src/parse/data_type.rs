use crate::{Error, Result};
use proc_macro2::{Ident, Span, TokenTree};
use std::iter::Peekable;

#[derive(Debug, Clone)]
pub enum DataType {
    Enum(Ident),
    Struct(Ident),
}

impl DataType {
    pub fn take(input: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<Self> {
        if let Some(TokenTree::Ident(ident)) = input.peek() {
            let result = match ident.to_string().as_str() {
                "struct" => DataType::Struct,
                "enum" => DataType::Enum,
                _ => return Err(Error::UnknownDataType(ident.span())),
            };
            let ident = super::assume_ident(input.next());
            return match input.next() {
                Some(TokenTree::Ident(ident)) => Ok((result)(ident)),
                Some(t) => Err(Error::InvalidRustSyntax(t.span())),
                None => Err(Error::InvalidRustSyntax(ident.span())),
            };
        }
        let span = input
            .peek()
            .map(|t| t.span())
            .unwrap_or_else(Span::call_site);
        Err(Error::InvalidRustSyntax(span))
    }

    // pub fn ident(&self) -> String {
    //     match self {
    //         Self::Enum(ident) => ident,
    //         Self::Struct(ident) => ident,
    //     }
    //     .to_string()
    // }
}

#[cfg(test)]
impl DataType {
    pub fn is_enum(&self, ident: &str) -> bool {
        if let Self::Enum(i) = self {
            i.to_string() == ident
        } else {
            false
        }
    }
    pub fn is_struct(&self, ident: &str) -> bool {
        if let Self::Struct(i) = self {
            i.to_string() == ident
        } else {
            false
        }
    }
}

#[test]
fn test_datatype_take() {
    use crate::token_stream;

    assert!(DataType::take(&mut token_stream("enum"))
        .unwrap_err()
        .is_invalid_rust_syntax());
    assert!(DataType::take(&mut token_stream("enum Foo"))
        .unwrap()
        .is_enum("Foo"));
    assert!(DataType::take(&mut token_stream("enum Foo { }"))
        .unwrap()
        .is_enum("Foo"));
    assert!(DataType::take(&mut token_stream("enum Foo { bar, baz }"))
        .unwrap()
        .is_enum("Foo"));
    assert!(
        DataType::take(&mut token_stream("enum Foo<'a, T> { bar, baz }"))
            .unwrap()
            .is_enum("Foo")
    );

    assert!(DataType::take(&mut token_stream("struct"))
        .unwrap_err()
        .is_invalid_rust_syntax());
    assert!(DataType::take(&mut token_stream("struct Foo { }"))
        .unwrap()
        .is_struct("Foo"));
    assert!(
        DataType::take(&mut token_stream("struct Foo { bar: u32, baz: u32 }"))
            .unwrap()
            .is_struct("Foo")
    );
    assert!(
        DataType::take(&mut token_stream("struct Foo<'a, T> { bar: &'a T }"))
            .unwrap()
            .is_struct("Foo")
    );

    assert!(DataType::take(&mut token_stream("fn foo() {}"))
        .unwrap_err()
        .is_unknown_data_type());

    assert!(DataType::take(&mut token_stream("() {}"))
        .unwrap_err()
        .is_invalid_rust_syntax());

    assert!(DataType::take(&mut token_stream(""))
        .unwrap_err()
        .is_invalid_rust_syntax());
}
