use crate::attribute::{ContainerAttributes, FieldAttributes};
use virtue::generate::Generator;
use virtue::parse::Fields;
use virtue::prelude::*;

pub(crate) struct DeriveStruct {
    pub fields: Fields,
    pub attributes: ContainerAttributes,
}

impl DeriveStruct {
    pub fn generate_encode(self, generator: &mut Generator) -> Result<()> {
        let crate_name = &self.attributes.crate_name;
        generator
            .impl_for(&format!("{}::Encode", crate_name))
            .modify_generic_constraints(|generics, where_constraints| {
                if let Some((bounds, lit)) =
                    (self.attributes.encode_bounds.as_ref()).or(self.attributes.bounds.as_ref())
                {
                    where_constraints.clear();
                    where_constraints
                        .push_parsed_constraint(bounds)
                        .map_err(|e| e.with_span(lit.span()))?;
                } else {
                    for g in generics.iter_generics() {
                        where_constraints
                            .push_constraint(g, format!("{}::Encode", crate_name))
                            .unwrap();
                    }
                }
                Ok(())
            })?
            .generate_fn("encode")
            .with_generic_deps("__E", [format!("{}::enc::Encoder", crate_name)])
            .with_self_arg(virtue::generate::FnSelfArg::RefSelf)
            .with_arg("encoder", "&mut __E")
            .with_return_type(format!(
                "core::result::Result<(), {}::error::EncodeError>",
                crate_name
            ))
            .body(|fn_body| {
                for field in self.fields.names() {
                    let attributes = field
                        .attributes()
                        .get_attribute::<FieldAttributes>()?
                        .unwrap_or_default();
                    if attributes.with_serde {
                        fn_body.push_parsed(format!(
                            "{0}::Encode::encode(&{0}::serde::Compat(&self.{1}), encoder)?;",
                            crate_name, field
                        ))?;
                    } else {
                        fn_body.push_parsed(format!(
                            "{}::Encode::encode(&self.{}, encoder)?;",
                            crate_name, field
                        ))?;
                    }
                }
                fn_body.push_parsed("Ok(())")?;
                Ok(())
            })?;
        Ok(())
    }

    pub fn generate_encoded_size(self, generator: &mut Generator) -> Result<()> {
        let crate_name = &self.attributes.crate_name;
        generator
            .impl_for(&format!("{}::EncodedSize", crate_name))
            .modify_generic_constraints(|generics, where_constraints| {
                if let Some((bounds, lit)) =
                    (self.attributes.encoded_size_bounds.as_ref()).or(self.attributes.bounds.as_ref())
                {
                    where_constraints.clear();
                    where_constraints
                        .push_parsed_constraint(bounds)
                        .map_err(|e| e.with_span(lit.span()))?;
                } else {
                    for g in generics.iter_generics() {
                        where_constraints
                            .push_constraint(g, format!("{}::EncodedSize", crate_name))
                            .unwrap();
                    }
                }
                Ok(())
            })?
            .generate_fn("encoded_size")
            .with_generic_deps("__C", [format!("{}::config::Config", crate_name)])
            .with_self_arg(virtue::generate::FnSelfArg::RefSelf)
            .with_return_type(format!(
                "core::result::Result<usize, {}::error::EncodeError>",
                crate_name
            ))
            .body(|fn_body| {
                fn_body.push_parsed("let mut __encoded_size = 0;")?;
                for field in self.fields.names() {
                    let attributes = field
                        .attributes()
                        .get_attribute::<FieldAttributes>()?
                        .unwrap_or_default();
                    if attributes.with_serde {
                        fn_body.push_parsed(format!(
                            "__encoded_size += {0}::EncodedSize::encoded_size::<__C>(&{0}::serde::Compat(&self.{1}))?;",
                            crate_name, field
                        ))?;
                    } else {
                        fn_body.push_parsed(format!(
                            "__encoded_size += {}::EncodedSize::encoded_size::<__C>(&self.{})?;",
                            crate_name, field
                        ))?;
                    }
                }
                fn_body.push_parsed("Ok(__encoded_size)")?;
                Ok(())
            })?;
        Ok(())
    }

    pub fn generate_decode(self, generator: &mut Generator) -> Result<()> {
        // Remember to keep this mostly in sync with generate_borrow_decode
        let crate_name = &self.attributes.crate_name;

        generator
            .impl_for(format!("{}::Decode", crate_name))
            .modify_generic_constraints(|generics, where_constraints| {
                if let Some((bounds, lit)) = (self.attributes.decode_bounds.as_ref()).or(self.attributes.bounds.as_ref()) {
                    where_constraints.clear();
                    where_constraints.push_parsed_constraint(bounds).map_err(|e| e.with_span(lit.span()))?;
                } else {
                    for g in generics.iter_generics() {
                        where_constraints.push_constraint(g, format!("{}::Decode", crate_name)).unwrap();
                    }
                }
                Ok(())
            })?
            .generate_fn("decode")
            .with_generic_deps("__D", [format!("{}::de::Decoder", crate_name)])
            .with_arg("decoder", "&mut __D")
            .with_return_type(format!("core::result::Result<Self, {}::error::DecodeError>", crate_name))
            .body(|fn_body| {
                // Ok(Self {
                fn_body.ident_str("Ok");
                fn_body.group(Delimiter::Parenthesis, |ok_group| {
                    ok_group.ident_str("Self");
                    ok_group.group(Delimiter::Brace, |struct_body| {
                        // Fields
                        // {
                        //      a: bincode::Decode::decode(decoder)?,
                        //      b: bincode::Decode::decode(decoder)?,
                        //      ...
                        // }
                        for field in &self.fields.names() {
                            let attributes = field.attributes().get_attribute::<FieldAttributes>()?.unwrap_or_default();
                            if attributes.with_serde {
                                struct_body
                                    .push_parsed(format!(
                                        "{1}: (<{0}::serde::Compat<_> as {0}::Decode>::decode(decoder)?).0,",
                                        crate_name,
                                        field
                                    ))?;
                            } else {
                                struct_body
                                    .push_parsed(format!(
                                        "{1}: {0}::Decode::decode(decoder)?,",
                                        crate_name,
                                        field
                                    ))?;
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
                Ok(())
            })?;
        self.generate_borrow_decode(generator)?;
        Ok(())
    }

    pub fn generate_borrow_decode(self, generator: &mut Generator) -> Result<()> {
        // Remember to keep this mostly in sync with generate_decode
        let crate_name = self.attributes.crate_name;

        generator
            .impl_for_with_lifetimes(format!("{}::BorrowDecode", crate_name), ["__de"])
            .modify_generic_constraints(|generics, where_constraints| {
                if let Some((bounds, lit)) = (self.attributes.borrow_decode_bounds.as_ref()).or(self.attributes.bounds.as_ref()) {
                    where_constraints.clear();
                    where_constraints.push_parsed_constraint(bounds).map_err(|e| e.with_span(lit.span()))?;
                } else {
                    for g in generics.iter_generics() {
                        where_constraints.push_constraint(g, format!("{}::de::BorrowDecode<'__de>", crate_name)).unwrap();
                    }
                }
                Ok(())
            })?
            .generate_fn("borrow_decode")
            .with_generic_deps("__D", [format!("{}::de::BorrowDecoder<'__de>", crate_name)])
            .with_arg("decoder", "&mut __D")
            .with_return_type(format!("core::result::Result<Self, {}::error::DecodeError>", crate_name))
            .body(|fn_body| {
                // Ok(Self {
                fn_body.ident_str("Ok");
                fn_body.group(Delimiter::Parenthesis, |ok_group| {
                    ok_group.ident_str("Self");
                    ok_group.group(Delimiter::Brace, |struct_body| {
                        for field in self.fields.names() {
                            let attributes = field.attributes().get_attribute::<FieldAttributes>()?.unwrap_or_default();
                            if attributes.with_serde {
                                struct_body
                                    .push_parsed(format!(
                                        "{1}: (<{0}::serde::BorrowCompat<_> as {0}::BorrowDecode>::borrow_decode(decoder)?).0,",
                                        crate_name,
                                        field
                                    ))?;
                            } else {
                                struct_body
                                    .push_parsed(format!(
                                        "{1}: {0}::BorrowDecode::borrow_decode(decoder)?,",
                                        crate_name,
                                        field
                                    ))?;
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
                Ok(())
            })?;
        Ok(())
    }
}
