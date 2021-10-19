use super::{stream_builder::PushParseError, FnBuilder, Generator, StreamBuilder};
use crate::prelude::Delimiter;

#[must_use]
pub struct ImplFor<'a> {
    pub(super) generator: &'a mut Generator,
    pub(super) group: StreamBuilder,
}

impl<'a> ImplFor<'a> {
    pub(super) fn new(
        generator: &'a mut Generator,
        trait_name: &str,
    ) -> Result<Self, PushParseError> {
        let mut builder = StreamBuilder::new();
        builder.ident_str("impl");

        if let Some(generics) = &generator.generics {
            builder.append(generics.impl_generics());
        }
        builder.push_parsed(trait_name)?;
        builder.ident_str("for");
        builder.ident(generator.name.clone());

        if let Some(generics) = &generator.generics {
            builder.append(generics.type_generics());
        }
        if let Some(generic_constraints) = &generator.generic_constraints {
            builder.append(generic_constraints.where_clause());
        }
        generator.stream.append(builder);

        let group = StreamBuilder::new();
        Ok(Self { generator, group })
    }

    pub(super) fn new_with_de_lifetime(
        generator: &'a mut Generator,
        trait_name: &str,
    ) -> Result<Self, PushParseError> {
        let mut builder = StreamBuilder::new();
        builder.ident_str("impl");

        if let Some(generics) = &generator.generics {
            builder.append(generics.impl_generics_with_additional_lifetime("__de"));
        } else {
            builder.punct('<');
            builder.lifetime_str("__de");
            builder.punct('>');
        }

        builder.push_parsed(trait_name)?;
        builder.ident_str("for");
        builder.ident(generator.name.clone());
        if let Some(generics) = &generator.generics {
            builder.append(generics.type_generics());
        }
        if let Some(generic_constraints) = &generator.generic_constraints {
            builder.append(generic_constraints.where_clause());
        }
        generator.stream.append(builder);

        let group = StreamBuilder::new();
        Ok(Self { generator, group })
    }

    /// Add a function to the trait implementation
    pub fn generate_fn<'b>(&'b mut self, name: &str) -> FnBuilder<'a, 'b> {
        FnBuilder::new(self, name)
    }
}

impl Drop for ImplFor<'_> {
    fn drop(&mut self) {
        let stream = std::mem::take(&mut self.group);
        self.generator
            .stream
            .group(Delimiter::Brace, |builder| builder.append(stream))
    }
}
