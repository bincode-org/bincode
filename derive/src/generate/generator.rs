use super::stream_builder::PushParseError;
use super::{ImplFor, StreamBuilder};
use crate::parse::{GenericConstraints, Generics};
use crate::prelude::{Ident, TokenStream};

#[must_use]
pub struct Generator {
    pub(super) name: Ident,
    pub(super) generics: Option<Generics>,
    pub(super) generic_constraints: Option<GenericConstraints>,
    pub(super) stream: StreamBuilder,
}

impl Generator {
    pub(crate) fn new(
        name: Ident,
        generics: Option<Generics>,
        generic_constraints: Option<GenericConstraints>,
    ) -> Self {
        Self {
            name,
            generics,
            generic_constraints,
            stream: StreamBuilder::new(),
        }
    }

    /// Return the name for the struct or enum that this is going to be implemented on.
    pub fn target_name(&self) -> &Ident {
        &self.name
    }

    /// Generate an `for <trait_name> for <target_name>` implementation. See [ImplFor] for more information.
    pub fn impl_for<'a>(&'a mut self, trait_name: &str) -> Result<ImplFor<'a>, PushParseError> {
        ImplFor::new(self, trait_name)
    }

    /// Generate an `for <'__de> <trait_name> for <target_name>` implementation. See [ImplFor] for more information.
    pub fn impl_for_with_de_lifetime<'a>(
        &'a mut self,
        trait_name: &str,
    ) -> Result<ImplFor<'a>, PushParseError> {
        ImplFor::new_with_de_lifetime(self, trait_name)
    }

    /// Returns `true` if the struct or enum has lifetimes.
    pub fn has_lifetimes(&self) -> bool {
        self.generics
            .as_ref()
            .map(|g| g.has_lifetime())
            .unwrap_or(false)
    }

    /// Consume the contents of this generator. This *must* be called, or else the generator will panic on drop.
    pub fn take_stream(mut self) -> TokenStream {
        std::mem::take(&mut self.stream).stream
    }
}

impl Drop for Generator {
    fn drop(&mut self) {
        if !self.stream.stream.is_empty() && !std::thread::panicking() {
            panic!("Generator dropped but the stream is not empty. Please call `.take_stream()` on the generator");
        }
    }
}
