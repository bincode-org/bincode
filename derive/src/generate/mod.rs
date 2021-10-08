mod stream_builder;

pub use self::stream_builder::StreamBuilder;

use crate::parse::{GenericConstraints, Generics};
use crate::prelude::{Delimiter, Ident, TokenStream};

#[must_use]
pub struct Generator {
    name: Ident,
    generics: Option<Generics>,
    generic_constraints: Option<GenericConstraints>,
    stream: StreamBuilder,
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

    pub fn impl_for<'a>(&'a mut self, trait_name: &str) -> ImplFor<'a> {
        ImplFor::new(self, trait_name)
    }

    pub fn impl_for_with_de_lifetime<'a>(&'a mut self, trait_name: &str) -> ImplFor<'a> {
        ImplFor::new_with_de_lifetime(self, trait_name)
    }

    pub fn has_lifetimes(&self) -> bool {
        self.generics
            .as_ref()
            .map(|g| g.has_lifetime())
            .unwrap_or(false)
    }

    pub fn take_stream(mut self) -> TokenStream {
        std::mem::take(&mut self.stream.stream)
    }
}

impl Drop for Generator {
    fn drop(&mut self) {
        if !self.stream.stream.is_empty() && !std::thread::panicking() {
            panic!("Generator dropped but the stream is not empty. Please call `.take_stream()` on the generator");
        }
    }
}

#[must_use]
pub struct ImplFor<'a> {
    generator: &'a mut Generator,
    group: StreamBuilder,
}

impl<'a> ImplFor<'a> {
    fn new(generator: &'a mut Generator, trait_name: &str) -> Self {
        let mut builder = StreamBuilder::new();
        builder.ident_str("impl");

        if let Some(generics) = &generator.generics {
            builder.append(generics.impl_generics());
        }
        builder.push_parsed(trait_name);
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
        Self { generator, group }
    }

    fn new_with_de_lifetime(generator: &'a mut Generator, trait_name: &str) -> Self {
        let mut builder = StreamBuilder::new();
        builder.ident_str("impl");

        if let Some(generics) = &generator.generics {
            builder.append(generics.impl_generics_with_additional_lifetime("__de"));
        } else {
            builder.punct('<');
            builder.lifetime_str("__de");
            builder.punct('>');
        }

        builder.push_parsed(trait_name);
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
        Self { generator, group }
    }

    pub fn generate_fn<'b>(
        &'b mut self,
        name: &str,
        builder: impl FnOnce(FnBuilder) -> FnBuilder,
    ) -> GenerateFnBody<'a, 'b> {
        let FnBuilder {
            name,
            lifetime_and_generics,
            self_arg,
            args,
            return_type,
        } = builder(FnBuilder::new(name));

        let mut builder = StreamBuilder::new();

        // function name; `fn name`
        builder.ident_str("fn");
        builder.ident_str(name);

        // lifetimes; `<'a: 'b, D: Display>`
        if !lifetime_and_generics.is_empty() {
            builder.punct('<');
            for (idx, (lifetime_and_generic, dependencies)) in
                lifetime_and_generics.into_iter().enumerate()
            {
                if idx != 0 {
                    builder.punct(',');
                }
                builder.ident_str(&lifetime_and_generic);
                if !dependencies.is_empty() {
                    for (idx, dependency) in dependencies.into_iter().enumerate() {
                        builder.punct(if idx == 0 { ':' } else { '+' });
                        builder.push_parsed(&dependency);
                    }
                }
            }
            builder.punct('>');
        }

        // Arguments; `(&self, foo: &Bar)`
        builder.group(Delimiter::Parenthesis, |arg_stream| {
            if let Some(self_arg) = self_arg.into_token_tree() {
                arg_stream.append(self_arg);
                arg_stream.punct(',');
            }
            for (idx, (arg_name, arg_ty)) in args.into_iter().enumerate() {
                if idx != 0 {
                    arg_stream.punct(',');
                }
                arg_stream.push_parsed(&arg_name);
                arg_stream.punct(':');
                arg_stream.push_parsed(&arg_ty);
            }
        });

        // Return type: `-> ResultType`
        if let Some(return_type) = return_type {
            builder.puncts("->");
            builder.push_parsed(&return_type);
        }

        self.group.append(builder);

        GenerateFnBody {
            generate: self,
            group: StreamBuilder::new(),
        }
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

pub struct GenerateFnBody<'a, 'b> {
    generate: &'b mut ImplFor<'a>,
    group: StreamBuilder,
}

impl GenerateFnBody<'_, '_> {
    pub fn stream(&mut self) -> &mut StreamBuilder {
        &mut self.group
    }
}

impl Drop for GenerateFnBody<'_, '_> {
    fn drop(&mut self) {
        let stream = std::mem::take(&mut self.group);
        self.generate.group.group(Delimiter::Brace, |b| *b = stream)
    }
}

pub struct FnBuilder {
    name: String,
    lifetime_and_generics: Vec<(String, Vec<String>)>,
    self_arg: FnSelfArg,
    args: Vec<(String, String)>,
    return_type: Option<String>,
}

impl FnBuilder {
    fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            lifetime_and_generics: Vec::new(),
            self_arg: FnSelfArg::None,
            args: Vec::new(),
            return_type: None,
        }
    }

    pub fn with_generic<T, U, V>(mut self, name: T, dependencies: U) -> Self
    where
        T: Into<String>,
        U: IntoIterator<Item = V>,
        V: Into<String>,
    {
        self.lifetime_and_generics.push((
            name.into(),
            dependencies.into_iter().map(|d| d.into()).collect(),
        ));
        self
    }

    pub fn with_self_arg(mut self, self_arg: FnSelfArg) -> Self {
        self.self_arg = self_arg;
        self
    }

    pub fn with_arg(mut self, name: impl Into<String>, ty: impl Into<String>) -> Self {
        self.args.push((name.into(), ty.into()));
        self
    }

    pub fn with_return_type(mut self, ret_type: impl Into<String>) -> Self {
        self.return_type = Some(ret_type.into());
        self
    }
}

pub enum FnSelfArg {
    None,
    RefSelf,
}

impl FnSelfArg {
    fn into_token_tree(self) -> Option<StreamBuilder> {
        let mut builder = StreamBuilder::new();
        match self {
            Self::None => return None,
            Self::RefSelf => {
                builder.punct('&');
                builder.ident_str("self");
            }
        }
        Some(builder)
    }
}
