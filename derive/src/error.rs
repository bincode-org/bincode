use crate::{generate::StreamBuilder, prelude::*};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    UnknownDataType(Span),
    InvalidRustSyntax { span: Span, expected: String },
    ExpectedIdent(Span),
}

impl Error {
    pub fn wrong_token<T>(token: Option<&TokenTree>, expected: &'static str) -> Result<T, Self> {
        Err(Self::InvalidRustSyntax {
            span: token.map(|t| t.span()).unwrap_or_else(Span::call_site),
            expected: format!("{}, got {:?}", expected, token),
        })
    }
}

// helper functions for the unit tests
#[cfg(test)]
impl Error {
    pub fn is_unknown_data_type(&self) -> bool {
        matches!(self, Error::UnknownDataType(_))
    }

    pub fn is_invalid_rust_syntax(&self) -> bool {
        matches!(self, Error::InvalidRustSyntax { .. })
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UnknownDataType(_) => {
                write!(fmt, "Unknown data type, only enum and struct are supported")
            }
            Self::InvalidRustSyntax { expected, .. } => {
                write!(fmt, "Invalid rust syntax, expected {}", expected)
            }
            Self::ExpectedIdent(_) => write!(fmt, "Expected ident"),
        }
    }
}

impl Error {
    pub fn into_token_stream(self) -> TokenStream {
        let maybe_span = match &self {
            Error::UnknownDataType(span)
            | Error::ExpectedIdent(span)
            | Error::InvalidRustSyntax { span, .. } => Some(*span),
        };
        self.throw_with_span(maybe_span.unwrap_or_else(Span::call_site))
    }

    pub fn throw_with_span(self, span: Span) -> TokenStream {
        // compile_error!($message)
        let mut builder = StreamBuilder::new();
        builder.ident_str("compile_error");
        builder.punct('!');
        builder.group(Delimiter::Brace, |b| {
            b.lit_str(self.to_string());
        });
        builder.set_span_on_all_tokens(span);
        builder.stream
    }
}
