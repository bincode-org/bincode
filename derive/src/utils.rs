use crate::prelude::{Ident, Punct, Spacing, Span, TokenTree};

pub fn ident(s: &str) -> TokenTree {
    TokenTree::Ident(Ident::new(s, Span::call_site()))
}
pub fn punct(p: char) -> TokenTree {
    TokenTree::Punct(Punct::new(p, Spacing::Joint))
}
