use crate::{Error, Result};
use proc_macro2::{Ident, TokenTree};
use std::iter::Peekable;

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
        match input.peek() {
            Some(TokenTree::Ident(_)) => {
                let ident = super::assume_ident(input.next());
                let constraint = Vec::new();
                // todo: parse constraints, e.g. `'a: 'b + 'c + 'static`

                Ok(Lifetime { ident, constraint })
            }
            Some(t) => Err(Error::ExpectedIdent(t.span())),
            None => Err(Error::ExpectedIdent(start.span())),
        }
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
    assert!(catch_unwind(|| Lifetime::take(&mut token_stream(""))).is_err());
    assert!(catch_unwind(|| Lifetime::take(&mut token_stream("'0"))).is_err());
    assert!(catch_unwind(|| Lifetime::take(&mut token_stream("'("))).is_err());
    assert!(catch_unwind(|| Lifetime::take(&mut token_stream("')"))).is_err());
    assert!(catch_unwind(|| Lifetime::take(&mut token_stream("'0'"))).is_err());
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
                let mut open_brackets = Vec::<char>::new();
                loop {
                    match input.peek() {
                        Some(TokenTree::Punct(punct)) => {
                            dbg!(punct);
                            if ['<', '(', '[', '{'].contains(&punct.as_char()) {
                                open_brackets.push(punct.as_char());
                            } else if ['>', ')', ']', '}'].contains(&punct.as_char()) {
                                let last_bracket = match open_brackets.pop() {
                                    Some(bracket) => Some(bracket),
                                    None if punct.as_char() == '>' => {
                                        // if the previous token was punctiation `-`, then this is part of an arrow ->
                                        let is_arrow = if let Some(TokenTree::Punct(punct)) =
                                            constraints.last()
                                        {
                                            punct.as_char() == '-'
                                        } else {
                                            false
                                        };
                                        if is_arrow {
                                            None
                                        } else {
                                            // If it's not an arrow, then it's the closing bracket of the actual generic group
                                            break;
                                        }
                                    }
                                    None => {
                                        return Err(Error::InvalidRustSyntax(punct.span()));
                                    }
                                };
                                if let Some(last_bracket) = last_bracket {
                                    let expected = match last_bracket {
                                        '<' => '>',
                                        '{' => '}',
                                        '(' => ')',
                                        '[' => ']',
                                        _ => unreachable!(),
                                    };
                                    assert_eq!(
                                        expected,
                                        punct.as_char(),
                                        "Unexpected closing bracket: found {}, expected {}",
                                        punct.as_char(),
                                        expected
                                    );
                                }
                            } else if punct.as_char() == ',' && open_brackets.is_empty() {
                                break;
                            }
                            constraints.push(input.next().unwrap());
                        }
                        Some(_) => constraints.push(input.next().unwrap()),
                        None => {
                            return Err(Error::InvalidRustSyntax(
                                constraints
                                    .last()
                                    .map(|c| c.span())
                                    .unwrap_or_else(|| ident.span()),
                            ))
                        }
                    }
                }
            }
        }
        Ok(Generic { ident, constraints })
    }

    #[cfg(test)]
    fn is_ident(&self, i: &str) -> bool {
        self.ident.to_string() == i
    }
}
