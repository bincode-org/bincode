use crate::generate::{FnSelfArg, Generator};
use crate::parse::EnumVariant;
use crate::prelude::*;
use crate::Result;

pub struct DeriveEnum {
    pub variants: Vec<EnumVariant>,
}

impl DeriveEnum {
    pub fn generate_encodable(self, generator: &mut Generator) -> Result<()> {
        let DeriveEnum { variants } = self;

        let mut impl_for = generator.impl_for("bincode::enc::Encodeable");
        let mut fn_body = impl_for.generate_fn("encode", |builder| {
            builder
                .with_generic("E", ["bincode::enc::Encode"])
                .with_self_arg(FnSelfArg::RefSelf)
                .with_arg("mut encoder", "E")
                .with_return_type("core::result::Result<(), bincode::error::EncodeError>")
        });
        let fn_body = fn_body.stream();

        fn_body.ident_str("match");
        fn_body.ident_str("self");
        fn_body.group(Delimiter::Brace, |match_body| {
            for (variant_index, variant) in variants.into_iter().enumerate() {
                // Self::Variant
                match_body.ident_str("Self");
                match_body.puncts("::");
                match_body.ident(variant.name.clone());

                // if we have any fields, declare them here
                if let Some(fields) = variant.fields.as_ref() {
                    let delimiter = if variant.is_struct_variant() {
                        Delimiter::Brace
                    } else if variant.is_tuple_variant() {
                        Delimiter::Parenthesis
                    } else {
                        unreachable!()
                    };

                    // field names
                    match_body.group(delimiter, |field_body| {
                        for (idx, field) in fields.iter().enumerate() {
                            if idx != 0 {
                                field_body.punct(',');
                            }
                            field_body.push(field.name_or_idx(idx));
                        }
                    });
                }

                // Arrow
                match_body.puncts("=>");
                match_body.group(Delimiter::Brace, |body| {
                    // variant index
                    body.push_parsed(format!("encoder.encode_u32({})?;", variant_index));
                    if let Some(fields) = variant.fields.as_ref() {
                        // If we have any fields, encode them all one by one
                        for (idx, field) in fields.iter().enumerate() {
                            body.push_parsed(format!(
                                "bincode::enc::Encodeable::encode({}, &mut encoder)?;",
                                field.name_or_idx(idx)
                            ));
                        }
                    }
                });
                match_body.punct(',');
            }
        });
        fn_body.push_parsed("Ok(())");
        Ok(())
    }

    pub fn generate_decodable(self, generator: &mut Generator) -> Result<()> {
        let DeriveEnum { variants } = self;

        if generator.has_lifetimes() {
            // enum has a lifetime, implement BorrowDecodable

            let mut impl_for =
                generator.impl_for_with_de_lifetime("bincode::de::BorrowDecodable<'__de>");
            let mut fn_builder = impl_for.generate_fn("borrow_decode", |builder| {
                builder
                    .with_generic("D", ["bincode::de::BorrowDecode<'__de>"])
                    .with_arg("mut decoder", "D")
                    .with_return_type("Result<Self, bincode::error::DecodeError>")
            });
            let fn_builder = fn_builder.stream();

            fn_builder
                .push_parsed("let variant_index = bincode::de::Decode::decode_u32(&mut decoder)?;");
            fn_builder.push_parsed("match variant_index");
            fn_builder.group(Delimiter::Brace, |variant_case| {
                for (idx, variant) in variants.iter().enumerate() {
                    // idx => Ok(..)
                    variant_case.lit_u32(idx as u32);
                    variant_case.puncts("=>");
                    variant_case.ident_str("Ok");
                    variant_case.group(Delimiter::Parenthesis, |variant_case_body| {
                        // Self::Variant
                        // Self::Variant { 0: ..., 1: ... 2: ... },
                        // Self::Variant { a: ..., b: ... c: ... },
                        variant_case_body.ident_str("Self");
                        variant_case_body.puncts("::");
                        variant_case_body.ident(variant.name.clone());

                        if let Some(fields) = variant.fields.as_ref() {
                            variant_case_body.group(Delimiter::Brace, |variant_body| {
                                for (idx, field) in fields.iter().enumerate() {
                                    if let Some(ident) = field.ident.clone() {
                                        variant_body.ident(ident);
                                    } else {
                                        variant_body.lit_usize(idx);
                                    }
                                    variant_body.punct(':');
                                    variant_body.push_parsed("bincode::de::BorrowDecodable::borrow_decode(&mut decoder)?,");
                                }
                            })
                        }
                    });
                    variant_case.punct(',');
                }

                // invalid idx
                variant_case.push_parsed(format!(
                    "variant => return Err(bincode::error::DecodeError::UnexpectedVariant {{ min: 0, max: {}, found: variant }})",
                    variants.len() - 1
                ));
            });
        } else {
            // enum has no lifetimes, implement Decodable

            let mut impl_for = generator.impl_for("bincode::de::Decodable");
            let mut fn_builder = impl_for.generate_fn("decode", |builder| {
                builder
                    .with_generic("D", ["bincode::de::Decode"])
                    .with_arg("mut decoder", "D")
                    .with_return_type("Result<Self, bincode::error::DecodeError>")
            });

            let fn_builder = fn_builder.stream();

            fn_builder
                .push_parsed("let variant_index = bincode::de::Decode::decode_u32(&mut decoder)?;");
            fn_builder.push_parsed("match variant_index");
            fn_builder.group(Delimiter::Brace, |variant_case| {
                for (idx, variant) in variants.iter().enumerate() {
                    // idx => Ok(..)
                    variant_case.lit_u32(idx as u32);
                    variant_case.puncts("=>");
                    variant_case.ident_str("Ok");
                    variant_case.group(Delimiter::Parenthesis, |variant_case_body| {
                            // Self::Variant
                            // Self::Variant { 0: ..., 1: ... 2: ... },
                            // Self::Variant { a: ..., b: ... c: ... },
                            variant_case_body.ident_str("Self");
                            variant_case_body.puncts("::");
                            variant_case_body.ident(variant.name.clone());

                            if let Some(fields) = variant.fields.as_ref() {
                                variant_case_body.group(Delimiter::Brace, |variant_body| {
                                    for (idx, field) in fields.iter().enumerate() {

                                        if let Some(ident) = field.ident.clone() {
                                            variant_body.ident(ident)
                                        } else {
                                            variant_body.lit_usize(idx)
                                        }

                                        variant_body.punct(':');
                                        variant_body.push_parsed("bincode::de::Decodable::decode(&mut decoder)?,");
                                    }
                                });
                            }
                        });
                        variant_case.  punct(',');
                }

                // invalid idx
                variant_case.push_parsed(format!(
                    "variant => return Err(bincode::error::DecodeError::UnexpectedVariant {{ min: 0, max: {}, found: variant }})",
                    variants.len() - 1
                ));
            });
        }

        Ok(())
    }
}
