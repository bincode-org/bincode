use crate::prelude::*;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    UnknownDataType(Span),
    InvalidRustSyntax(Span),
    ExpectedIdent(Span),
}

// helper functions for the unit tests
#[cfg(test)]
impl Error {
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
            Error::UnknownDataType(span)
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
