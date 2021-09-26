use proc_macro2::TokenTree;
use std::iter::Peekable;

mod data_type;
mod generics;
mod visibility;

pub use self::data_type::DataType;
pub use self::generics::{Generic, Generics, Lifetime};
pub use self::visibility::Visibility;

pub(self) fn assume_group(t: Option<TokenTree>) -> proc_macro2::Group {
    match t {
        Some(TokenTree::Group(group)) => group,
        _ => unreachable!(),
    }
}
pub(self) fn assume_ident(t: Option<TokenTree>) -> proc_macro2::Ident {
    match t {
        Some(TokenTree::Ident(ident)) => ident,
        _ => unreachable!(),
    }
}
pub(self) fn assume_punct(t: Option<TokenTree>, punct: char) -> proc_macro2::Punct {
    match t {
        Some(TokenTree::Punct(p)) => {
            debug_assert_eq!(punct, p.as_char());
            p
        }
        _ => unreachable!(),
    }
}

pub(self) fn consume_punct_if(input: &mut Peekable<impl Iterator<Item = TokenTree>>, punct: &str) {
    if let Some(TokenTree::Punct(p)) = input.peek() {
        if p.to_string() == punct {
            input.next();
        }
    }
}
