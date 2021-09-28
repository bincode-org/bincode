use crate::parse::{GenericConstraints, Generics};
use crate::prelude::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};
use std::str::FromStr;

#[must_use]
pub struct Generate {
    stream: TokenStream,
    group: (Delimiter, TokenStream),
}

impl Generate {
    pub fn impl_for(
        trait_name: &str,
        name: &Ident,
        generics: &Option<Generics>,
        generic_constraints: &Option<GenericConstraints>,
    ) -> Self {
        let mut stream = TokenStream::new();
        stream.extend([ident("impl")]);

        if let Some(generics) = &generics {
            stream.extend([generics.impl_generics()]);
        }
        stream.extend([
            TokenStream::from_str(trait_name).unwrap(),
            ident("for").into(),
            TokenTree::Ident(name.clone()).into(),
        ]);
        if let Some(generics) = &generics {
            stream.extend([generics.type_generics()]);
        }
        if let Some(generic_constraints) = &generic_constraints {
            stream.extend([generic_constraints.where_clause()]);
        }
        let group = (Delimiter::Brace, TokenStream::new());
        Self { stream, group }
    }

    pub fn generate_fn(
        &mut self,
        name: &str,
        constraints: Option<&str>,
        args: &str,
        result: &str,
    ) -> GenerateFnBody {
        let stream = &mut self.group.1;
        // fn name
        stream.extend([ident("fn"), ident(name)]);
        if let Some(constraints) = constraints {
            // <T: Display>
            stream.extend([
                punct('<').into(),
                TokenStream::from_str(constraints).unwrap(),
                punct('>').into(),
            ])
        }
        // (&self, foo: &Bar)
        stream.extend([TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::from_str(args).unwrap(),
        ))]);
        // -> ResultType
        stream.extend([
            punct('-').into(),
            punct('>').into(),
            TokenStream::from_str(result).unwrap(),
        ]);

        GenerateFnBody {
            generate: self,
            group: (Delimiter::Brace, TokenStream::new()),
        }
    }

    pub fn build(mut self) -> TokenStream {
        self.stream
            .extend([TokenTree::Group(Group::new(self.group.0, self.group.1))]);
        self.stream
    }
}

pub struct GenerateFnBody<'a> {
    generate: &'a mut Generate,
    group: (Delimiter, TokenStream),
}

impl GenerateFnBody<'_> {
    pub fn push(&mut self, str: impl AsRef<str>) {
        self.group
            .1
            .extend([TokenStream::from_str(str.as_ref()).unwrap()]);
    }
}

impl<'a> Drop for GenerateFnBody<'a> {
    fn drop(&mut self) {
        let stream = std::mem::replace(&mut self.group.1, TokenStream::new());
        self.generate
            .group
            .1
            .extend([TokenTree::Group(Group::new(self.group.0, stream))]);
    }
}

fn ident(s: &str) -> TokenTree {
    TokenTree::Ident(Ident::new(s, Span::call_site()))
}
fn punct(p: char) -> TokenTree {
    TokenTree::Punct(Punct::new(p, Spacing::Joint))
}
