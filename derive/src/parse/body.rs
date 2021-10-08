use super::{assume_group, assume_ident, read_tokens_until_punct, Visibility};
use crate::parse::consume_punct_if;
use crate::prelude::{Delimiter, Ident, Span, TokenTree};
use crate::{Error, Result};
use std::iter::Peekable;

#[derive(Debug)]
pub struct StructBody {
    pub fields: Fields,
}

impl StructBody {
    pub fn take(input: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<Self> {
        match input.peek() {
            Some(TokenTree::Group(_)) => {}
            Some(TokenTree::Punct(p)) if p.as_char() == ';' => {
                return Ok(StructBody {
                    fields: Fields::Unit,
                })
            }
            Some(t) => {
                return Err(Error::InvalidRustSyntax(t.span()));
            }
            _ => {
                return Err(Error::InvalidRustSyntax(Span::call_site()));
            }
        }
        let group = assume_group(input.next());
        let mut stream = group.stream().into_iter().peekable();
        let fields = match group.delimiter() {
            Delimiter::Brace => Fields::Struct(UnnamedField::parse_with_name(&mut stream)?),
            Delimiter::Parenthesis => Fields::Tuple(UnnamedField::parse(&mut stream)?),
            _ => return Err(Error::InvalidRustSyntax(group.span())),
        };
        Ok(StructBody { fields })
    }
}

#[test]
fn test_struct_body_take() {
    use crate::token_stream;

    let stream = &mut token_stream(
        "struct Foo { pub bar: u8, pub(crate) baz: u32, bla: Vec<Box<dyn Future<Output = ()>>> }",
    );
    let (data_type, ident) = super::DataType::take(stream).unwrap();
    assert_eq!(data_type, super::DataType::Struct);
    assert_eq!(ident, "Foo");
    let body = StructBody::take(stream).unwrap();

    assert_eq!(body.fields.len(), 3);
    let (ident, field) = body.fields.get(0).unwrap();
    assert_eq!(ident.unwrap(), "bar");
    assert_eq!(field.vis, Visibility::Pub);
    assert_eq!(field.type_string(), "u8");

    let (ident, field) = body.fields.get(1).unwrap();
    assert_eq!(ident.unwrap(), "baz");
    assert_eq!(field.vis, Visibility::Pub);
    assert_eq!(field.type_string(), "u32");

    let (ident, field) = body.fields.get(2).unwrap();
    assert_eq!(ident.unwrap(), "bla");
    assert_eq!(field.vis, Visibility::Default);
    assert_eq!(field.type_string(), "Vec<Box<dynFuture<Output=()>>>");

    let stream = &mut token_stream(
        "struct Foo ( pub u8, pub(crate) u32, Vec<Box<dyn Future<Output = ()>>> )",
    );
    let (data_type, ident) = super::DataType::take(stream).unwrap();
    assert_eq!(data_type, super::DataType::Struct);
    assert_eq!(ident, "Foo");
    let body = StructBody::take(stream).unwrap();

    assert_eq!(body.fields.len(), 3);

    let (ident, field) = body.fields.get(0).unwrap();
    assert!(ident.is_none());
    assert_eq!(field.vis, Visibility::Pub);
    assert_eq!(field.type_string(), "u8");

    let (ident, field) = body.fields.get(1).unwrap();
    assert!(ident.is_none());
    assert_eq!(field.vis, Visibility::Pub);
    assert_eq!(field.type_string(), "u32");

    let (ident, field) = body.fields.get(2).unwrap();
    assert!(ident.is_none());
    assert_eq!(field.vis, Visibility::Default);
    assert_eq!(field.type_string(), "Vec<Box<dynFuture<Output=()>>>");

    let stream = &mut token_stream("struct Foo;");
    let (data_type, ident) = super::DataType::take(stream).unwrap();
    assert_eq!(data_type, super::DataType::Struct);
    assert_eq!(ident, "Foo");
    let body = StructBody::take(stream).unwrap();
    assert_eq!(body.fields.len(), 0);

    let stream = &mut token_stream("struct Foo {}");
    let (data_type, ident) = super::DataType::take(stream).unwrap();
    assert_eq!(data_type, super::DataType::Struct);
    assert_eq!(ident, "Foo");
    let body = StructBody::take(stream).unwrap();
    assert_eq!(body.fields.len(), 0);

    let stream = &mut token_stream("struct Foo ()");
    let (data_type, ident) = super::DataType::take(stream).unwrap();
    assert_eq!(data_type, super::DataType::Struct);
    assert_eq!(ident, "Foo");
    assert_eq!(body.fields.len(), 0);
}

#[derive(Debug)]
pub struct EnumBody {
    pub variants: Vec<EnumVariant>,
}

impl EnumBody {
    pub fn take(input: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<Self> {
        match input.peek() {
            Some(TokenTree::Group(_)) => {}
            Some(TokenTree::Punct(p)) if p.as_char() == ';' => {
                return Ok(EnumBody {
                    variants: Vec::new(),
                })
            }
            Some(t) => {
                return Err(Error::InvalidRustSyntax(t.span()));
            }
            _ => {
                return Err(Error::InvalidRustSyntax(Span::call_site()));
            }
        }
        let group = assume_group(input.next());
        let mut variants = Vec::new();
        let stream = &mut group.stream().into_iter().peekable();
        while stream.peek().is_some() {
            let ident = match stream.peek() {
                Some(TokenTree::Ident(_)) => assume_ident(stream.next()),
                Some(x) => return Err(Error::InvalidRustSyntax(x.span())),
                None => return Err(Error::InvalidRustSyntax(Span::call_site())),
            };

            let mut fields = Fields::Unit;

            if let Some(TokenTree::Group(_)) = stream.peek() {
                let group = assume_group(stream.next());
                let stream = &mut group.stream().into_iter().peekable();
                match group.delimiter() {
                    Delimiter::Brace => {
                        fields = Fields::Struct(UnnamedField::parse_with_name(stream)?)
                    }
                    Delimiter::Parenthesis => fields = Fields::Tuple(UnnamedField::parse(stream)?),
                    _ => return Err(Error::InvalidRustSyntax(group.span())),
                }
            }
            consume_punct_if(stream, ',');

            variants.push(EnumVariant {
                name: ident,
                fields,
            });
        }

        Ok(EnumBody { variants })
    }
}

#[test]
fn test_enum_body_take() {
    use crate::token_stream;

    let stream = &mut token_stream("enum Foo { }");
    let (data_type, ident) = super::DataType::take(stream).unwrap();
    assert_eq!(data_type, super::DataType::Enum);
    assert_eq!(ident, "Foo");
    let body = EnumBody::take(stream).unwrap();
    assert_eq!(0, body.variants.len());

    let stream = &mut token_stream("enum Foo { Bar, Baz(u8), Blah { a: u32, b: u128 } }");
    let (data_type, ident) = super::DataType::take(stream).unwrap();
    assert_eq!(data_type, super::DataType::Enum);
    assert_eq!(ident, "Foo");
    let body = EnumBody::take(stream).unwrap();
    assert_eq!(3, body.variants.len());

    assert_eq!(body.variants[0].name, "Bar");
    assert!(body.variants[0].fields.is_unit());

    assert_eq!(body.variants[1].name, "Baz");
    assert_eq!(1, body.variants[1].fields.len());
    let (ident, field) = body.variants[1].fields.get(0).unwrap();
    assert!(ident.is_none());
    assert_eq!(field.type_string(), "u8");

    assert_eq!(body.variants[2].name, "Blah");
    assert_eq!(2, body.variants[2].fields.len());
    let (ident, field) = body.variants[2].fields.get(0).unwrap();
    assert_eq!(ident.unwrap(), "a");
    assert_eq!(field.type_string(), "u32");
    let (ident, field) = body.variants[2].fields.get(1).unwrap();
    assert_eq!(ident.unwrap(), "b");
    assert_eq!(field.type_string(), "u128");
}

#[derive(Debug)]
pub struct EnumVariant {
    pub name: Ident,
    pub fields: Fields,
}

#[derive(Debug)]
pub enum Fields {
    /// Empty variant.
    /// ```rs
    /// enum Foo {
    ///     Baz,
    /// }
    /// struct Bar { }
    /// ```
    Unit,

    /// Tuple-like variant
    /// ```rs
    /// enum Foo {
    ///     Baz(u32)
    /// }
    /// struct Bar(u32);
    /// ```
    Tuple(Vec<UnnamedField>),

    /// Struct-like variant
    /// ```rs
    /// enum Foo {
    ///     Baz {
    ///         baz: u32
    ///     }
    /// }
    /// struct Bar {
    ///     baz: u32
    /// }
    /// ```
    Struct(Vec<(Ident, UnnamedField)>),
}

impl Fields {
    pub fn names(&self) -> Vec<IdentOrIndex> {
        match self {
            Self::Tuple(fields) => fields
                .iter()
                .enumerate()
                .map(|(idx, field)| IdentOrIndex::Index(idx, field.span()))
                .collect(),
            Self::Struct(fields) => fields
                .iter()
                .map(|(ident, _)| IdentOrIndex::Ident(ident))
                .collect(),
            Self::Unit => Vec::new(),
        }
    }

    pub fn delimiter(&self) -> Option<Delimiter> {
        match self {
            Self::Tuple(_) => Some(Delimiter::Parenthesis),
            Self::Struct(_) => Some(Delimiter::Brace),
            Self::Unit => None,
        }
    }
}

#[cfg(test)]
impl Fields {
    pub fn is_unit(&self) -> bool {
        matches!(self, Self::Unit)
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Tuple(fields) => fields.len(),
            Self::Struct(fields) => fields.len(),
            Self::Unit => 0,
        }
    }

    pub fn get(&self, index: usize) -> Option<(Option<&Ident>, &UnnamedField)> {
        match self {
            Self::Tuple(fields) => fields.get(index).map(|f| (None, f)),
            Self::Struct(fields) => fields.get(index).map(|(ident, field)| (Some(ident), field)),
            Self::Unit => None,
        }
    }
}

#[derive(Debug)]
pub struct UnnamedField {
    pub vis: Visibility,
    pub r#type: Vec<TokenTree>,
}

impl UnnamedField {
    pub fn parse_with_name(
        input: &mut Peekable<impl Iterator<Item = TokenTree>>,
    ) -> Result<Vec<(Ident, Self)>> {
        let mut result = Vec::new();
        loop {
            let vis = Visibility::try_take(input)?;

            let ident = match input.peek() {
                Some(TokenTree::Ident(_)) => assume_ident(input.next()),
                Some(x) => return Err(Error::InvalidRustSyntax(x.span())),
                None => break,
            };
            match input.peek() {
                Some(TokenTree::Punct(p)) if p.as_char() == ':' => {
                    input.next();
                }
                Some(x) => return Err(Error::InvalidRustSyntax(x.span())),
                None => return Err(Error::InvalidRustSyntax(Span::call_site())),
            }
            let r#type = read_tokens_until_punct(input, &[','])?;
            consume_punct_if(input, ',');
            result.push((ident, Self { vis, r#type }));
        }
        Ok(result)
    }

    pub fn parse(input: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<Vec<Self>> {
        let mut result = Vec::new();
        while input.peek().is_some() {
            let vis = Visibility::try_take(input)?;

            let r#type = read_tokens_until_punct(input, &[','])?;
            consume_punct_if(input, ',');
            result.push(Self { vis, r#type });
        }
        Ok(result)
    }

    #[cfg(test)]
    pub fn type_string(&self) -> String {
        self.r#type.iter().map(|t| t.to_string()).collect()
    }

    pub fn span(&self) -> Span {
        // BlockedTODO: https://github.com/rust-lang/rust/issues/54725
        // Span::join is unstable
        // if let Some(first) = self.r#type.first() {
        //     let mut span = first.span();
        //     for token in self.r#type.iter().skip(1) {
        //         span = span.join(span).unwrap();
        //     }
        //     span
        // } else {
        //     Span::call_site()
        // }

        match self.r#type.first() {
            Some(first) => first.span(),
            None => Span::call_site(),
        }
    }
}

#[derive(Debug)]
pub enum IdentOrIndex<'a> {
    Ident(&'a Ident),
    Index(usize, Span),
}

impl<'a> IdentOrIndex<'a> {
    pub fn unwrap_ident(&self) -> &'a Ident {
        match self {
            Self::Ident(i) => i,
            x => panic!("Expected ident, found {:?}", x),
        }
    }

    pub fn to_token_tree_with_prefix(&self, prefix: &str) -> TokenTree {
        TokenTree::Ident(match self {
            IdentOrIndex::Ident(i) => (*i).clone(),
            IdentOrIndex::Index(idx, span) => {
                let name = format!("{}{}", prefix, idx);
                Ident::new(&name, *span)
            }
        })
    }
    pub fn to_string_with_prefix(&self, prefix: &str) -> String {
        match self {
            IdentOrIndex::Ident(i) => i.to_string(),
            IdentOrIndex::Index(idx, _) => {
                format!("{}{}", prefix, idx)
            }
        }
    }
}

impl std::fmt::Display for IdentOrIndex<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            IdentOrIndex::Ident(i) => write!(fmt, "{}", i),
            IdentOrIndex::Index(idx, _) => write!(fmt, "{}", idx),
        }
    }
}
