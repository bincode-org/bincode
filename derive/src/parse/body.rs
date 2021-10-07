use super::{assume_group, assume_ident, read_tokens_until_punct, Visibility};
use crate::parse::consume_punct_if;
use crate::prelude::{Delimiter, Ident, Span, TokenTree};
use crate::{Error, Result};
use std::iter::Peekable;

#[derive(Debug)]
pub struct StructBody {
    pub fields: Vec<Field>,
}

impl StructBody {
    pub fn take(input: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<Self> {
        match input.peek() {
            Some(TokenTree::Group(_)) => {}
            Some(TokenTree::Punct(p)) if p.as_char() == ';' => {
                return Ok(StructBody { fields: Vec::new() })
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
            Delimiter::Brace => Field::parse_named(&mut stream)?,
            Delimiter::Parenthesis => Field::parse_unnamed(&mut stream)?,
            _ => return Err(Error::InvalidRustSyntax(group.span())),
        };
        assert!(
            stream.peek().is_none(),
            "Stream should be empty: {:?}",
            stream.collect::<Vec<_>>()
        );
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

    assert_eq!(body.fields[0].vis, Some(Visibility::Pub));
    assert_eq!(body.fields[0].ident.as_ref().unwrap(), "bar");
    assert_eq!(assume_ident(body.fields[0].field_type(0)), "u8");

    assert_eq!(body.fields[1].vis, Some(Visibility::PubCrate));
    assert_eq!(body.fields[1].ident.as_ref().unwrap(), "baz");
    assert_eq!(assume_ident(body.fields[1].field_type(0)), "u32");

    assert_eq!(body.fields[2].vis, None);
    assert_eq!(body.fields[2].ident.as_ref().unwrap(), "bla");

    let stream = &mut token_stream(
        "struct Foo ( pub u8, pub(crate) u32, Vec<Box<dyn Future<Output = ()>>> )",
    );
    let (data_type, ident) = super::DataType::take(stream).unwrap();
    assert_eq!(data_type, super::DataType::Struct);
    assert_eq!(ident, "Foo");
    let body = StructBody::take(stream).unwrap();

    assert_eq!(body.fields.len(), 3);

    assert_eq!(body.fields[0].vis, Some(Visibility::Pub));
    assert!(body.fields[0].ident.is_none());
    assert_eq!(assume_ident(body.fields[0].field_type(0)), "u8");

    assert_eq!(body.fields[1].vis, Some(Visibility::PubCrate));
    assert!(body.fields[1].ident.is_none());
    assert_eq!(assume_ident(body.fields[1].field_type(0)), "u32");

    assert_eq!(body.fields[2].vis, None);
    assert!(body.fields[2].ident.is_none());

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

            let mut fields = None;

            if let Some(TokenTree::Group(_)) = stream.peek() {
                let group = assume_group(stream.next());
                let stream = &mut group.stream().into_iter().peekable();
                match group.delimiter() {
                    Delimiter::Brace => fields = Some(Field::parse_named(stream)?),
                    Delimiter::Parenthesis => fields = Some(Field::parse_unnamed(stream)?),
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
    assert!(body.variants[0].fields.is_none());

    assert_eq!(body.variants[1].name, "Baz");
    assert_eq!(1, body.variants[1].fields.as_ref().unwrap().len());
    let field = &body.variants[1].fields.as_ref().unwrap()[0];
    assert!(field.ident.is_none());
    assert_eq!(assume_ident(field.field_type(0)), "u8");

    assert_eq!(body.variants[2].name, "Blah");
    assert_eq!(2, body.variants[2].fields.as_ref().unwrap().len());
    let field = &body.variants[2].fields.as_ref().unwrap()[0];
    assert_eq!(field.ident.as_ref().unwrap(), "a");
    assert_eq!(assume_ident(field.field_type(0)), "u32");
    let field = &body.variants[2].fields.as_ref().unwrap()[1];
    assert_eq!(field.ident.as_ref().unwrap(), "b");
    assert_eq!(assume_ident(field.field_type(0)), "u128");
}

#[derive(Debug)]
pub struct EnumVariant {
    pub name: Ident,
    pub fields: Option<Vec<Field>>,
}

impl EnumVariant {
    pub fn is_struct_variant(&self) -> bool {
        // An enum variant is a struct variant if it has any fields with a name
        // enum Foo { Bar { a: u32, b: u32 } }
        self.fields
            .as_ref()
            .map(|f| f.iter().any(|f| f.ident.is_some()))
            .unwrap_or(false)
    }
    pub fn is_tuple_variant(&self) -> bool {
        // An enum variant is a struct variant if it has no fields with a name
        // enum Foo { Bar(u32, u32) }
        self.fields
            .as_ref()
            .map(|f| f.iter().all(|f| f.ident.is_none()))
            .unwrap_or(false)
    }
}

#[derive(Debug)]
pub struct Field {
    pub vis: Option<Visibility>,
    pub ident: Option<Ident>,
    pub r#type: Vec<TokenTree>,
}

impl Field {
    pub fn parse_named(input: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<Vec<Self>> {
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
            result.push(Field {
                vis,
                ident: Some(ident),
                r#type,
            });
        }
        Ok(result)
    }

    pub fn parse_unnamed(
        input: &mut Peekable<impl Iterator<Item = TokenTree>>,
    ) -> Result<Vec<Self>> {
        let mut result = Vec::new();
        while input.peek().is_some() {
            let vis = Visibility::try_take(input)?;

            let r#type = read_tokens_until_punct(input, &[','])?;
            consume_punct_if(input, ',');
            result.push(Field {
                vis,
                ident: None,
                r#type,
            });
        }
        Ok(result)
    }

    #[cfg(test)]
    fn field_type(&self, n: usize) -> Option<TokenTree> {
        self.r#type.get(n).cloned()
    }

    pub fn name_or_idx(&self, idx: usize) -> IdentOrString {
        match self.ident.as_ref() {
            Some(i) => IdentOrString::Ident(i),
            None => IdentOrString::String(format!("field_{}", idx)),
        }
    }
}

pub enum IdentOrString<'a> {
    Ident(&'a Ident),
    String(String),
}

impl IdentOrString<'_> {
    pub fn into_token_tree(self) -> TokenTree {
        TokenTree::Ident(match self {
            Self::Ident(i) => i.clone(),
            Self::String(s) => Ident::new(&s, Span::call_site()),
        })
    }
}

impl std::fmt::Display for IdentOrString<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Ident(i) => write!(fmt, "{}", i.to_string()),
            Self::String(s) => write!(fmt, "{}", s),
        }
    }
}
