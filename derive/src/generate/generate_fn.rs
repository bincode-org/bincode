use super::{ImplFor, StreamBuilder};
use crate::prelude::Delimiter;

/// A builder for functions.
pub struct FnBuilder<'a, 'b> {
    generate: Option<&'b mut ImplFor<'a>>,
    name: String,

    lifetime_and_generics: Vec<(String, Vec<String>)>,
    self_arg: FnSelfArg,
    args: Vec<(String, String)>,
    return_type: Option<String>,
}

impl<'a, 'b> FnBuilder<'a, 'b> {
    pub(super) fn new(generate: &'b mut ImplFor<'a>, name: impl Into<String>) -> Self {
        Self {
            generate: Some(generate),
            name: name.into(),
            lifetime_and_generics: Vec::new(),
            self_arg: FnSelfArg::None,
            args: Vec::new(),
            return_type: None,
        }
    }

    #[cfg(test)]
    #[doc(hidden)]
    #[allow(unused)]
    pub fn for_test() -> Self {
        Self {
            generate: None,
            name: String::new(),
            lifetime_and_generics: Vec::new(),
            self_arg: FnSelfArg::None,
            args: Vec::new(),
            return_type: None,
        }
    }

    /// Add a generic parameter. Keep in mind that this is *not* a valid lifetime.
    ///
    /// `dependencies` are the optional dependencies of the parameter.
    ///
    /// ```ignore
    /// let mut builder: FnBuilder = ...;
    /// builder
    ///     .with_generic("D", None) // fn Foo<D>()
    ///     .with_generic("E", &["Encodable"]); // fn foo<D, E: Encodable>();
    /// ```
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

    pub fn body(self, body_builder: impl FnOnce(&mut StreamBuilder)) {
        let FnBuilder {
            mut generate,
            name,
            lifetime_and_generics,
            self_arg,
            args,
            return_type,
        } = self;

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

        let generator = generate.take().unwrap();

        generator.group.append(builder);
        generator.group.group(Delimiter::Brace, body_builder);
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
