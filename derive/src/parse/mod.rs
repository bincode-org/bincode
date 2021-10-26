use crate::error::Error;
use crate::prelude::{Delimiter, Group, Ident, Punct, TokenTree};
use std::iter::Peekable;

mod attributes;
mod body;
mod data_type;
mod generics;
mod visibility;

pub use self::attributes::Attribute;
pub use self::body::{EnumBody, EnumVariant, Fields, StructBody, UnnamedField};
pub use self::data_type::DataType;
pub use self::generics::{GenericConstraints, Generics, Lifetime, SimpleGeneric};
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

pub(self) fn consume_punct_if(
    input: &mut Peekable<impl Iterator<Item = TokenTree>>,
    punct: char,
) -> Option<Punct> {
    if let Some(TokenTree::Punct(p)) = input.peek() {
        if p.as_char() == punct {
            match input.next() {
                Some(TokenTree::Punct(p)) => return Some(p),
                _ => unreachable!(),
            }
        }
    }
    None
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
const BRACKET_DELIMITER: &[Option<Delimiter>] = &[
    None,
    Some(Delimiter::Parenthesis),
    Some(Delimiter::Bracket),
    Some(Delimiter::Brace),
];

pub(self) fn read_tokens_until_punct(
    input: &mut Peekable<impl Iterator<Item = TokenTree>>,
    expected_puncts: &[char],
) -> Result<Vec<TokenTree>, Error> {
    let mut result = Vec::new();
    let mut open_brackets = Vec::<char>::new();
    'outer: loop {
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
                            return Err(Error::InvalidRustSyntax {
                                span: punct.span(),
                                expected: format!(
                                    "one of {:?}, got '{}'",
                                    expected_puncts,
                                    punct.as_char()
                                ),
                            });
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
            Some(TokenTree::Group(g)) if open_brackets.is_empty() => {
                for punct in expected_puncts {
                    if let Some(idx) = OPEN_BRACKETS.iter().position(|c| c == punct) {
                        if let Some(delim) = BRACKET_DELIMITER[idx] {
                            if delim == g.delimiter() {
                                // we need to split on this delimiter
                                break 'outer;
                            }
                        }
                    }
                }
                result.push(input.next().unwrap());
            }
            Some(_) => result.push(input.next().unwrap()),
            None => {
                break;
            }
        }
    }
    Ok(result)
}
