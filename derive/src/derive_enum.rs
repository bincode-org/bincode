use crate::generate::{FnSelfArg, Generator};
use crate::parse::{EnumVariant, Fields};
use crate::prelude::*;
use crate::Result;

const TUPLE_FIELD_PREFIX: &str = "field_";

pub struct DeriveEnum {
    pub variants: Vec<EnumVariant>,
}

impl DeriveEnum {
    pub fn generate_encodable(self, generator: &mut Generator) -> Result<()> {
        let DeriveEnum { variants } = self;

        generator
            .impl_for("bincode::enc::Encodeable")
            .generate_fn("encode")
            .with_generic("E", ["bincode::enc::Encode"])
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
                        //      bincode::enc::Encodeable::encode(a, &mut encoder)?;
                        //      bincode::enc::Encodeable::encode(b, &mut encoder)?;
                        //      bincode::enc::Encodeable::encode(c, &mut encoder)?;
                        // }
                        match_body.group(Delimiter::Brace, |body| {
                            // variant index
                            body.push_parsed(format!("encoder.encode_u32({})?;", variant_index));
                            // If we have any fields, encode them all one by one
                            for field_name in variant.fields.names() {
                                body.push_parsed(format!(
                                    "bincode::enc::Encodeable::encode({}, &mut encoder)?;",
                                    field_name.to_string_with_prefix(TUPLE_FIELD_PREFIX),
                                ));
                            }
                        });
                        match_body.punct(',');
                    }
                });
                fn_body.push_parsed("Ok(())");
            });
        Ok(())
    }

    pub fn generate_decodable(self, generator: &mut Generator) -> Result<()> {
        let DeriveEnum { variants } = self;

        if generator.has_lifetimes() {
            // enum has a lifetime, implement BorrowDecodable

            generator.impl_for_with_de_lifetime("bincode::de::BorrowDecodable<'__de>")
                .generate_fn("borrow_decode")
                .with_generic("D", ["bincode::de::BorrowDecode<'__de>"])
                .with_arg("mut decoder", "D")
                .with_return_type("Result<Self, bincode::error::DecodeError>")
                .body(|fn_builder| {
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
                                    variant_body.push_parsed("bincode::de::BorrowDecodable::borrow_decode(&mut decoder)?,");
                                }
                            });
                        });
                        variant_case.punct(',');
                    }

                    // invalid idx
                    variant_case.push_parsed(format!(
                        "variant => return Err(bincode::error::DecodeError::UnexpectedVariant {{ min: 0, max: {}, found: variant }})",
                        variants.len() - 1
                    ));
                });
            });
        } else {
            // enum has no lifetimes, implement Decodable

            generator.impl_for("bincode::de::Decodable")
                .generate_fn("decode")
                .with_generic("D", ["bincode::de::Decode"])
                .with_arg("mut decoder", "D")
                .with_return_type("Result<Self, bincode::error::DecodeError>")
                .body(|fn_builder| {

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
                                variant_body.push_parsed("bincode::de::Decodable::decode(&mut decoder)?,");
                            }
                        });
                    });
                    variant_case.punct(',');
                }

                // invalid idx
                variant_case.push_parsed(format!(
                    "variant => return Err(bincode::error::DecodeError::UnexpectedVariant {{ min: 0, max: {}, found: variant }})",
                    variants.len() - 1
                ));
            });
        });
        }

        Ok(())
    }
}
