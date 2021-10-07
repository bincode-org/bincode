use crate::parse::{GenericConstraints, Generics};
use crate::prelude::{Delimiter, Group, Ident, TokenStream, TokenTree};
use crate::utils::*;
use std::str::FromStr;

#[must_use]
pub struct Generator {
    name: Ident,
    generics: Option<Generics>,
    generic_constraints: Option<GenericConstraints>,
    stream: TokenStream,
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
            stream: TokenStream::new(),
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
        std::mem::replace(&mut self.stream, TokenStream::new())
    }
}

impl Drop for Generator {
    fn drop(&mut self) {
        if !self.stream.is_empty() && !std::thread::panicking() {
            panic!("Generator dropped but the stream is not empty. Please call `.take_stream()` on the generator");
        }
    }
}

#[must_use]
pub struct ImplFor<'a> {
    generator: &'a mut Generator,
    group: (Delimiter, TokenStream),
}

impl<'a> ImplFor<'a> {
    fn new(generator: &'a mut Generator, trait_name: &str) -> Self {
        let mut stream = vec![ident("impl")];

        if let Some(generics) = &generator.generics {
            stream.extend(generics.impl_generics());
        }
        stream.extend(TokenStream::from_str(trait_name).unwrap());
        stream.extend([ident("for"), TokenTree::Ident(generator.name.clone())]);

        if let Some(generics) = &generator.generics {
            stream.extend(generics.type_generics());
        }
        if let Some(generic_constraints) = &generator.generic_constraints {
            stream.extend(generic_constraints.where_clause());
        }
        generator.stream.extend(stream);

        let group = (Delimiter::Brace, TokenStream::new());
        Self { generator, group }
    }

    fn new_with_de_lifetime(generator: &'a mut Generator, trait_name: &str) -> Self {
        let mut stream = vec![ident("impl")];

        if let Some(generics) = &generator.generics {
            stream.extend(generics.impl_generics_with_additional_lifetime("__de"));
        } else {
            stream.extend([punct('<'), punct('\''), ident("__de"), punct('>')]);
        }

        stream.extend(TokenStream::from_str(trait_name).unwrap());
        stream.extend([ident("for"), TokenTree::Ident(generator.name.clone())]);
        if let Some(generics) = &generator.generics {
            stream.extend(generics.type_generics());
        }
        if let Some(generic_constraints) = &generator.generic_constraints {
            stream.extend(generic_constraints.where_clause());
        }
        generator.stream.extend(stream);

        let group = (Delimiter::Brace, TokenStream::new());
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

        let mut stream = Vec::<TokenTree>::new();

        // function name; `fn name`
        stream.extend([ident("fn"), ident(&name)]);

        // lifetimes; `<'a: 'b, D: Display>`
        if !lifetime_and_generics.is_empty() {
            stream.push(punct('<'));
            for (idx, (lifetime_and_generic, dependencies)) in
                lifetime_and_generics.into_iter().enumerate()
            {
                if idx != 0 {
                    stream.push(punct(','));
                }
                stream.extend([ident(&lifetime_and_generic)]);
                if !dependencies.is_empty() {
                    for (idx, dependency) in dependencies.into_iter().enumerate() {
                        stream.push(punct(if idx == 0 { ':' } else { '+' }));
                        stream.extend(TokenStream::from_str(&dependency).unwrap());
                    }
                }
            }
            stream.push(punct('>'));
        }

        // Arguments; `(&self, foo: &Bar)`
        stream.push(TokenTree::Group(Group::new(Delimiter::Parenthesis, {
            let mut arg_stream = Vec::<TokenTree>::new();
            if let Some(self_arg) = self_arg.into_token_tree() {
                arg_stream.extend(self_arg);
                arg_stream.push(punct(','));
            }
            for (idx, (arg_name, arg_ty)) in args.into_iter().enumerate() {
                if idx != 0 {
                    arg_stream.push(punct(','));
                }
                arg_stream.extend(TokenStream::from_str(&arg_name).unwrap());
                arg_stream.push(punct(':'));
                arg_stream.extend(TokenStream::from_str(&arg_ty).unwrap());
            }

            let mut result = TokenStream::new();
            result.extend(arg_stream);
            result
        })));

        // Return type: `-> ResultType`
        if let Some(return_type) = return_type {
            stream.push(punct('-'));
            stream.push(punct('>'));
            stream.extend(TokenStream::from_str(&return_type).unwrap());
        }

        self.group.1.extend(stream);

        GenerateFnBody {
            generate: self,
            group: (Delimiter::Brace, TokenStream::new()),
        }
    }
}

impl Drop for ImplFor<'_> {
    fn drop(&mut self) {
        let stream = std::mem::replace(&mut self.group.1, TokenStream::new());
        self.generator
            .stream
            .extend([TokenTree::Group(Group::new(self.group.0, stream))]);
    }
}

pub struct GenerateFnBody<'a, 'b> {
    generate: &'b mut ImplFor<'a>,
    group: (Delimiter, TokenStream),
}

impl GenerateFnBody<'_, '_> {
    pub fn push_str(&mut self, str: impl AsRef<str>) {
        self.group
            .1
            .extend([TokenStream::from_str(str.as_ref()).unwrap()]);
    }

    pub fn push_group(
        &mut self,
        delim: Delimiter,
        group_content: impl IntoIterator<Item = TokenTree>,
    ) {
        let mut stream = TokenStream::new();
        stream.extend(group_content);
        self.group
            .1
            .extend([TokenTree::Group(Group::new(delim, stream))]);
    }
}

impl Drop for GenerateFnBody<'_, '_> {
    fn drop(&mut self) {
        let stream = std::mem::replace(&mut self.group.1, TokenStream::new());
        self.generate
            .group
            .1
            .extend([TokenTree::Group(Group::new(self.group.0, stream))]);
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

    // pub fn with_lifetime(
    //     mut self,
    //     name: impl Into<String>,
    //     dependencies: impl Into<Vec<String>>,
    // ) -> Self {
    //     let name = name.into();
    //     assert!(name.starts_with('\''));
    //     self.lifetime_and_generics.push((name, dependencies.into()));
    //     self
    // }

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
    // TakeSelf,
    RefSelf,
    // MutSelf,
}

impl FnSelfArg {
    fn into_token_tree(self) -> Option<Vec<TokenTree>> {
        match self {
            Self::None => None,
            // Self::TakeSelf => Some(vec![ident("self")]),
            Self::RefSelf => Some(vec![punct('&'), ident("self")]),
            // Self::MutSelf => Some(vec![punct('&'), ident("mut"), ident("self")]),
        }
    }
}
