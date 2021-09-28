use super::assume_punct;
use crate::parse::{ident_eq, read_tokens_until_punct};
use crate::prelude::{Ident, Punct, Spacing, TokenStream, TokenTree};
use crate::{Error, Result};
use std::iter::Peekable;

#[derive(Debug)]
pub struct Generics {
    lifetimes_and_generics: Vec<LifetimeOrGeneric>,
}

impl Generics {
    pub fn try_take(input: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<Option<Self>> {
        let maybe_punct = input.peek();
        if let Some(TokenTree::Punct(punct)) = maybe_punct {
            if punct.as_char() == '<' {
                let punct = super::assume_punct(input.next(), '<');
                let mut result = Generics {
                    lifetimes_and_generics: Vec::new(),
                };
                loop {
                    match input.peek() {
                        Some(TokenTree::Punct(punct)) if punct.as_char() == '\'' => {
                            result
                                .lifetimes_and_generics
                                .push(Lifetime::take(input)?.into());
                            super::consume_punct_if(input, ',');
                        }
                        Some(TokenTree::Punct(punct)) if punct.as_char() == '>' => {
                            assume_punct(input.next(), '>');
                            break;
                        }
                        Some(TokenTree::Ident(_)) => {
                            result
                                .lifetimes_and_generics
                                .push(Generic::take(input)?.into());
                            super::consume_punct_if(input, ',');
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

    pub fn impl_generics(&self) -> TokenStream {
        let mut result = vec![TokenTree::Punct(Punct::new('<', Spacing::Alone))];

        let mut is_first = true;
        for generic in &self.lifetimes_and_generics {
            if is_first {
                is_first = false;
            } else {
                result.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));
            }

            if generic.is_lifetime() {
                result.push(TokenTree::Punct(Punct::new('\'', Spacing::Joint)));
            }

            result.push(TokenTree::Ident(generic.ident()));

            if generic.has_constraints() {
                result.push(TokenTree::Punct(Punct::new(':', Spacing::Alone)));
                result.extend(generic.constraints());
            }
        }

        result.push(TokenTree::Punct(Punct::new('>', Spacing::Alone)));

        let mut stream = TokenStream::new();
        stream.extend(result);
        stream
    }

    pub fn type_generics(&self) -> TokenStream {
        let mut result = vec![TokenTree::Punct(Punct::new('<', Spacing::Alone))];

        let mut is_first = true;
        for generic in &self.lifetimes_and_generics {
            if is_first {
                is_first = false;
            } else {
                result.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));
            }
            if generic.is_lifetime() {
                result.push(TokenTree::Punct(Punct::new('\'', Spacing::Joint)));
            }

            result.push(TokenTree::Ident(generic.ident()));
        }

        result.push(TokenTree::Punct(Punct::new('>', Spacing::Alone)));

        let mut stream = TokenStream::new();
        stream.extend(result);
        stream
    }
}

#[derive(Debug)]
enum LifetimeOrGeneric {
    Lifetime(Lifetime),
    Generic(Generic),
}

impl LifetimeOrGeneric {
    fn is_lifetime(&self) -> bool {
        matches!(self, LifetimeOrGeneric::Lifetime(_))
    }

    fn ident(&self) -> Ident {
        match self {
            Self::Lifetime(lt) => lt.ident.clone(),
            Self::Generic(gen) => gen.ident.clone(),
        }
    }

    fn has_constraints(&self) -> bool {
        match self {
            Self::Lifetime(lt) => !lt.constraint.is_empty(),
            Self::Generic(gen) => !gen.constraints.is_empty(),
        }
    }

    fn constraints(&self) -> Vec<TokenTree> {
        match self {
            Self::Lifetime(lt) => lt.constraint.clone(),
            Self::Generic(gen) => gen.constraints.clone(),
        }
    }
}

impl From<Lifetime> for LifetimeOrGeneric {
    fn from(lt: Lifetime) -> Self {
        Self::Lifetime(lt)
    }
}

impl From<Generic> for LifetimeOrGeneric {
    fn from(gen: Generic) -> Self {
        Self::Generic(gen)
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
    assert_eq!(generics.lifetimes_and_generics.len(), 2);
    assert_eq!(generics.lifetimes_and_generics[0].ident(), "a");
    assert_eq!(generics.lifetimes_and_generics[1].ident(), "T");

    let mut stream = token_stream("struct Foo<A, B>()");
    assert!(DataType::take(&mut stream).unwrap().is_struct("Foo"));
    let generics = Generics::try_take(&mut stream).unwrap().unwrap();
    assert_eq!(generics.lifetimes_and_generics.len(), 2);
    assert_eq!(generics.lifetimes_and_generics[0].ident(), "A");
    assert_eq!(generics.lifetimes_and_generics[1].ident(), "B");

    let mut stream = token_stream("struct Foo<'a, T: Display>()");
    assert!(DataType::take(&mut stream).unwrap().is_struct("Foo"));
    let generics = Generics::try_take(&mut stream).unwrap().unwrap();
    dbg!(&generics);
    assert_eq!(generics.lifetimes_and_generics.len(), 2);
    assert_eq!(generics.lifetimes_and_generics[0].ident(), "a");
    assert_eq!(generics.lifetimes_and_generics[1].ident(), "T");

    let mut stream = token_stream("struct Foo<'a, T: for<'a> Bar<'a> + 'static>()");
    assert!(DataType::take(&mut stream).unwrap().is_struct("Foo"));
    let generics = Generics::try_take(&mut stream).unwrap().unwrap();
    dbg!(&generics);
    assert_eq!(generics.lifetimes_and_generics.len(), 2);
    assert_eq!(generics.lifetimes_and_generics[0].ident(), "a");
    assert_eq!(generics.lifetimes_and_generics[1].ident(), "T");

    let mut stream =
        token_stream("struct Baz<T: for<'a> Bar<'a, for<'b> Bar<'b, for<'c> Bar<'c, u32>>>> {}");
    assert!(DataType::take(&mut stream).unwrap().is_struct("Baz"));
    let generics = Generics::try_take(&mut stream).unwrap().unwrap();
    dbg!(&generics);
    assert_eq!(generics.lifetimes_and_generics.len(), 1);
    assert_eq!(generics.lifetimes_and_generics[0].ident(), "T");

    let mut stream = token_stream("struct Baz<()> {}");
    assert!(DataType::take(&mut stream).unwrap().is_struct("Baz"));
    assert!(Generics::try_take(&mut stream)
        .unwrap_err()
        .is_invalid_rust_syntax());

    let mut stream = token_stream("struct Bar<A: FnOnce(&'static str) -> SomeStruct, B>");
    assert!(DataType::take(&mut stream).unwrap().is_struct("Bar"));
    let generics = Generics::try_take(&mut stream).unwrap().unwrap();
    dbg!(&generics);
    assert_eq!(generics.lifetimes_and_generics.len(), 2);
    assert_eq!(generics.lifetimes_and_generics[0].ident(), "A");
    assert_eq!(generics.lifetimes_and_generics[1].ident(), "B");
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
    assume_punct(stream.next(), '>');
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
}

#[derive(Debug)]
pub struct GenericConstraints {
    constraints: Vec<TokenTree>,
}

impl GenericConstraints {
    pub fn try_take(input: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<Option<Self>> {
        match input.peek() {
            Some(TokenTree::Ident(ident)) => {
                if !ident_eq(ident, "where") {
                    return Ok(None);
                }
            }
            _ => {
                return Ok(None);
            }
        }
        input.next();
        let constraints = read_tokens_until_punct(input, &['{', '('])?;
        Ok(Some(Self { constraints }))
    }

    pub fn where_clause(&self) -> TokenStream {
        use std::str::FromStr;
        let mut stream = TokenStream::from_str("where").unwrap();
        stream.extend(self.constraints.clone());
        stream
    }
}

#[test]
fn test_generic_constraints_try_take() {
    use super::{DataType, StructBody, Visibility};
    use crate::token_stream;

    let stream = &mut token_stream("struct Foo where Foo: Bar { }");
    super::DataType::take(stream).unwrap();
    assert!(GenericConstraints::try_take(stream).unwrap().is_some());

    let stream = &mut token_stream("struct Foo { }");
    super::DataType::take(stream).unwrap();
    assert!(GenericConstraints::try_take(stream).unwrap().is_none());

    let stream = &mut token_stream("struct Foo where Foo: Bar(Foo)");
    super::DataType::take(stream).unwrap();
    assert!(GenericConstraints::try_take(stream).unwrap().is_some());

    let stream = &mut token_stream("struct Foo()");
    super::DataType::take(stream).unwrap();
    assert!(GenericConstraints::try_take(stream).unwrap().is_none());

    let stream = &mut token_stream("struct Foo()");
    assert!(GenericConstraints::try_take(stream).unwrap().is_none());

    let stream = &mut token_stream("{}");
    assert!(GenericConstraints::try_take(stream).unwrap().is_none());

    let stream = &mut token_stream("");
    assert!(GenericConstraints::try_take(stream).unwrap().is_none());

    let stream = &mut token_stream("pub(crate) struct Test<T: Encodeable> {}");
    assert_eq!(Ok(Some(Visibility::PubCrate)), Visibility::try_take(stream));
    assert!(DataType::take(stream).unwrap().is_struct("Test"));
    let constraints = Generics::try_take(stream).unwrap().unwrap();
    assert_eq!(constraints.lifetimes_and_generics.len(), 1);
    assert_eq!(constraints.lifetimes_and_generics[0].ident(), "T");
    let body = StructBody::take(stream).unwrap();
    assert_eq!(body.fields.len(), 0);
}
