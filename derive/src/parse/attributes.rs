use super::{assume_group, assume_ident, assume_punct};
use crate::parse::{consume_punct_if, ident_eq};
use crate::prelude::{Delimiter, Group, Punct, TokenTree};
use crate::{Error, Result};
use std::iter::Peekable;

#[derive(Debug)]
pub enum Attribute {
    Field(FieldAttribute),
    Unknown { punct: Punct, tokens: Option<Group> },
}
#[derive(Debug, PartialEq)]
pub enum FieldAttribute {
    /// The field is a serde type and should implement Encode/Decode through a wrapper
    WithSerde,
}

#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
pub enum AttributeLocation {
    Container,
    Variant,
    Field,
}

impl Attribute {
    pub fn try_take(
        loc: AttributeLocation,
        input: &mut Peekable<impl Iterator<Item = TokenTree>>,
    ) -> Result<Vec<Self>> {
        let mut result = Vec::new();

        while let Some(punct) = consume_punct_if(input, '#') {
            match input.peek() {
                Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Bracket => {
                    let group = assume_group(input.next());
                    let stream = &mut group.stream().into_iter().peekable();
                    if let Some(TokenTree::Ident(attribute_ident)) = stream.peek() {
                        if super::ident_eq(attribute_ident, "bincode") {
                            assume_ident(stream.next());
                            match stream.next() {
                                Some(TokenTree::Group(group)) => {
                                    result.push(Self::parse_bincode_attribute(
                                        loc,
                                        &mut group.stream().into_iter().peekable(),
                                    )?);
                                }
                                token => {
                                    return Error::wrong_token(
                                        token.as_ref(),
                                        "Bracketed group of attributes",
                                    )
                                }
                            }
                            continue;
                        }
                    }
                    result.push(Attribute::Unknown {
                        punct,
                        tokens: Some(group),
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
                    result.push(Attribute::Unknown {
                        punct: assume_punct(input.next(), '#'),
                        tokens: None,
                    })
                }
                token => return Error::wrong_token(token, "[] group or next # attribute"),
            }
        }
        Ok(result)
    }

    fn parse_bincode_attribute(
        loc: AttributeLocation,
        stream: &mut Peekable<impl Iterator<Item = TokenTree>>,
    ) -> Result<Self> {
        match (stream.next(), loc) {
            (Some(TokenTree::Ident(ident)), AttributeLocation::Field)
                if ident_eq(&ident, "with_serde") =>
            {
                Ok(Self::Field(FieldAttribute::WithSerde))
            }
            (token @ Some(TokenTree::Ident(_)), AttributeLocation::Field) => {
                Error::wrong_token(token.as_ref(), "one of: `with_serde`")
            }
            (token @ Some(TokenTree::Ident(_)), loc) => Error::wrong_token(
                token.as_ref(),
                &format!("{:?} attributes not supported", loc),
            ),
            (token, _) => Error::wrong_token(token.as_ref(), "ident"),
        }
    }
}

#[test]
fn test_attributes_try_take() {
    use crate::token_stream;

    let stream = &mut token_stream("struct Foo;");
    assert!(Attribute::try_take(AttributeLocation::Container, stream)
        .unwrap()
        .is_empty());
    match stream.next().unwrap() {
        TokenTree::Ident(i) => assert_eq!(i, "struct"),
        x => panic!("Expected ident, found {:?}", x),
    }

    let stream = &mut token_stream("#[cfg(test)] struct Foo;");
    assert!(!Attribute::try_take(AttributeLocation::Container, stream)
        .unwrap()
        .is_empty());
    match stream.next().unwrap() {
        TokenTree::Ident(i) => assert_eq!(i, "struct"),
        x => panic!("Expected ident, found {:?}", x),
    }
}
