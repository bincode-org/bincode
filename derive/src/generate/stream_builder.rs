use crate::prelude::{
    Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree,
};
use std::str::FromStr;

#[must_use]
#[derive(Default)]
pub struct StreamBuilder {
    pub(super) stream: TokenStream,
}

impl StreamBuilder {
    pub fn new() -> Self {
        Self {
            stream: TokenStream::new(),
        }
    }

    pub fn extend(&mut self, item: impl IntoIterator<Item = TokenTree>) {
        self.stream.extend(item);
    }

    pub fn append(&mut self, builder: StreamBuilder) {
        self.stream.extend(builder.stream);
    }

    pub fn push(&mut self, item: impl Into<TokenTree>) {
        self.stream.extend([item.into()]);
    }

    pub fn push_parsed(&mut self, item: impl AsRef<str>) {
        self.stream
            .extend(TokenStream::from_str(item.as_ref()).unwrap_or_else(|e| {
                panic!(
                    "Could not parse string as rust: {:?}\n{:?}",
                    item.as_ref(),
                    e
                )
            }));
    }

    pub fn ident(&mut self, ident: Ident) {
        self.stream.extend([TokenTree::Ident(ident)]);
    }

    pub fn ident_str(&mut self, ident: impl AsRef<str>) {
        self.stream.extend([TokenTree::Ident(Ident::new(
            ident.as_ref(),
            Span::call_site(),
        ))]);
    }

    pub fn group(&mut self, delim: Delimiter, inner: impl FnOnce(&mut StreamBuilder)) {
        let mut stream = StreamBuilder::new();
        inner(&mut stream);
        self.stream
            .extend([TokenTree::Group(Group::new(delim, stream.stream))]);
    }

    pub fn punct(&mut self, p: char) {
        self.stream
            .extend([TokenTree::Punct(Punct::new(p, Spacing::Alone))]);
    }

    pub fn puncts(&mut self, puncts: &str) {
        self.stream.extend(
            puncts
                .chars()
                .map(|char| TokenTree::Punct(Punct::new(char, Spacing::Joint))),
        );
    }

    pub fn lifetime(&mut self, lt: Ident) {
        self.stream.extend([
            TokenTree::Punct(Punct::new('\'', Spacing::Joint)),
            TokenTree::Ident(lt),
        ]);
    }
    pub fn lifetime_str(&mut self, lt: &str) {
        self.stream.extend([
            TokenTree::Punct(Punct::new('\'', Spacing::Joint)),
            TokenTree::Ident(Ident::new(lt, Span::call_site())),
        ]);
    }

    pub fn lit_u32(&mut self, val: u32) {
        self.stream
            .extend([TokenTree::Literal(Literal::u32_unsuffixed(val))]);
    }

    pub fn lit_usize(&mut self, val: usize) {
        self.stream
            .extend([TokenTree::Literal(Literal::usize_unsuffixed(val))]);
    }
}
