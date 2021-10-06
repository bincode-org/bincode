use crate::parse::{GenericConstraints, Generics};
use crate::prelude::{Delimiter, Group, Ident, TokenStream, TokenTree};
use crate::utils::*;
use std::str::FromStr;

#[must_use]
pub struct Generate {
    stream: TokenStream,
    group: (Delimiter, TokenStream),
}

impl Generate {
    pub fn impl_for(
        trait_name: &str,
        name: &Ident,
        generics: &Option<Generics>,
        generic_constraints: &Option<GenericConstraints>,
    ) -> Self {
        let mut stream = TokenStream::new();
        stream.extend([ident("impl")]);

        if let Some(generics) = &generics {
            stream.extend([generics.impl_generics()]);
        }
        stream.extend([
            TokenStream::from_str(trait_name).unwrap(),
            ident("for").into(),
            TokenTree::Ident(name.clone()).into(),
        ]);
        if let Some(generics) = &generics {
            stream.extend([generics.type_generics()]);
        }
        if let Some(generic_constraints) = &generic_constraints {
            stream.extend([generic_constraints.where_clause()]);
        }
        let group = (Delimiter::Brace, TokenStream::new());
        Self { stream, group }
    }

    pub fn impl_for_with_de_lifetime(
        trait_name: &str,
        name: &Ident,
        generics: &Option<Generics>,
        generic_constraints: &Option<GenericConstraints>,
    ) -> Self {
        let mut stream = TokenStream::new();
        stream.extend([ident("impl")]);

        if let Some(generics) = &generics {
            stream.extend([generics.impl_generics_with_additional_lifetime("__de")]);
        } else {
            stream.extend([punct('<'), punct('\''), ident("__de"), punct('>')]);
        }

        stream.extend([
            TokenStream::from_str(trait_name).unwrap(),
            ident("for").into(),
            TokenTree::Ident(name.clone()).into(),
        ]);
        if let Some(generics) = &generics {
            stream.extend([generics.type_generics()]);
        }
        if let Some(generic_constraints) = &generic_constraints {
            stream.extend([generic_constraints.where_clause()]);
        }
        let group = (Delimiter::Brace, TokenStream::new());
        Self { stream, group }
    }

    pub fn generate_fn(
        &mut self,
        name: &str,
        builder: impl FnOnce(FnBuilder) -> FnBuilder,
    ) -> GenerateFnBody {
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

    pub fn build(mut self) -> TokenStream {
        self.stream
            .extend([TokenTree::Group(Group::new(self.group.0, self.group.1))]);
        self.stream
    }
}

pub struct GenerateFnBody<'a> {
    generate: &'a mut Generate,
    group: (Delimiter, TokenStream),
}

impl GenerateFnBody<'_> {
    pub fn push(&mut self, str: impl AsRef<str>) {
        self.group
            .1
            .extend([TokenStream::from_str(str.as_ref()).unwrap()]);
    }

    pub fn finish(self) {
        // make sure this is dropped so we release the lifetime on &'a mut Generate
    }
}

impl<'a> Drop for GenerateFnBody<'a> {
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
