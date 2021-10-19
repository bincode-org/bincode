use crate::generate::{FnSelfArg, Generator};
use crate::parse::{EnumVariant, Fields};
use crate::prelude::*;
use crate::Result;

const TUPLE_FIELD_PREFIX: &str = "field_";

pub struct DeriveEnum {
    pub variants: Vec<EnumVariant>,
}

impl DeriveEnum {
    pub fn generate_encode(self, generator: &mut Generator) -> Result<()> {
        let DeriveEnum { variants } = self;

        generator
            .impl_for("bincode::enc::Encode")
            .unwrap()
            .generate_fn("encode")
            .with_generic("E", ["bincode::enc::Encoder"])
            .with_self_arg(FnSelfArg::RefSelf)
            .with_arg("mut encoder", "E")
            .with_return_type("core::result::Result<(), bincode::error::EncodeError>")
            .body(|fn_body| {
                fn_body.ident_str("match");
                fn_body.ident_str("self");
                fn_body.group(Delimiter::Brace, |match_body| {
                    for (variant_index, variant) in variants.into_iter().enumerate() {
                        // Self::Variant
                        match_body.ident_str("Self");
                        match_body.puncts("::");
                        match_body.ident(variant.name.clone());

                        // if we have any fields, declare them here
                        // Self::Variant { a, b, c }
                        if let Some(delimiter) = variant.fields.delimiter() {
                            match_body.group(delimiter, |field_body| {
                                for (idx, field_name) in
                                    variant.fields.names().into_iter().enumerate()
                                {
                                    if idx != 0 {
                                        field_body.punct(',');
                                    }
                                    field_body.push(
                                        field_name.to_token_tree_with_prefix(TUPLE_FIELD_PREFIX),
                                    );
                                }
                            });
                        }

                        // Arrow
                        // Self::Variant { a, b, c } =>
                        match_body.puncts("=>");

                        // Body of this variant
                        // Note that the fields are available as locals because of the match destructuring above
                        // {
                        //      encoder.encode_u32(n)?;
                        //      bincode::enc::Encode::encode(a, &mut encoder)?;
                        //      bincode::enc::Encode::encode(b, &mut encoder)?;
                        //      bincode::enc::Encode::encode(c, &mut encoder)?;
                        // }
                        match_body.group(Delimiter::Brace, |body| {
                            // variant index
                            body.push_parsed(format!(
                                "<u32 as bincode::enc::Encode>::encode(&{}, &mut encoder)?;",
                                variant_index
                            ))
                            .unwrap();
                            // If we have any fields, encode them all one by one
                            for field_name in variant.fields.names() {
                                body.push_parsed(format!(
                                    "bincode::enc::Encode::encode({}, &mut encoder)?;",
                                    field_name.to_string_with_prefix(TUPLE_FIELD_PREFIX),
                                ))
                                .unwrap();
                            }
                        });
                        match_body.punct(',');
                    }
                });
                fn_body.push_parsed("Ok(())").unwrap();
            })
            .unwrap();
        Ok(())
    }

    pub fn generate_decode(self, generator: &mut Generator) -> Result<()> {
        let DeriveEnum { variants } = self;
        let enum_name = generator.target_name().to_string();

        if generator.has_lifetimes() {
            // enum has a lifetime, implement BorrowDecode

            generator.impl_for_with_de_lifetime("bincode::de::BorrowDecode<'__de>")
                .unwrap()
                .generate_fn("borrow_decode")
                .with_generic("D", ["bincode::de::BorrowDecoder<'__de>"])
                .with_arg("mut decoder", "D")
                .with_return_type("Result<Self, bincode::error::DecodeError>")
                .body(|fn_builder| {
                    fn_builder
                        .push_parsed("let variant_index = <u32 as bincode::de::Decode>::decode(&mut decoder)?;").unwrap();
                    fn_builder.push_parsed("match variant_index").unwrap();
                    fn_builder.group(Delimiter::Brace, |variant_case| {
                    for (idx, variant) in variants.iter().enumerate() {
                        // idx => Ok(..)
                        variant_case.lit_u32(idx as u32);
                        variant_case.puncts("=>");
                        variant_case.ident_str("Ok");
                        variant_case.group(Delimiter::Parenthesis, |variant_case_body| {
                            // Self::Variant { }
                            // Self::Variant { 0: ..., 1: ... 2: ... },
                            // Self::Variant { a: ..., b: ... c: ... },
                            variant_case_body.ident_str("Self");
                            variant_case_body.puncts("::");
                            variant_case_body.ident(variant.name.clone());

                            variant_case_body.group(Delimiter::Brace, |variant_body| {
                                let is_tuple = matches!(variant.fields, Fields::Tuple(_));
                                for (idx, field) in variant.fields.names().into_iter().enumerate() {
                                    if is_tuple {
                                        variant_body.lit_usize(idx);
                                    } else {
                                        variant_body.ident(field.unwrap_ident().clone());
                                    }
                                    variant_body.punct(':');
                                    variant_body.push_parsed("bincode::de::BorrowDecode::borrow_decode(&mut decoder)?,").unwrap();
                                }
                            });
                        });
                        variant_case.punct(',');
                    }

                    // invalid idx
                    variant_case.push_parsed(format!(
                        "variant => return Err(bincode::error::DecodeError::UnexpectedVariant {{ min: 0, max: {}, found: variant, type_name: {:?} }})",
                        variants.len() - 1,
                        enum_name.to_string()
                    )).unwrap();
                });
            }).unwrap();
        } else {
            // enum has no lifetimes, implement Decode
            generator.impl_for("bincode::de::Decode")
            .unwrap()
                .generate_fn("decode")
                .with_generic("D", ["bincode::de::Decoder"])
                .with_arg("mut decoder", "D")
                .with_return_type("Result<Self, bincode::error::DecodeError>")
                .body(|fn_builder| {
                    fn_builder
                        .push_parsed("let variant_index = <u32 as bincode::de::Decode>::decode(&mut decoder)?;").unwrap();
                    fn_builder.push_parsed("match variant_index").unwrap();
                    fn_builder.group(Delimiter::Brace, |variant_case| {
                    for (idx, variant) in variants.iter().enumerate() {
                        // idx => Ok(..)
                        variant_case.lit_u32(idx as u32);
                        variant_case.puncts("=>");
                        variant_case.ident_str("Ok");
                        variant_case.group(Delimiter::Parenthesis, |variant_case_body| {
                            // Self::Variant { }
                            // Self::Variant { 0: ..., 1: ... 2: ... },
                            // Self::Variant { a: ..., b: ... c: ... },
                            variant_case_body.ident_str("Self");
                            variant_case_body.puncts("::");
                            variant_case_body.ident(variant.name.clone());

                            variant_case_body.group(Delimiter::Brace, |variant_body| {
                                let is_tuple = matches!(variant.fields, Fields::Tuple(_));
                                for (idx, field) in variant.fields.names().into_iter().enumerate() {
                                    if is_tuple {
                                        variant_body.lit_usize(idx);
                                    } else {
                                        variant_body.ident(field.unwrap_ident().clone());
                                    }
                                    variant_body.punct(':');
                                    variant_body.push_parsed("bincode::de::Decode::decode(&mut decoder)?,").unwrap();
                                }
                            });
                        });
                        variant_case.punct(',');
                    }

                    // invalid idx
                    variant_case.push_parsed(format!(
                        "variant => return Err(bincode::error::DecodeError::UnexpectedVariant {{ min: 0, max: {}, found: variant, type_name: {:?} }})",
                        variants.len() - 1,
                        enum_name.to_string()
                    )).unwrap();
                });
            }).unwrap();
        }

        Ok(())
    }
}
