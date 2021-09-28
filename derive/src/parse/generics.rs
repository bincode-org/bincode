use crate::prelude::{Ident, TokenTree};
use crate::{Error, Result};
use std::iter::Peekable;

use super::assume_punct;

#[derive(Debug)]
pub struct Generics {
    lifetimes: Vec<Lifetime>,
    generics: Vec<Generic>,
}
impl Generics {
    pub fn try_take(input: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<Option<Self>> {
        let maybe_punct = input.peek();
        if let Some(TokenTree::Punct(punct)) = maybe_punct {
            if punct.as_char() == '<' {
                let punct = super::assume_punct(input.next(), '<');
                let mut result = Generics {
                    lifetimes: Vec::new(),
                    generics: Vec::new(),
                };
                loop {
                    match input.peek() {
                        Some(TokenTree::Punct(punct)) if punct.as_char() == '\'' => {
                            result.lifetimes.push(Lifetime::take(input)?);
                            super::consume_punct_if(input, ",");
                        }
                        Some(TokenTree::Punct(punct)) if punct.as_char() == '>' => {
                            break;
                        }
                        Some(TokenTree::Ident(_)) => {
                            result.generics.push(Generic::take(input)?);
                            super::consume_punct_if(input, ",");
                        }
                        x => {
                            return Err(Error::InvalidRustSyntax(
                                x.map(|x| x.span()).unwrap_or_else(|| punct.span()),
                            ));
                        }
                    }
                }
                return Ok(Some(result));
            }
        }
        Ok(None)
    }
}

#[test]
fn test_generics_try_take() {
    use super::DataType;
    use crate::token_stream;

    assert!(Generics::try_take(&mut token_stream("")).unwrap().is_none());
    assert!(Generics::try_take(&mut token_stream("foo"))
        .unwrap()
        .is_none());
    assert!(Generics::try_take(&mut token_stream("()"))
        .unwrap()
        .is_none());

    let mut stream = token_stream("struct Foo<'a, T>()");
    assert!(DataType::take(&mut stream).unwrap().is_struct("Foo"));
    let generics = Generics::try_take(&mut stream).unwrap().unwrap();
    assert_eq!(generics.lifetimes.len(), 1);
    assert_eq!(generics.generics.len(), 1);
    assert!(generics.lifetimes[0].is_ident("a"));
    assert!(generics.generics[0].is_ident("T"));

    let mut stream = token_stream("struct Foo<A, B>()");
    assert!(DataType::take(&mut stream).unwrap().is_struct("Foo"));
    let generics = Generics::try_take(&mut stream).unwrap().unwrap();
    assert_eq!(generics.lifetimes.len(), 0);
    assert_eq!(generics.generics.len(), 2);
    assert!(generics.generics[0].is_ident("A"));
    assert!(generics.generics[1].is_ident("B"));

    let mut stream = token_stream("struct Foo<'a, T: Display>()");
    assert!(DataType::take(&mut stream).unwrap().is_struct("Foo"));
    let generics = Generics::try_take(&mut stream).unwrap().unwrap();
    dbg!(&generics);
    assert_eq!(generics.lifetimes.len(), 1);
    assert_eq!(generics.generics.len(), 1);
    assert!(generics.lifetimes[0].is_ident("a"));
    assert!(generics.generics[0].is_ident("T"));

    let mut stream = token_stream("struct Foo<'a, T: for<'a> Bar<'a> + 'static>()");
    assert!(DataType::take(&mut stream).unwrap().is_struct("Foo"));
    let generics = Generics::try_take(&mut stream).unwrap().unwrap();
    dbg!(&generics);
    assert_eq!(generics.lifetimes.len(), 1);
    assert_eq!(generics.generics.len(), 1);
    assert!(generics.lifetimes[0].is_ident("a"));
    assert!(generics.generics[0].is_ident("T"));

    let mut stream =
        token_stream("struct Baz<T: for<'a> Bar<'a, for<'b> Bar<'b, for<'c> Bar<'c, u32>>>> {}");
    assert!(DataType::take(&mut stream).unwrap().is_struct("Baz"));
    let generics = Generics::try_take(&mut stream).unwrap().unwrap();
    dbg!(&generics);
    assert_eq!(generics.lifetimes.len(), 0);
    assert_eq!(generics.generics.len(), 1);
    assert!(generics.generics[0].is_ident("T"));

    let mut stream = token_stream("struct Baz<()> {}");
    assert!(DataType::take(&mut stream).unwrap().is_struct("Baz"));
    assert!(Generics::try_take(&mut stream)
        .unwrap_err()
        .is_invalid_rust_syntax());

    let mut stream = token_stream("struct Bar<A: FnOnce(&'static str) -> SomeStruct, B>");
    assert!(DataType::take(&mut stream).unwrap().is_struct("Bar"));
    let generics = Generics::try_take(&mut stream).unwrap().unwrap();
    dbg!(&generics);
    assert_eq!(generics.lifetimes.len(), 0);
    assert_eq!(generics.generics.len(), 2);
    assert!(generics.generics[0].is_ident("A"));
    assert!(generics.generics[1].is_ident("B"));
}

#[derive(Debug)]
pub struct Lifetime {
    ident: Ident,
    constraint: Vec<TokenTree>,
}

impl Lifetime {
    pub fn take(input: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<Self> {
        let start = super::assume_punct(input.next(), '\'');
        let ident = match input.peek() {
            Some(TokenTree::Ident(_)) => super::assume_ident(input.next()),
            Some(t) => return Err(Error::ExpectedIdent(t.span())),
            None => return Err(Error::ExpectedIdent(start.span())),
        };

        let mut constraint = Vec::new();
        if let Some(TokenTree::Punct(p)) = input.peek() {
            if p.as_char() == ':' {
                assume_punct(input.next(), ':');
                constraint = super::read_tokens_until_punct(input, &[',', '>'])?;
            }
        }

        Ok(Self { ident, constraint })
    }

    #[cfg(test)]
    fn is_ident(&self, s: &str) -> bool {
        self.ident.to_string() == s
    }
}

#[test]
fn test_lifetime_take() {
    use crate::token_stream;
    use std::panic::catch_unwind;
    assert!(Lifetime::take(&mut token_stream("'a"))
        .unwrap()
        .is_ident("a"));
    assert!(catch_unwind(|| Lifetime::take(&mut token_stream("'0"))).is_err());
    assert!(catch_unwind(|| Lifetime::take(&mut token_stream("'("))).is_err());
    assert!(catch_unwind(|| Lifetime::take(&mut token_stream("')"))).is_err());
    assert!(catch_unwind(|| Lifetime::take(&mut token_stream("'0'"))).is_err());

    let stream = &mut token_stream("'a: 'b>");
    let lifetime = Lifetime::take(stream).unwrap();
    assert_eq!(lifetime.ident, "a");
    assert_eq!(lifetime.constraint.len(), 2);
    if let Some(TokenTree::Punct(p)) = stream.next() {
        assert_eq!(p.as_char(), '>');
    } else {
        assert!(false);
    }
    assert!(stream.next().is_none());
}

#[derive(Debug)]
pub struct Generic {
    ident: Ident,
    constraints: Vec<TokenTree>,
}

impl Generic {
    pub fn take(input: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<Self> {
        let ident = super::assume_ident(input.next());
        let mut constraints = Vec::new();
        if let Some(TokenTree::Punct(punct)) = input.peek() {
            if punct.as_char() == ':' {
                super::assume_punct(input.next(), ':');
                constraints = super::read_tokens_until_punct(input, &['>', ','])?;
            }
        }
        Ok(Generic { ident, constraints })
    }

    #[cfg(test)]
    fn is_ident(&self, i: &str) -> bool {
        self.ident.to_string() == i
    }
}
