use super::assume_group;
use crate::parse::consume_punct_if;
use crate::prelude::{Delimiter, Group, Punct, TokenTree};
use crate::{Error, Result};
use std::iter::Peekable;

#[derive(Debug)]
pub struct Attributes {
    // we don't use these fields yet
    #[allow(dead_code)]
    punct: Punct,
    #[allow(dead_code)]
    tokens: Group,
}

impl Attributes {
    pub fn try_take(input: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<Option<Self>> {
        if let Some(punct) = consume_punct_if(input, '#') {
            // found attributes, next token should be a [] group
            if let Some(TokenTree::Group(g)) = input.peek() {
                if g.delimiter() != Delimiter::Bracket {
                    return Err(Error::InvalidRustSyntax(g.span()));
                }
                return Ok(Some(Attributes {
                    punct,
                    tokens: assume_group(input.next()),
                }));
            }
            // expected [] group, found something else
            return Err(Error::InvalidRustSyntax(match input.peek() {
                Some(next_token) => next_token.span(),
                None => punct.span(),
            }));
        }
        Ok(None)
    }
}

#[test]
fn test_attributes_try_take() {
    use crate::token_stream;

    let stream = &mut token_stream("struct Foo;");
    assert!(Attributes::try_take(stream).unwrap().is_none());
    match stream.next().unwrap() {
        TokenTree::Ident(i) => assert_eq!(i, "struct"),
        x => panic!("Expected ident, found {:?}", x),
    }

    let stream = &mut token_stream("#[cfg(test)] struct Foo;");
    assert!(Attributes::try_take(stream).unwrap().is_some());
    match stream.next().unwrap() {
        TokenTree::Ident(i) => assert_eq!(i, "struct"),
        x => panic!("Expected ident, found {:?}", x),
    }
}
