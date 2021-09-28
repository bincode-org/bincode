use crate::error::Error;
use crate::prelude::{Group, Ident, Punct, Span, TokenTree};
use std::iter::Peekable;

mod data_type;
mod generics;
mod visibility;

pub use self::data_type::DataType;
pub use self::generics::{Generic, Generics, Lifetime};
pub use self::visibility::Visibility;

pub(self) fn assume_group(t: Option<TokenTree>) -> Group {
    match t {
        Some(TokenTree::Group(group)) => group,
        _ => unreachable!(),
    }
}
pub(self) fn assume_ident(t: Option<TokenTree>) -> Ident {
    match t {
        Some(TokenTree::Ident(ident)) => ident,
        _ => unreachable!(),
    }
}
pub(self) fn assume_punct(t: Option<TokenTree>, punct: char) -> Punct {
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

#[cfg(test)]
pub(self) fn ident_eq(ident: &Ident, text: &str) -> bool {
    ident == text
}

#[cfg(not(test))]
pub(self) fn ident_eq(ident: &Ident, text: &str) -> bool {
    ident.to_string() == text
}

fn check_if_arrow(tokens: &[TokenTree], punct: &Punct) -> bool {
    if punct.as_char() == '>' {
        if let Some(TokenTree::Punct(previous_punct)) = tokens.last() {
            if previous_punct.as_char() == '-' {
                return true;
            }
        }
    }
    false
}

const OPEN_BRACKETS: &[char] = &['<', '(', '[', '{'];
const CLOSING_BRACKETS: &[char] = &['>', ')', ']', '}'];

pub(self) fn read_tokens_until_punct(
    input: &mut Peekable<impl Iterator<Item = TokenTree>>,
    expected_puncts: &[char],
) -> Result<Vec<TokenTree>, Error> {
    let mut result = Vec::new();
    let mut open_brackets = Vec::<char>::new();
    loop {
        match input.peek() {
            Some(TokenTree::Punct(punct)) => {
                if check_if_arrow(&result, punct) {
                    // do nothing
                } else if OPEN_BRACKETS.contains(&punct.as_char()) {
                    open_brackets.push(punct.as_char());
                } else if let Some(index) =
                    CLOSING_BRACKETS.iter().position(|c| c == &punct.as_char())
                {
                    let last_bracket = match open_brackets.pop() {
                        Some(bracket) => bracket,
                        None => {
                            if expected_puncts.contains(&punct.as_char()) {
                                break;
                            }
                            return Err(Error::InvalidRustSyntax(punct.span()));
                        }
                    };
                    let expected = OPEN_BRACKETS[index];
                    assert_eq!(
                        expected,
                        last_bracket,
                        "Unexpected closing bracket: found {}, expected {}",
                        punct.as_char(),
                        expected
                    );
                } else if expected_puncts.contains(&punct.as_char()) && open_brackets.is_empty() {
                    break;
                }
                result.push(input.next().unwrap());
            }
            Some(_) => result.push(input.next().unwrap()),
            None => {
                return Err(Error::InvalidRustSyntax(
                    result
                        .last()
                        .map(|c| c.span())
                        .unwrap_or_else(Span::call_site),
                ))
            }
        }
    }
    Ok(result)
}
