use super::FieldAttribute;
use virtue::generate::Generator;
use virtue::parse::Fields;
use virtue::prelude::*;

pub struct DeriveStruct {
    pub fields: Fields,
}

impl DeriveStruct {
    pub fn generate_encode(self, generator: &mut Generator) -> Result<()> {
        let DeriveStruct { fields } = self;

        generator
            .impl_for("bincode::enc::Encode")?
            .modify_generic_constraints(|generics, where_constraints| {
                for g in generics.iter_generics() {
                    where_constraints
                        .push_constraint(g, "bincode::enc::Encode")
                        .unwrap();
                }
            })
            .generate_fn("encode")
            .with_generic("E", ["bincode::enc::Encoder"])
            .with_self_arg(virtue::generate::FnSelfArg::RefSelf)
            .with_arg("encoder", "&mut E")
            .with_return_type("core::result::Result<(), bincode::error::EncodeError>")
            .body(|fn_body| {
                for field in fields.names() {
                    if field
                        .attributes()
                        .has_attribute(FieldAttribute::WithSerde)?
                    {
                        fn_body.push_parsed(format!(
                            "bincode::Encode::encode(&bincode::serde::Compat(&self.{}), encoder)?;",
                            field
                        ))?;
                    } else {
                        fn_body.push_parsed(format!(
                            "bincode::enc::Encode::encode(&self.{}, encoder)?;",
                            field
                        ))?;
                    }
                }
                fn_body.push_parsed("Ok(())")?;
                Ok(())
            })?;
        Ok(())
    }

    pub fn generate_decode(self, generator: &mut Generator) -> Result<()> {
        // Remember to keep this mostly in sync with generate_borrow_decode
        let DeriveStruct { fields } = self;

        generator
            .impl_for("bincode::Decode")?
            .modify_generic_constraints(|generics, where_constraints| {
                for g in generics.iter_generics() {
                    where_constraints.push_constraint(g, "bincode::de::Decode").unwrap();
                }
            })
            .generate_fn("decode")
            .with_generic("D", ["bincode::de::Decoder"])
            .with_arg("decoder", "&mut D")
            .with_return_type("core::result::Result<Self, bincode::error::DecodeError>")
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
                        for field in fields.names() {
                            if field.attributes().has_attribute(FieldAttribute::WithSerde)? {
                                struct_body
                                    .push_parsed(format!(
                                        "{}: (<bincode::serde::Compat<_> as bincode::Decode>::decode(decoder)?).0,",
                                        field
                                    ))?;
                            } else {
                                struct_body
                                    .push_parsed(format!(
                                        "{}: bincode::Decode::decode(decoder)?,",
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

    pub fn generate_borrow_decode(self, generator: &mut Generator) -> Result<()> {
        // Remember to keep this mostly in sync with generate_decode
        let DeriveStruct { fields } = self;

        generator
            .impl_for_with_lifetimes("bincode::de::BorrowDecode", &["__de"])?
            .modify_generic_constraints(|generics, where_constraints| {
                for g in generics.iter_generics() {
                    where_constraints.push_constraint(g, "bincode::de::BorrowDecode").unwrap();
                }
            })
            .generate_fn("borrow_decode")
            .with_generic("D", ["bincode::de::BorrowDecoder<'__de>"])
            .with_arg("decoder", "&mut D")
            .with_return_type("core::result::Result<Self, bincode::error::DecodeError>")
            .body(|fn_body| {
                // Ok(Self {
                fn_body.ident_str("Ok");
                fn_body.group(Delimiter::Parenthesis, |ok_group| {
                    ok_group.ident_str("Self");
                    ok_group.group(Delimiter::Brace, |struct_body| {
                        for field in fields.names() {
                            if field.attributes().has_attribute(FieldAttribute::WithSerde)? {
                                struct_body
                                    .push_parsed(format!(
                                        "{}: (<bincode::serde::BorrowCompat<_> as bincode::de::BorrowDecode>::borrow_decode(decoder)?).0,",
                                        field
                                    ))?;
                            } else {
                                struct_body
                                    .push_parsed(format!(
                                        "{}: bincode::de::BorrowDecode::borrow_decode(decoder)?,",
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
