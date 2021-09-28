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
            Some(TokenTree::Group(_)) => {
                let group = assume_group(input.next());
                dbg!(&group);
                let mut stream = group.stream().into_iter().peekable();
                let fields = match group.delimiter() {
                    Delimiter::Brace => Field::parse_named(&mut stream)?,
                    Delimiter::Parenthesis => Field::parse_unnamed(&mut stream)?,
                    _ => return Err(Error::InvalidRustSyntax(group.span())),
                };
                dbg!(&fields);
                assert!(
                    stream.peek().is_none(),
                    "Stream should be empty: {:?}",
                    stream.collect::<Vec<_>>()
                );
                Ok(StructBody { fields })
            }
            Some(TokenTree::Punct(p)) if p.as_char() == ';' => {
                Ok(StructBody { fields: Vec::new() })
            }
            Some(t) => Err(Error::InvalidRustSyntax(t.span())),
            _ => Err(Error::InvalidRustSyntax(Span::call_site())),
        }
    }
}

#[test]
fn test_struct_body_take() {
    use crate::token_stream;

    let stream = &mut token_stream(
        "struct Foo { pub bar: u8, pub(crate) baz: u32, bla: Vec<Box<dyn Future<Output = ()>>> }",
    );
    let data_type = super::DataType::take(stream).unwrap();
    assert!(data_type.is_struct("Foo"));
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
    let data_type = super::DataType::take(stream).unwrap();
    assert!(data_type.is_struct("Foo"));
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
    let data_type = super::DataType::take(stream).unwrap();
    assert!(data_type.is_struct("Foo"));
    let body = StructBody::take(stream).unwrap();
    assert_eq!(body.fields.len(), 0);

    let stream = &mut token_stream("struct Foo {}");
    let data_type = super::DataType::take(stream).unwrap();
    assert!(data_type.is_struct("Foo"));
    let body = StructBody::take(stream).unwrap();
    assert_eq!(body.fields.len(), 0);

    let stream = &mut token_stream("struct Foo ()");
    let data_type = super::DataType::take(stream).unwrap();
    assert!(data_type.is_struct("Foo"));
    let body = StructBody::take(stream).unwrap();
    assert_eq!(body.fields.len(), 0);
}

#[derive(Debug)]
pub struct EnumBody {
    pub variants: Vec<EnumVariant>,
}

impl EnumBody {
    pub fn take(_input: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<Self> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct EnumVariant {
    pub name: Ident,
    pub fields: Option<Vec<Field>>,
}

#[derive(Debug)]
pub struct Field {
    pub vis: Option<Visibility>,
    pub ident: Option<Ident>,
    pub r#type: Option<Vec<TokenTree>>,
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
                r#type: Some(r#type),
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
                r#type: Some(r#type),
            });
        }
        Ok(result)
    }

    #[cfg(test)]
    fn field_type(&self, n: usize) -> Option<TokenTree> {
        self.r#type.as_ref().and_then(|t| t.get(n)).cloned()
    }
}
