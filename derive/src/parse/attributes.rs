use super::{assume_group, assume_punct};
use crate::parse::consume_punct_if;
use crate::prelude::{Delimiter, Group, Punct, TokenTree};
use crate::{Error, Result};
use std::iter::Peekable;

#[derive(Debug)]
pub struct Attribute {
    // we don't use these fields yet
    #[allow(dead_code)]
    punct: Punct,
    #[allow(dead_code)]
    tokens: Option<Group>,
}

impl Attribute {
    pub fn try_take(input: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<Vec<Self>> {
        let mut result = Vec::new();

        while let Some(punct) = consume_punct_if(input, '#') {
            match input.peek() {
                Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Bracket => {
                    result.push(Attribute {
                        punct,
                        tokens: Some(assume_group(input.next())),
                    });
                }
                Some(TokenTree::Group(g)) => {
                    return Err(Error::InvalidRustSyntax {
                        span: g.span(),
                        expected: format!("[] bracket, got {:?}", g.delimiter()),
                    });
                }
                Some(TokenTree::Punct(p)) if p.as_char() == '#' => {
                    // sometimes with empty lines of doc comments, we get two #'s in a row
                    // add an empty attributes and continue to the next loop
                    result.push(Attribute {
                        punct: assume_punct(input.next(), '#'),
                        tokens: None,
                    })
                }
                token => return Error::wrong_token(token, "[] group or next # attribute"),
            }
        }
        Ok(result)
    }
}

#[test]
fn test_attributes_try_take() {
    use crate::token_stream;

    let stream = &mut token_stream("struct Foo;");
    assert!(Attribute::try_take(stream).unwrap().is_empty());
    match stream.next().unwrap() {
        TokenTree::Ident(i) => assert_eq!(i, "struct"),
        x => panic!("Expected ident, found {:?}", x),
    }

    let stream = &mut token_stream("#[cfg(test)] struct Foo;");
    assert!(!Attribute::try_take(stream).unwrap().is_empty());
    match stream.next().unwrap() {
        TokenTree::Ident(i) => assert_eq!(i, "struct"),
        x => panic!("Expected ident, found {:?}", x),
    }
}
