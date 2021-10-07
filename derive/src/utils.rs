use crate::prelude::{Ident, Literal, Punct, Spacing, Span, TokenTree};

pub fn ident(s: &str) -> TokenTree {
    TokenTree::Ident(Ident::new(s, Span::call_site()))
}
pub fn punct(p: char) -> TokenTree {
    TokenTree::Punct(Punct::new(p, Spacing::Joint))
}

pub fn lit_u32(val: u32) -> TokenTree {
    TokenTree::Literal(Literal::u32_unsuffixed(val))
}

pub fn lit_usize(val: usize) -> TokenTree {
    TokenTree::Literal(Literal::usize_unsuffixed(val))
}
