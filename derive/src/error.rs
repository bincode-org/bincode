use proc_macro::TokenStream;
use quote::__private::Span;
use std::fmt;

pub enum Error {
    UnionNotSupported,
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UnionNotSupported => write!(fmt, "Unions are not supported"),
        }
    }
}

impl Error {
    pub fn into_token_stream(self) -> TokenStream {
        self.into_token_stream_with_span(Span::call_site())
    }
    pub fn into_token_stream_with_span(self, span: Span) -> TokenStream {
        syn::Error::new(span, self).into_compile_error().into()
    }
}
