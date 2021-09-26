use proc_macro2::*;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    UnknownVisibility(Span),
    UnknownDataType(Span),
    InvalidRustSyntax(Span),
    ExpectedIdent(Span),
    // UnionNotSupported,
}

impl PartialEq for Error {
    fn eq(&self, other: &Error) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match (self, other) {
            (Error::UnknownVisibility(_), Error::UnknownVisibility(_)) => true,
            (Error::UnknownDataType(_), Error::UnknownDataType(_)) => true,
            // (Error::UnionNotSupported, Error::UnionNotSupported) => true,
            (Error::InvalidRustSyntax(_), Error::InvalidRustSyntax(_)) => true,
            (Error::ExpectedIdent(_), Error::ExpectedIdent(_)) => true,
            _ => false,
        }
    }
}

// helper functions for the unit tests
#[cfg(test)]
impl Error {
    pub fn is_unknown_visibility(&self) -> bool {
        matches!(self, Error::UnknownVisibility(_))
    }

    pub fn is_unknown_data_type(&self) -> bool {
        matches!(self, Error::UnknownDataType(_))
    }

    pub fn is_invalid_rust_syntax(&self) -> bool {
        matches!(self, Error::InvalidRustSyntax(_))
    }

    // pub fn is_expected_ident(&self) -> bool {
    //     matches!(self, Error::ExpectedIdent(_))
    // }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UnknownVisibility(_) => write!(fmt, "Unknown visibility"),
            Self::UnknownDataType(_) => {
                write!(fmt, "Unknown data type, only enum and struct are supported")
            }
            // Self::UnionNotSupported => write!(fmt, "Unions are not supported"),
            Self::InvalidRustSyntax(_) => write!(fmt, "Invalid rust syntax"),
            Self::ExpectedIdent(_) => write!(fmt, "Expected ident"),
        }
    }
}

impl Error {
    pub fn into_token_stream(self) -> TokenStream {
        let maybe_span = match &self {
            Error::UnknownVisibility(span)
            | Error::UnknownDataType(span)
            | Error::ExpectedIdent(span)
            | Error::InvalidRustSyntax(span) => Some(*span),
            // Error::UnionNotSupported => None,
        };
        self.throw_with_span(maybe_span.unwrap_or_else(Span::call_site))
    }

    pub fn throw_with_span(self, span: Span) -> TokenStream {
        // compile_error!($message)
        vec![
            TokenTree::Ident(Ident::new("compile_error", span)),
            TokenTree::Punct({
                let mut punct = Punct::new('!', Spacing::Alone);
                punct.set_span(span);
                punct
            }),
            TokenTree::Group({
                let mut group = Group::new(Delimiter::Brace, {
                    TokenTree::Literal({
                        let mut string = Literal::string(&self.to_string());
                        string.set_span(span);
                        string
                    })
                    .into()
                });
                group.set_span(span);
                group
            }),
        ]
        .into_iter()
        .collect()
    }
}
