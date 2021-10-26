use crate::prelude::{Ident, TokenTree};
use crate::{Error, Result};
use std::iter::Peekable;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataType {
    Enum,
    Struct,
}

impl DataType {
    pub fn take(input: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<(Self, Ident)> {
        if let Some(TokenTree::Ident(_)) = input.peek() {
            let ident = super::assume_ident(input.next());
            let result = match ident.to_string().as_str() {
                "struct" => DataType::Struct,
                "enum" => DataType::Enum,
                _ => return Err(Error::UnknownDataType(ident.span())),
            };
            return match input.next() {
                Some(TokenTree::Ident(ident)) => Ok((result, ident)),
                token => Error::wrong_token(token.as_ref(), "ident"),
            };
        }
        Error::wrong_token(input.peek(), "ident")
    }
}

#[test]
fn test_datatype_take() {
    use crate::token_stream;

    fn validate_output_eq(input: &str, expected_dt: DataType, expected_ident: &str) {
        let (dt, ident) = DataType::take(&mut token_stream(input)).unwrap_or_else(|e| {
            panic!("Could not parse tokenstream {:?}: {:?}", input, e);
        });
        if dt != expected_dt || ident != expected_ident {
            println!("While parsing {:?}", input);
            panic!(
                "Expected {:?} {:?}, received {:?} {:?}",
                dt, ident, expected_dt, expected_ident
            );
        }
    }

    assert!(DataType::take(&mut token_stream("enum"))
        .unwrap_err()
        .is_invalid_rust_syntax());
    validate_output_eq("enum Foo", DataType::Enum, "Foo");
    validate_output_eq("enum Foo { }", DataType::Enum, "Foo");
    validate_output_eq("enum Foo { bar, baz }", DataType::Enum, "Foo");
    validate_output_eq("enum Foo<'a, T> { bar, baz }", DataType::Enum, "Foo");

    assert!(DataType::take(&mut token_stream("struct"))
        .unwrap_err()
        .is_invalid_rust_syntax());
    validate_output_eq("struct Foo { }", DataType::Struct, "Foo");
    validate_output_eq("struct Foo { bar: u32, baz: u32 }", DataType::Struct, "Foo");
    validate_output_eq("struct Foo<'a, T> { bar: &'a T }", DataType::Struct, "Foo");

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
