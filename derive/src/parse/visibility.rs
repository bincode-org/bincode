use crate::{Error, Result};
use proc_macro2::TokenTree;
use std::iter::Peekable;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Visibility {
    Pub,
    PubCrate,
}

impl Visibility {
    pub fn try_take(input: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<Option<Self>> {
        if let Some(TokenTree::Ident(ident)) = input.peek() {
            if ident == "pub" {
                // Consume this token
                let ident = super::assume_ident(input.next());

                // check if the next token is `pub(crate)`
                if let Some(TokenTree::Group(_)) = input.peek() {
                    let group = super::assume_group(input.next());
                    let mut group_stream = group.stream().into_iter();
                    return match (group_stream.next(), group_stream.next()) {
                        (Some(TokenTree::Ident(ident)), None) => {
                            if ident == "crate" {
                                return Ok(Some(Visibility::PubCrate));
                            } else {
                                Err(Error::UnknownVisibility(ident.span()))
                            }
                        }
                        _ => Err(Error::UnknownVisibility(ident.span())),
                    };
                }

                return Ok(Some(Visibility::Pub));
            }
        }
        Ok(None)
    }
}

#[test]
fn test_visibility_try_take() {
    use crate::token_stream;

    assert_eq!(Ok(None), Visibility::try_take(&mut token_stream("")));
    assert_eq!(
        Ok(Some(Visibility::Pub)),
        Visibility::try_take(&mut token_stream("pub"))
    );
    assert_eq!(
        Ok(Some(Visibility::Pub)),
        Visibility::try_take(&mut token_stream(" pub "))
    );
    assert_eq!(
        Ok(Some(Visibility::Pub)),
        Visibility::try_take(&mut token_stream("\tpub\t"))
    );
    assert_eq!(
        Ok(Some(Visibility::PubCrate)),
        Visibility::try_take(&mut token_stream("pub(crate)"))
    );
    assert_eq!(
        Ok(Some(Visibility::PubCrate)),
        Visibility::try_take(&mut token_stream(" pub ( crate ) "))
    );
    assert_eq!(
        Ok(Some(Visibility::PubCrate)),
        Visibility::try_take(&mut token_stream("\tpub\t(\tcrate\t)\t"))
    );

    assert!(Visibility::try_take(&mut token_stream("pub(foo)"))
        .unwrap_err()
        .is_unknown_visibility());

    assert!(Visibility::try_take(&mut token_stream("pub(,)"))
        .unwrap_err()
        .is_unknown_visibility());

    assert_eq!(Ok(None), Visibility::try_take(&mut token_stream("pb")));
}
