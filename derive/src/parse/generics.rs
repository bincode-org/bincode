use super::assume_punct;
use crate::generate::StreamBuilder;
use crate::parse::{ident_eq, read_tokens_until_punct};
use crate::prelude::{Ident, TokenTree};
use crate::{Error, Result};
use std::iter::Peekable;

#[derive(Debug)]
pub struct Generics {
    generics: Vec<Generic>,
}

impl Generics {
    pub fn try_take(input: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<Option<Self>> {
        let maybe_punct = input.peek();
        if let Some(TokenTree::Punct(punct)) = maybe_punct {
            if punct.as_char() == '<' {
                let punct = super::assume_punct(input.next(), '<');
                let mut result = Generics {
                    generics: Vec::new(),
                };
                loop {
                    match input.peek() {
                        Some(TokenTree::Punct(punct)) if punct.as_char() == '\'' => {
                            result.generics.push(Lifetime::take(input)?.into());
                            super::consume_punct_if(input, ',');
                        }
                        Some(TokenTree::Punct(punct)) if punct.as_char() == '>' => {
                            assume_punct(input.next(), '>');
                            break;
                        }
                        Some(TokenTree::Ident(ident)) if ident_eq(ident, "const") => {
                            result.generics.push(ConstGeneric::take(input)?.into());
                            super::consume_punct_if(input, ',');
                        }
                        Some(TokenTree::Ident(_)) => {
                            result.generics.push(SimpleGeneric::take(input)?.into());
                            super::consume_punct_if(input, ',');
                        }
                        x => {
                            return Err(Error::InvalidRustSyntax {
                                span: x.map(|x| x.span()).unwrap_or_else(|| punct.span()),
                                expected: format!("', > or an ident, got {:?}", x),
                            });
                        }
                    }
                }
                return Ok(Some(result));
            }
        }
        Ok(None)
    }

    pub fn has_lifetime(&self) -> bool {
        self.generics.iter().any(|lt| lt.is_lifetime())
    }

    pub fn impl_generics(&self) -> StreamBuilder {
        let mut result = StreamBuilder::new();
        result.punct('<');

        for (idx, generic) in self.generics.iter().enumerate() {
            if idx > 0 {
                result.punct(',');
            }

            generic.append_to_result_with_constraints(&mut result);
        }

        result.punct('>');

        result
    }

    pub fn impl_generics_with_additional_lifetime(&self, lifetime: &str) -> StreamBuilder {
        assert!(self.has_lifetime());

        let mut result = StreamBuilder::new();
        result.punct('<');
        result.lifetime_str(lifetime);

        if self.has_lifetime() {
            for (idx, lt) in self
                .generics
                .iter()
                .filter_map(|lt| lt.as_lifetime())
                .enumerate()
            {
                result.punct(if idx == 0 { ':' } else { '+' });
                result.lifetime(lt.ident.clone());
            }
        }

        for generic in &self.generics {
            result.punct(',');
            generic.append_to_result_with_constraints(&mut result);
        }

        result.punct('>');

        result
    }

    pub fn type_generics(&self) -> StreamBuilder {
        let mut result = StreamBuilder::new();
        result.punct('<');

        for (idx, generic) in self.generics.iter().enumerate() {
            if idx > 0 {
                result.punct(',');
            }
            if generic.is_lifetime() {
                result.lifetime(generic.ident());
            } else {
                result.ident(generic.ident());
            }
        }

        result.punct('>');
        result
    }
}

#[derive(Debug)]
enum Generic {
    Lifetime(Lifetime),
    Generic(SimpleGeneric),
    Const(ConstGeneric),
}

impl Generic {
    fn is_lifetime(&self) -> bool {
        matches!(self, Generic::Lifetime(_))
    }

    fn ident(&self) -> Ident {
        match self {
            Self::Lifetime(lt) => lt.ident.clone(),
            Self::Generic(gen) => gen.ident.clone(),
            Self::Const(gen) => gen.ident.clone(),
        }
    }

    fn as_lifetime(&self) -> Option<&Lifetime> {
        match self {
            Self::Lifetime(lt) => Some(lt),
            _ => None,
        }
    }

    fn has_constraints(&self) -> bool {
        match self {
            Self::Lifetime(lt) => !lt.constraint.is_empty(),
            Self::Generic(gen) => !gen.constraints.is_empty(),
            Self::Const(_) => true, // const generics always have a constraint
        }
    }

    fn constraints(&self) -> Vec<TokenTree> {
        match self {
            Self::Lifetime(lt) => lt.constraint.clone(),
            Self::Generic(gen) => gen.constraints.clone(),
            Self::Const(gen) => gen.constraints.clone(),
        }
    }

    fn append_to_result_with_constraints(&self, builder: &mut StreamBuilder) {
        match self {
            Self::Lifetime(lt) => builder.lifetime(lt.ident.clone()),
            Self::Generic(gen) => {
                builder.ident(gen.ident.clone());
            }
            Self::Const(gen) => {
                builder.ident(gen.const_token.clone());
                builder.ident(gen.ident.clone());
            }
        }
        if self.has_constraints() {
            builder.punct(':');
            builder.extend(self.constraints());
        }
    }
}

impl From<Lifetime> for Generic {
    fn from(lt: Lifetime) -> Self {
        Self::Lifetime(lt)
    }
}

impl From<SimpleGeneric> for Generic {
    fn from(gen: SimpleGeneric) -> Self {
        Self::Generic(gen)
    }
}

impl From<ConstGeneric> for Generic {
    fn from(gen: ConstGeneric) -> Self {
        Self::Const(gen)
    }
}

#[test]
fn test_generics_try_take() {
    use crate::token_stream;

    assert!(Generics::try_take(&mut token_stream("")).unwrap().is_none());
    assert!(Generics::try_take(&mut token_stream("foo"))
        .unwrap()
        .is_none());
    assert!(Generics::try_take(&mut token_stream("()"))
        .unwrap()
        .is_none());

    let stream = &mut token_stream("struct Foo<'a, T>()");
    let (data_type, ident) = super::DataType::take(stream).unwrap();
    assert_eq!(data_type, super::DataType::Struct);
    assert_eq!(ident, "Foo");
    let generics = Generics::try_take(stream).unwrap().unwrap();
    assert_eq!(generics.generics.len(), 2);
    assert_eq!(generics.generics[0].ident(), "a");
    assert_eq!(generics.generics[1].ident(), "T");

    let stream = &mut token_stream("struct Foo<A, B>()");
    let (data_type, ident) = super::DataType::take(stream).unwrap();
    assert_eq!(data_type, super::DataType::Struct);
    assert_eq!(ident, "Foo");
    let generics = Generics::try_take(stream).unwrap().unwrap();
    assert_eq!(generics.generics.len(), 2);
    assert_eq!(generics.generics[0].ident(), "A");
    assert_eq!(generics.generics[1].ident(), "B");

    let stream = &mut token_stream("struct Foo<'a, T: Display>()");
    let (data_type, ident) = super::DataType::take(stream).unwrap();
    assert_eq!(data_type, super::DataType::Struct);
    assert_eq!(ident, "Foo");
    let generics = Generics::try_take(stream).unwrap().unwrap();
    dbg!(&generics);
    assert_eq!(generics.generics.len(), 2);
    assert_eq!(generics.generics[0].ident(), "a");
    assert_eq!(generics.generics[1].ident(), "T");

    let stream = &mut token_stream("struct Foo<'a, T: for<'a> Bar<'a> + 'static>()");
    let (data_type, ident) = super::DataType::take(stream).unwrap();
    assert_eq!(data_type, super::DataType::Struct);
    assert_eq!(ident, "Foo");
    dbg!(&generics);
    assert_eq!(generics.generics.len(), 2);
    assert_eq!(generics.generics[0].ident(), "a");
    assert_eq!(generics.generics[1].ident(), "T");

    let stream = &mut token_stream(
        "struct Baz<T: for<'a> Bar<'a, for<'b> Bar<'b, for<'c> Bar<'c, u32>>>> {}",
    );
    let (data_type, ident) = super::DataType::take(stream).unwrap();
    assert_eq!(data_type, super::DataType::Struct);
    assert_eq!(ident, "Baz");
    let generics = Generics::try_take(stream).unwrap().unwrap();
    dbg!(&generics);
    assert_eq!(generics.generics.len(), 1);
    assert_eq!(generics.generics[0].ident(), "T");

    let stream = &mut token_stream("struct Baz<()> {}");
    let (data_type, ident) = super::DataType::take(stream).unwrap();
    assert_eq!(data_type, super::DataType::Struct);
    assert_eq!(ident, "Baz");
    assert!(Generics::try_take(stream)
        .unwrap_err()
        .is_invalid_rust_syntax());

    let stream = &mut token_stream("struct Bar<A: FnOnce(&'static str) -> SomeStruct, B>");
    let (data_type, ident) = super::DataType::take(stream).unwrap();
    assert_eq!(data_type, super::DataType::Struct);
    assert_eq!(ident, "Bar");
    let generics = Generics::try_take(stream).unwrap().unwrap();
    dbg!(&generics);
    assert_eq!(generics.generics.len(), 2);
    assert_eq!(generics.generics[0].ident(), "A");
    assert_eq!(generics.generics[1].ident(), "B");
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
pub struct SimpleGeneric {
    ident: Ident,
    constraints: Vec<TokenTree>,
}

impl SimpleGeneric {
    pub fn take(input: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<Self> {
        let ident = super::assume_ident(input.next());
        let mut constraints = Vec::new();
        if let Some(TokenTree::Punct(punct)) = input.peek() {
            if punct.as_char() == ':' {
                super::assume_punct(input.next(), ':');
                constraints = super::read_tokens_until_punct(input, &['>', ','])?;
            }
        }
        Ok(Self { ident, constraints })
    }
}

#[derive(Debug)]
pub struct ConstGeneric {
    const_token: Ident,
    ident: Ident,
    constraints: Vec<TokenTree>,
}

impl ConstGeneric {
    pub fn take(input: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<Self> {
        let const_token = super::assume_ident(input.next());
        let ident = super::assume_ident(input.next());
        let mut constraints = Vec::new();
        if let Some(TokenTree::Punct(punct)) = input.peek() {
            if punct.as_char() == ':' {
                super::assume_punct(input.next(), ':');
                constraints = super::read_tokens_until_punct(input, &['>', ','])?;
            }
        }
        Ok(Self {
            const_token,
            ident,
            constraints,
        })
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

    pub fn where_clause(&self) -> StreamBuilder {
        let mut result = StreamBuilder::new();
        result.ident_str("where");
        result.extend(self.constraints.clone());
        result
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

    let stream = &mut token_stream("pub(crate) struct Test<T: Encode> {}");
    assert_eq!(Visibility::Pub, Visibility::try_take(stream).unwrap());
    let (data_type, ident) = DataType::take(stream).unwrap();
    assert_eq!(data_type, DataType::Struct);
    assert_eq!(ident, "Test");
    let constraints = Generics::try_take(stream).unwrap().unwrap();
    assert_eq!(constraints.generics.len(), 1);
    assert_eq!(constraints.generics[0].ident(), "T");
    let body = StructBody::take(stream).unwrap();
    assert_eq!(body.fields.len(), 0);
}
